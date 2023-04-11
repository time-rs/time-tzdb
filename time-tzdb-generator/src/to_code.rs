use anyhow::{Context, Result};
use convert_case::Case;
use tzif::data::posix::{
    DstTransitionInfo, PosixTzString, TransitionDate, TransitionDay, ZoneVariantInfo,
};
use tzif::data::time::Seconds;
use tzif::data::tzif::{
    DataBlock, LeapSecondRecord, LocalTimeTypeRecord, StandardWallIndicator, TzifData,
    UtLocalIndicator,
};

use crate::{name_to_ident, Item};

/// Convert the given value to a string of Rust code. Note that the output may rely on certain
/// imports being present.
pub(crate) trait ToCode {
    fn to_code(&self) -> Result<String>;
}

impl ToCode for String {
    fn to_code(&self) -> Result<String> {
        self.as_str().to_code()
    }
}

impl ToCode for &str {
    fn to_code(&self) -> Result<String> {
        Ok(format!(r#""{}""#, self.escape_default()))
    }
}

impl<T: ToCode> ToCode for Option<T> {
    fn to_code(&self) -> Result<String> {
        Ok(match self {
            Some(value) => format!("Some({})", value.to_code()?),
            None => "None".to_string(),
        })
    }
}

impl<T: ToCode> ToCode for Vec<T> {
    fn to_code(&self) -> Result<String> {
        Ok(format!(
            "&[{}]",
            self.iter()
                .map(|value| value.to_code())
                .collect::<Result<Vec<_>>>()?
                .join(", ")
        ))
    }
}

impl ToCode for usize {
    fn to_code(&self) -> Result<String> {
        Ok(self.to_string())
    }
}

impl ToCode for Seconds {
    fn to_code(&self) -> Result<String> {
        Ok(self.0.to_string())
    }
}

impl ToCode for TzifData {
    fn to_code(&self) -> Result<String> {
        Ok(format!(
            "TzifData {{ data_block: {}, footer: {} }}",
            self.data_block2
                .as_ref()
                .context("data block not present")?
                .to_code()?,
            self.footer
                .as_ref()
                .context("footer not present")?
                .to_code()?
        ))
    }
}

impl ToCode for DataBlock {
    fn to_code(&self) -> Result<String> {
        let mut any_is_empty = false;
        let mut any_is_present = false;

        let mut s = "DataBlock {".to_owned();

        if self.transition_times.is_empty() {
            any_is_empty = true;
        } else {
            any_is_present = true;
            s.push_str(&format!(
                "transition_times: {}",
                self.transition_times.to_code()?
            ));
        }

        if self.transition_types.is_empty() {
            any_is_empty = true;
        } else {
            if any_is_present {
                s.push_str(", ");
            }
            any_is_present = true;
            s.push_str(&format!(
                "transition_types: {}",
                self.transition_types.to_code()?
            ));
        }

        if self.local_time_type_records.is_empty() {
            any_is_empty = true;
        } else {
            if any_is_present {
                s.push_str(", ");
            }
            any_is_present = true;
            s.push_str(&format!(
                "local_time_type_records: {}",
                self.local_time_type_records.to_code()?
            ));
        }

        if self.time_zone_designations.is_empty() {
            any_is_empty = true;
        } else {
            if any_is_present {
                s.push_str(", ");
            }
            any_is_present = true;
            s.push_str(&format!(
                "time_zone_designations: {}",
                self.time_zone_designations.to_code()?
            ));
        }

        if self.leap_second_records.is_empty() {
            any_is_empty = true;
        } else {
            if any_is_present {
                s.push_str(", ");
            }
            any_is_present = true;
            s.push_str(&format!(
                "leap_second_records: {}",
                self.leap_second_records.to_code()?
            ));
        }

        if self.standard_wall_indicators.is_empty() {
            any_is_empty = true;
        } else {
            if any_is_present {
                s.push_str(", ");
            }
            any_is_present = true;
            s.push_str(&format!(
                "standard_wall_indicators: {}",
                self.standard_wall_indicators.to_code()?
            ));
        }

        if self.ut_local_indicators.is_empty() {
            any_is_empty = true;
        } else {
            if any_is_present {
                s.push_str(", ");
            }
            any_is_present = true;
            s.push_str(&format!(
                "utc_local_indicators: {}",
                self.ut_local_indicators.to_code()?
            ));
        }

        if !any_is_present {
            return Ok("DataBlock::EMPTY".to_owned());
        }
        if any_is_empty {
            s.push_str(", ..DataBlock::EMPTY");
        }

        s.push('}');

        Ok(s)
    }
}

impl ToCode for LocalTimeTypeRecord {
    fn to_code(&self) -> Result<String> {
        Ok(format!(
            "LocalTimeTypeRecord {{ utc_offset: {}, is_dst: {}, idx: {} }}",
            self.utoff.0, self.is_dst, self.idx
        ))
    }
}

impl ToCode for LeapSecondRecord {
    fn to_code(&self) -> Result<String> {
        Ok(format!(
            "LeapSecondRecord {{ transition_time: {}, correction: {} }}",
            self.occurrence.0, self.correction
        ))
    }
}

impl ToCode for StandardWallIndicator {
    fn to_code(&self) -> Result<String> {
        Ok(match self {
            Self::Standard => "Standard",
            Self::Wall => "Wall",
        }
        .to_string())
    }
}

impl ToCode for UtLocalIndicator {
    fn to_code(&self) -> Result<String> {
        Ok(match self {
            Self::Ut => "Utc",
            Self::Local => "Local",
        }
        .to_string())
    }
}

impl ToCode for PosixTzString {
    fn to_code(&self) -> Result<String> {
        Ok(format!(
            "PosixTzString {{ std_info: {}, dst_info: {} }}",
            self.std_info.to_code()?,
            self.dst_info.to_code()?
        ))
    }
}

impl ToCode for ZoneVariantInfo {
    fn to_code(&self) -> Result<String> {
        Ok(format!(
            "ZoneVariantInfo {{ name: {}, offset: {} }}",
            self.name.to_code()?,
            self.offset.to_code()?
        ))
    }
}

impl ToCode for DstTransitionInfo {
    fn to_code(&self) -> Result<String> {
        Ok(format!(
            "DstTransitionInfo {{ variant_info: {}, start_date: {}, end_date: {} }}",
            self.variant_info.to_code()?,
            self.start_date.to_code()?,
            self.end_date.to_code()?
        ))
    }
}

impl ToCode for TransitionDate {
    fn to_code(&self) -> Result<String> {
        Ok(format!(
            "TransitionDate {{ day: {}, time: {} }}",
            self.day.to_code()?,
            self.time.to_code()?
        ))
    }
}

impl ToCode for TransitionDay {
    fn to_code(&self) -> Result<String> {
        Ok(match self {
            Self::NoLeap(day) => format!("NoLeap({day})"),
            Self::WithLeap(day) => format!("WithLeap({day})"),
            Self::Mwd(month, week, day) => format!("MonthWeekDay({month}, {week}, {day})"),
        })
    }
}

impl ToCode for Item {
    fn to_code(&self) -> Result<String> {
        let mut s = String::new();
        match self {
            Self::Module { name, children } => {
                s.push_str(&format!("pub mod {name} {{"));
                s.push_str("use super::*;");
                for child in children {
                    s.push_str(&child.to_code()?);
                }
                s.push('}');
            }
            Self::Struct { name, full_name } => {
                s.push_str(&format!(
                    "/// The `{full_name}` time zone.\npub struct {};",
                    name_to_ident(name, Case::Pascal)
                ));
                s.push_str(&format!(
                    "impl Sealed for {} {{\nconst NAME: &'static str = \"{}\";\nconst DATA: \
                     TzifData = {};}}",
                    name_to_ident(name, Case::Pascal),
                    full_name,
                    name_to_ident(full_name, Case::UpperSnake)
                ));
                s.push_str(&format!(
                    "impl TimeZone for {} {{}}",
                    name_to_ident(name, Case::Pascal)
                ));
            }
        }

        Ok(s)
    }
}
