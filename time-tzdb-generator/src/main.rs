#![deny(
    anonymous_parameters,
    clippy::all,
    clippy::explicit_auto_deref,
    clippy::obfuscated_if_else,
    clippy::undocumented_unsafe_blocks,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_extern_crates,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::nursery,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented,
    clippy::uninlined_format_args,
    clippy::unnested_or_patterns,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    deprecated_in_future,
    missing_debug_implementations,
    unused_qualifications,
    variant_size_differences
)]
#![allow(clippy::redundant_pub_crate, clippy::option_if_let_else)]

mod modules;
mod to_code;

use std::collections::{btree_map, BTreeMap, BTreeSet};
use std::convert::Infallible;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use once_cell::sync::Lazy;
use pico_args::Arguments;
use regex::Regex;
use tempfile::{tempdir, TempDir};
use tzif::data::tzif::TzifData;
use tzif::parse_tzif_file;
use walkdir::WalkDir;

use self::modules::Item;
use self::to_code::ToCode;

#[derive(Debug)]
struct TimeZone {
    name: String,
    data: TimeZoneData,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
enum TimeZoneData {
    Canonical(TzifData),
    Link(String),
}

impl PartialEq for TimeZone {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for TimeZone {}

impl PartialOrd for TimeZone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeZone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl TimeZone {
    fn data_name(&self) -> &str {
        match &self.data {
            TimeZoneData::Canonical(_) => &self.name,
            TimeZoneData::Link(canonical_name) => canonical_name,
        }
    }
}

/// Convert a time zone name to a valid Rust identifier.
fn name_to_ident(name: &str, case: Case) -> String {
    static REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"-([0-9]+)").expect("regex should be valid"));
    REGEX
        .replace(name, "_minus_$1")
        .replace(['/', '-'], "_")
        .replace('+', "_plus_")
        .to_case(case)
}

/// Build the tzif files from scratch.
fn generate_tzif() -> Result<TempDir> {
    let tzdb_dir = tempdir()?;
    let tzif_dir = tempdir()?;

    Command::new("curl")
        .args([
            "https://ftp.iana.org/tz/tzdb-latest.tar.lz",
            "-o",
            "tzdb-latest.tar.lz",
        ])
        .current_dir(&tzdb_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;
    Command::new("tar")
        .args([
            "--lzip",
            "-xvf",
            "tzdb-latest.tar.lz",
            "--strip-components=1",
        ])
        .current_dir(&tzdb_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;
    Command::new("make")
        .current_dir(&tzdb_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;
    Command::new("./zic")
        .arg("-d")
        .arg(tzif_dir.path())
        .arg("tzdata.zi")
        .current_dir(&tzdb_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;

    tzdb_dir.close()?;

    Ok(tzif_dir)
}

/// Gather the time zones from the tzif files. Requires the tzif files to be present in the provided
/// directory.
fn gather_time_zones(tzif_dir: TempDir) -> Result<BTreeSet<TimeZone>> {
    // It's not necessary to determine the canonical name for each hard link, as the final output
    // will have identical semantics. For this reason we can assume that the first time an inode is
    // seen, it is the canonical name.
    //
    // This isn't strictly necessary, but reduces the size of the generated code.
    let mut inode_to_tz_name = BTreeMap::new();

    let mut time_zones = BTreeSet::new();

    for entry in WalkDir::new(&tzif_dir) {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let tz_name = path
            .strip_prefix(&tzif_dir)?
            .to_str()
            .context("zone name should be valid utf-8")?
            .to_owned();

        // Detect hard links, indicating a time zone is a link.
        match inode_to_tz_name.entry(path.metadata()?.ino()) {
            // inode not seen before, so this is the canonical name.
            btree_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(tz_name.clone());

                // Get the data from the file.
                let data = parse_tzif_file(path)?;
                assert!(data.version_number() >= 2);
                time_zones.insert(TimeZone {
                    name: tz_name,
                    data: TimeZoneData::Canonical(data),
                });
            }
            // inode has been seen before, so this is a link.
            btree_map::Entry::Occupied(occupied_entry) => {
                let canonical_name = occupied_entry.get().clone();
                time_zones.insert(TimeZone {
                    name: tz_name,
                    data: TimeZoneData::Link(canonical_name),
                });
            }
        }
    }

    tzif_dir.close()?;

    Ok(time_zones)
}

/// Write the time zones.
fn write_zones<'a>(
    output: &mut impl Write,
    time_zones: impl Iterator<Item = &'a TimeZone>,
) -> Result<()> {
    for tz in time_zones {
        write!(output, "{}", tz.to_code()?)?;
    }
    Ok(())
}

/// Write the function to parse a time zone from a string.
fn write_zone_data_from_name<'a>(
    output: &mut impl Write,
    time_zones: impl Iterator<Item = &'a TimeZone>,
) -> Result<()> {
    writeln!(
        output,
        "/// Get a time zone's data from its name. Returns `None` if the name is not recognized."
    )?;
    write!(
        output,
        "fn zone_data_from_name(s: &str) -> Option<TzifData> {{"
    )?;

    let mut is_first = true;

    for tz in time_zones {
        if is_first {
            is_first = false;
        } else {
            write!(output, "else ")?;
        }

        write!(
            output,
            "if s.eq_ignore_ascii_case(\"{}\") {{ Some({}) }}",
            &tz.name,
            name_to_ident(tz.data_name(), Case::UpperSnake)
        )?;
    }
    writeln!(output, "else {{ None }}}}")?;

    Ok(())
}

/// Write the module hierarchy and structs.
fn write_modules_and_structs<'a>(
    output: &mut impl Write,
    time_zones: impl Iterator<Item = &'a TimeZone>,
) -> Result<()> {
    let root_item = modules::generate_items(time_zones);
    writeln!(output, "{}", root_item.to_code()?)?;
    Ok(())
}

