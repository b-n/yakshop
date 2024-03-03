use serde::{Deserialize, Deserializer, Serializer};

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

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn yak_age_to_float_years<S>(age: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let years = f64::from(*age) / DAYS_IN_YAK_YEAR;
    serializer.serialize_f64(years)
}
