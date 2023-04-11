//! Data from the time zone database for use in the `time` crate.

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
    clippy::missing_docs_in_private_items,
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
#![doc(html_favicon_url = "https://avatars0.githubusercontent.com/u/55999857")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/55999857")]
#![doc(test(attr(deny(warnings))))]

/// The raw data of the time zone database.
mod raw_info;

pub use raw_info::time_zones;

/// Sealed to prevent downstream implementations.
mod sealed {
    use super::*;

    /// Raw data for a time zone. Sealed to prevent downstream implementations.
    pub trait Sealed {
        /// The name of the time zone.
        const NAME: &'static str;
        /// The raw data.
        const DATA: TzifData;
    }
}

/// A time zone.
pub trait TimeZone: sealed::Sealed {}

/// A struct containing the data of a `TZif` file.
#[derive(Debug)]
pub struct TzifData {
    /// The data block.
    pub data_block: DataBlock,
    /// The footer.
    pub footer: PosixTzString,
}

/// A `TZif` data block consists of seven variable-length elements, each of which is a series of
/// items.
#[derive(Debug)]
pub struct DataBlock {
    /// A series of UNIX leap-time values sorted in strictly ascending order. Each value is used as
    /// a transition time at which the rules for computing local time may change.
    pub transition_times: &'static [i64],
    /// A series of integers specifying the type of local time of the corresponding transition
    /// time. These values serve as zero-based indices into the array of local time type
    /// records.
    pub transition_types: &'static [usize],
    /// A series of [`LocalTimeTypeRecord`] objects.
    pub local_time_type_records: &'static [LocalTimeTypeRecord],
    /// The string representations for a time-zone designation, such as "PST" or "PDT".
    pub time_zone_designations: &'static [&'static str],
    /// A series of [`LeapSecondRecord`] objects.
    pub leap_second_records: &'static [LeapSecondRecord],
    /// A series of [`StandardWallIndicator`] objects.
    pub standard_wall_indicators: &'static [StandardWallIndicator],
    /// A series of [`UtcLocalIndicator`] objects.
    pub utc_local_indicators: &'static [UtcLocalIndicator],
}

impl DataBlock {
    /// A data block containing no data.
    pub const EMPTY: Self = Self {
        transition_times: &[],
        transition_types: &[],
        local_time_type_records: &[],
        time_zone_designations: &[],
        leap_second_records: &[],
        standard_wall_indicators: &[],
        utc_local_indicators: &[],
    };
}

/// A record specifying a local time type.
#[derive(Debug)]
pub struct LocalTimeTypeRecord {
    /// A signed integer specifying the number of seconds to be added to local time in order to
    /// determine UTC. Note that this is the reverse of how UTC offsets are typically thought
    /// about.
    pub utc_offset: i32,
    /// A value indicating whether local time should be considered Daylight Saving Time (DST). The
    /// value MUST be 0 A value of [`true`] indicates that this type of time is DST. A value of
    /// [`false`] indicates that this time type is standard time.
    pub is_dst: bool,
    /// An integer specifying a zero-based index into the series of time zone designation bytes,
    /// thereby selecting a particular designation string.
    pub idx: usize,
}

/// A record specifying the corrections that need to be applied to the UTC in in order to determine
/// TAI.
#[derive(Debug)]
pub struct LeapSecondRecord {
    /// A UNIX leap time value specifying the time at which a leap-second correction occurs.
    pub occurrence: i32,
    /// A signed integer specifying the value of LEAPCORR on or after the occurrence. If no records
    /// are present, the value of LEAPCORR is zero.
    pub correction: i32,
}

/// Indicates whether the transition times associated with local time types were specified as
/// standard time or wall-clock time.
#[derive(Debug)]
pub enum StandardWallIndicator {
    /// Standard time
    Standard,
    /// Wall-clock time
    Wall,
}

/// Indicates whether the transition times associated with local time types were specified as UTC or
/// local time.
#[derive(Debug)]
pub enum UtcLocalIndicator {
    /// UTC time
    Utc,
    /// Local time
    Local,
}

/// A struct for holding data encoded by a POSIX time-zone string.
#[derive(Debug)]
pub struct PosixTzString {
    /// The variant info of the STD time-zone variant.
    pub std_info: ZoneVariantInfo,
    /// The variant info of the DST time-zone variant if present.
    pub dst_info: Option<DstTransitionInfo>,
}

/// A struct to hold a time-zone variant name and its offset. The offset is how many seconds must be
/// added to the local time to reach UTC.
#[derive(Debug)]
pub struct ZoneVariantInfo {
    /// The name of the time-zone variant.
    pub name: &'static str,
    /// The offset time in seconds that must be added to the local time to reach UTC.
    pub offset: i32,
}

/// A struct for holding DST transition info.
#[derive(Debug)]
pub struct DstTransitionInfo {
    /// The zone variant info including name and offset.
    pub variant_info: ZoneVariantInfo,
    /// The DST transition start date.
    pub start_date: TransitionDate,
    /// The DST transition end date.
    pub end_date: TransitionDate,
}

/// A struct to hold a DST transition date.
#[derive(Debug)]
pub struct TransitionDate {
    /// The day on which the transition occurs.
    pub day: TransitionDay,
    /// The time in seconds in which the transition occurs.
    pub time: i32,
}

/// A struct for defining a DST transition day.
#[derive(Debug)]
pub enum TransitionDay {
    /// The day of the year, ignoring Feb. 29 on leap years.
    ///
    /// Ranges from [1, 365]
    NoLeap(u16),
    /// The day of the year, accounting for Feb. 29 on leap years.
    ///
    /// Ranges from [0, 365]
    WithLeap(u16),
    /// The month, week, day value.
    ///
    /// Month ranges from [1, 12].
    ///
    /// Week ranges from [1, 5].
    ///
    /// Day ranges from [0, 6].
    MonthWeekDay(u8, u8, u8),
}
