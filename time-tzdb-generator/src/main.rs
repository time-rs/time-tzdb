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

use std::collections::{btree_map, BTreeMap};
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

type TimeZones = Vec<(String, TzifData)>;
type TimeZoneLinks = BTreeMap<String, Vec<String>>;

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
fn gather_time_zones(tzif_dir: TempDir) -> Result<(TimeZones, TimeZoneLinks)> {
    // It's not necessary to determine the canonical name for each hard link, as the final output
    // will have identical semantics. For this reason we can assume that the first time an inode is
    // seen, it is the canonical name.
    //
    // This isn't strictly necessary, but reduces the size of the generated code.
    let mut inode_to_tz_name = BTreeMap::new();
    let mut tz_links = BTreeMap::new();

    let mut time_zones = Vec::new();

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
            // inode not seen before, so this is the canonical name. Take note of this and carry on.
            btree_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(tz_name.clone());
            }
            // inode has been seen before. Add it to the list of hard links and move to the next
            // file.
            btree_map::Entry::Occupied(occupied_entry) => {
                let canonical_name = occupied_entry.get().clone();
                tz_links
                    .entry(canonical_name)
                    .or_insert_with(Vec::new)
                    .push(tz_name.clone());
                continue;
            }
        }

        // Get the data from the file.
        let data = parse_tzif_file(path)?;
        assert!(data.version_number() >= 2);
        time_zones.push((tz_name, data));
    }

    tzif_dir.close()?;

    Ok((time_zones, tz_links))
}

/// Write the time zones.
fn write_zones(output: &mut impl Write, time_zones: &TimeZones) -> Result<()> {
    for (tz_name, data) in time_zones {
        let tz_ident = name_to_ident(tz_name, Case::UpperSnake);
        let expr = data.to_code()?;
        writeln!(
            output,
            "/// The `{tz_name}` time zone.\nconst {tz_ident}: TzifData = {expr};"
        )?;
    }

    Ok(())
}

/// Write the time zone links.
fn write_links(output: &mut impl Write, tz_links: &TimeZoneLinks) -> Result<()> {
    for (canonical_name, links) in tz_links {
        let canonical_ident = name_to_ident(canonical_name, Case::UpperSnake);

        for tz_name in links {
            let tz_ident = name_to_ident(tz_name, Case::UpperSnake);
            writeln!(
                output,
                "/// The `{tz_name}` time zone.\nconst {tz_ident}: TzifData = {canonical_ident};",
            )?;
        }
    }

    Ok(())
}

/// Write the function to parse a time zone from a string.
fn write_zone_data_from_name(
    output: &mut impl Write,
    time_zones: &TimeZones,
    tz_links: &TimeZoneLinks,
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

    for (tz_name, _) in time_zones {
        if is_first {
            is_first = false;
        } else {
            write!(output, "else ")?;
        }

        write!(
            output,
            "if s.eq_ignore_ascii_case(\"{}\") {{ Some({}) }}",
            tz_name,
            name_to_ident(tz_name, Case::UpperSnake)
        )?;
    }
    for links in tz_links.values() {
        for tz_name in links {
            write!(
                output,
                "else if s.eq_ignore_ascii_case(\"{}\") {{ Some({}) }}",
                tz_name,
                name_to_ident(tz_name, Case::UpperSnake)
            )?;
        }
    }
    writeln!(output, "else {{ None }}}}")?;

    Ok(())
}

/// Write the module hierarchy and structs.
fn write_modules_and_structs(
    output: &mut impl Write,
    time_zones: &TimeZones,
    tz_links: &TimeZoneLinks,
) -> Result<()> {
    write_module_item(output, modules::generate_items(time_zones, tz_links))?;
    Ok(())
}

/// Write the module hierarchy and structs, excluding the root module.
fn write_module_item(output: &mut impl Write, item: Item) -> Result<()> {
    match item {
        Item::Module { name, children } => {
            write!(output, "pub mod {name} {{")?;
            write!(output, "use super::*;")?;
            for child in children {
                write_module_item(output, child)?;
            }
            write!(output, "}}")?;
        }
        Item::Struct { name, full_name } => {
            write!(
                output,
                "/// The `{full_name}` time zone.\npub struct {};",
                name_to_ident(&name, Case::Pascal)
            )?;
            write!(
                output,
                "impl Sealed for {} {{\nconst NAME: &'static str = \"{}\";\nconst DATA: TzifData \
                 = {};}}",
                name_to_ident(&name, Case::Pascal),
                full_name,
                name_to_ident(&full_name, Case::UpperSnake)
            )?;
            write!(
                output,
                "impl TimeZone for {} {{}}",
                name_to_ident(&name, Case::Pascal)
            )?;
        }
    }

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
    let (time_zones, tz_links) = gather_time_zones(tzif_dir)?;

    let mut rustfmt_process = spawn_rustfmt(&output_path)?;
    let mut rustfmt_stdin = rustfmt_process
        .stdin
        .take()
        .context("could not obtain rustfmt stdin")?;

    writeln!(rustfmt_stdin, "#![allow(warnings)]")?;
    // Imports to significantly reduce the size of the generated code.
    writeln!(
        rustfmt_stdin,
        "use crate::{{*, TransitionDay::*, UtcLocalIndicator::*, StandardWallIndicator::*, \
         sealed::Sealed}};",
    )?;

    write_zones(&mut rustfmt_stdin, &time_zones)?;
    write_links(&mut rustfmt_stdin, &tz_links)?;
    write_zone_data_from_name(&mut rustfmt_stdin, &time_zones, &tz_links)?;
    write_modules_and_structs(&mut rustfmt_stdin, &time_zones, &tz_links)?;

    drop(rustfmt_stdin);
    rustfmt_process.wait()?;

    Ok(())
}
