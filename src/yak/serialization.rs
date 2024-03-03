use serde::{Deserialize, Deserializer};

use super::DAYS_IN_YAK_YEAR;

pub fn yak_float_years_to_days<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let mut years = f64::deserialize(deserializer)?;

    years *= DAYS_IN_YAK_YEAR;

    match years {
        years if years.is_nan() => Err(serde::de::Error::custom("Yak age is NaN")),
        years if years.is_infinite() => Err(serde::de::Error::custom("Yak age is infinite")),
        years if years.is_sign_negative() => {
            Err(serde::de::Error::custom("Yak age cannot be negative"))
        }
        years if (years > f64::from(u32::MAX)) => {
            Err(serde::de::Error::custom("Yak age is too large"))
        }
        _ => {
            // SAFETY: The following  allows have been checked above
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_sign_loss)]
            Ok(years as u32)
        }
    }
}