/// Create a rustfmt process that outputs to the provided path. Code can be written to the returned
/// processes's stdin.
fn spawn_rustfmt(path: &Path) -> Result<process::Child> {
    let output_file = File::create(path)?;

    let rustfmt_process = Command::new("rustfmt")
        .args([
            "+nightly",
            "--config",
            "hard_tabs=true,use_small_heuristics=max,imports_granularity=Preserve,\
             overflow_delimited_expr=true,group_imports=One,\
             short_array_element_width_threshold=10000",
        ])
        .stdin(Stdio::piped())
        .stdout(output_file)
        .stderr(Stdio::null())
        .spawn()?;

    Ok(rustfmt_process)
}

fn main() -> Result<()> {
    let mut args = Arguments::from_env();
    let output_path = args.value_from_os_str(["-o", "--output"], |s| {
        Ok::<_, Infallible>(PathBuf::from(s))
    })?;

    let tzif_dir = generate_tzif()?;
    let time_zones = gather_time_zones(tzif_dir)?;

    let mut rustfmt_process = spawn_rustfmt(&output_path)?;
    let mut rustfmt_stdin = rustfmt_process
        .stdin
        .take()
        .context("could not obtain rustfmt stdin")?;

    write!(
        rustfmt_stdin,
        "#![allow(warnings, clippy::if_same_then_else)]"
    )?;
    // Imports to significantly reduce the size of the generated code.
    write!(
        rustfmt_stdin,
        "use crate::{{*, TransitionDay::*, UtcLocalIndicator::*, StandardWallIndicator::*, \
         sealed::Sealed}};",
    )?;

    write_zones(&mut rustfmt_stdin, time_zones.iter())?;
    write_zone_data_from_name(&mut rustfmt_stdin, time_zones.iter())?;
    write_modules_and_structs(&mut rustfmt_stdin, time_zones.iter())?;

    drop(rustfmt_stdin);
    rustfmt_process.wait()?;

    Ok(())
}
