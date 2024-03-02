use serde::de::Deserializer;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

const DAYS_IN_YAK_YEAR: f64 = 100.0;

/// A yak lives for 10 years, there are 100 days in a yak year.
const MAX_YAK_AGE: u32 = 1_000;

/// A yak can only be shaved after it is 100 days (1 year) old.
const MIN_YAK_SHAVE_AGE: u32 = 100;

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
        write!(f, "{} {} years old", self.name, self.year_age())?;

        if self.age >= MAX_YAK_AGE {
            write!(f, " (dead)")?;
        }

        Ok(())
    }
}

impl Yak {
    pub fn step_days(&mut self, days: u32) -> Option<YakProducts> {
        if !self.is_alive() {
            return None;
        }

        let mut products = YakProducts::default();

        for _ in 0..days {
            products.milk += self.produce_milk();
            if self.can_produce_wool() {
                self.age_last_shaved = self.age;
                products.wool += 1;
            }

            self.age += 1;
        }

        Some(products)
    }

    fn produce_milk(&self) -> f64 {
        50.0 - (f64::from(self.age) * 0.03)
    }

    fn can_produce_wool(&self) -> bool {
        let day_age = f64::from(self.age);
        if self.age < MIN_YAK_SHAVE_AGE {
            return false;
        }

        // The next shave date is 8 + (0.01 * age years after the last shave)
        let next_shave_date: f64 = f64::from(self.age_last_shaved) + 8.0 + (day_age * 0.01);

        day_age >= next_shave_date
    }

    pub fn is_alive(&self) -> bool {
        self.age < MAX_YAK_AGE
    }

    pub fn year_age(&self) -> f64 {
        f64::from(self.age) / DAYS_IN_YAK_YEAR
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct YakProducts {
    pub milk: f64,
    pub wool: u32,
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

    #[test]
    fn test_yak_does_not_age_after_death() {
        let mut yak = default_yak();

        yak.age = 999;
        assert!(yak.is_alive());

        yak.step_days(1);
        assert_eq!(yak.age, 1000);
        assert!(!yak.is_alive());

        yak.step_days(1);
        assert_eq!(yak.age, 1000);
        assert!(!yak.is_alive());
    }

    #[test]
    fn test_step_days_only_milk() {
        let mut yak = default_yak();
        let products = yak.step_days(1).unwrap();

        assert_eq!(yak.age, 1);
        assert_ulps_eq!(products.milk, 50.0);
        assert_eq!(products.wool, 0);
    }

    #[test]
    fn test_step_days_milk_and_wool() {
        let mut yak = default_yak();
        yak.age = MIN_YAK_SHAVE_AGE;
        let products = yak.step_days(1).unwrap();

        assert_eq!(yak.age, MIN_YAK_SHAVE_AGE + 1);
        assert_ulps_eq!(products.milk, 47.0);
        assert_eq!(products.wool, 1);
    }

    #[test]
    fn test_two_days_only_milk() {
        let mut yak = default_yak();
        let products = yak.step_days(2).unwrap();

        assert_eq!(yak.age, 2);
        assert_ulps_eq!(products.milk, 50.0 + 49.97);
        assert_eq!(products.wool, 0);
    }

    #[test]
    fn test_two_days_milk_and_wool() {
        let mut yak = default_yak();
        yak.age = MIN_YAK_SHAVE_AGE;
        let products = yak.step_days(2).unwrap();

        assert_eq!(yak.age, MIN_YAK_SHAVE_AGE + 2);
        assert_ulps_eq!(products.milk, 47.0 + 46.97);
        assert_eq!(products.wool, 1);
    }

    #[test]
    fn test_double_wool() {
        let mut yak = default_yak();
        yak.age = MIN_YAK_SHAVE_AGE;
        // First shave = Yak age 100. (last shave = 0)
        // Second shave = Yak age 110. (last shave = 100, 8 + 109 * 0.01 = 9.09, there 109 is to
        //                soon to shave again)
        // Therefore need 11 days to tick from day 100 to 110 to completion for 2 wools.
        let products = yak.step_days(11).unwrap();

        assert_eq!(yak.age, MIN_YAK_SHAVE_AGE + 11);
        assert_eq!(products.wool, 2);
    }
}
