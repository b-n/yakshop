use serde::de::Deserializer;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

const DAYS_IN_YAK_YEAR: f64 = 100.0;

/// A yak lives for 10 years, there are 100 days in a yak year.
const MAX_YAK_AGE: u32 = 1_000;

fn yak_float_years_to_days<'de, D>(deserializer: D) -> Result<u32, D::Error>
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

#[derive(Deserialize, Debug)]
#[allow(never_read)]
pub struct Yak {
    name: String,
    /// The age of the yak in yak years
    #[serde(deserialize_with = "yak_float_years_to_days")]
    age: u32,
    /// The age of the yak when it was last shaved
    #[serde(skip_deserializing)]
    age_last_shaved: u32,
}

impl Display for Yak {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:.2} years old", self.name, self.float_age())?;

        if self.age >= MAX_YAK_AGE {
            write!(f, " (dead)")?;
        }

        Ok(())
    }
}

impl Yak {
    pub fn step_days(&mut self, days: u32) {
        if !self.is_alive() {
            return;
        }

        self.age += days;
    }

    pub fn is_alive(&self) -> bool {
        self.age < MAX_YAK_AGE
    }

    pub fn float_age(&self) -> f64 {
        f64::from(self.age) / DAYS_IN_YAK_YEAR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_yak() -> Yak {
        Yak {
            name: "Yak".to_string(),
            age: 0,
            age_last_shaved: 0,
        }
    }

    #[test]
    fn test_default_yak() {
        let yak = default_yak();
        assert_eq!(yak.name, "Yak");
        assert_eq!(yak.age, 0);
        assert_eq!(yak.age_last_shaved, 0);
    }

    #[test]
    fn test_display_alive() {
        let yak = default_yak();
        assert_eq!(format!("{yak}"), "Yak 0 years old");
    }

    #[test]
    fn test_display_floating_point_age() {
        let mut yak = default_yak();
        yak.age = 1;
        assert_eq!(format!("{yak}"), "Yak 0.01 years old");
    }

    #[test]
    fn test_display_dead() {
        let mut yak = default_yak();
        yak.age = MAX_YAK_AGE;
        assert_eq!(format!("{yak}"), "Yak 10 years old (dead)");
    }

    #[test]
    fn test_step_until_death() {
        let mut yak = default_yak();

        yak.age = 998;
        assert!(yak.is_alive());

        yak.step_days(1);
        assert_eq!(yak.age, 999);
        assert!(yak.is_alive());

        yak.step_days(1);
        assert_eq!(yak.age, 1000);
        assert!(!yak.is_alive());
    }
}
