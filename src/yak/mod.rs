use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

mod products;
mod serialization;

pub use products::Products;
use products::{yak_can_produce_wool, yak_milk_production};
use serialization::yak_float_years_to_days;

/// The number of days in a yak year.
const DAYS_IN_YAK_YEAR: f64 = 100.0;
/// A yak lives for 10 years, there are 100 days in a yak year.
const MAX_YAK_AGE: u32 = 1_000;

#[derive(Deserialize, Debug, Clone)]
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
    pub fn step_days(&mut self, days: u32) -> Option<Products> {
        if !self.is_alive() {
            return None;
        }

        let mut products = Products::default();

        for _ in 0..days {
            products.add_milk(yak_milk_production(self.age));
            if yak_can_produce_wool(self.age, self.age_last_shaved) {
                self.age_last_shaved = self.age;
                products.add_wool(1);
            }

            self.age += 1;
        }

        Some(products)
    }

    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.age < MAX_YAK_AGE
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn year_age(&self) -> f64 {
        f64::from(self.age) / DAYS_IN_YAK_YEAR
    }

    #[must_use]
    pub fn year_age_last_shaved(&self) -> f64 {
        f64::from(self.age_last_shaved) / DAYS_IN_YAK_YEAR
    }
}

#[cfg(test)]
mod tests {
    use super::products::MIN_YAK_SHAVE_AGE;
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
        assert_ulps_eq!(products.milk(), 50.0);
        assert_eq!(products.wool(), 0);
    }

    #[test]
    fn test_step_days_milk_and_wool() {
        let mut yak = default_yak();
        yak.age = MIN_YAK_SHAVE_AGE;
        let products = yak.step_days(1).unwrap();

        assert_eq!(yak.age, MIN_YAK_SHAVE_AGE + 1);
        assert_ulps_eq!(products.milk(), 47.0);
        assert_eq!(products.wool(), 1);
    }

    #[test]
    fn test_two_days_only_milk() {
        let mut yak = default_yak();
        let products = yak.step_days(2).unwrap();

        assert_eq!(yak.age, 2);
        assert_ulps_eq!(products.milk(), 50.0 + 49.97);
        assert_eq!(products.wool(), 0);
    }

    #[test]
    fn test_two_days_milk_and_wool() {
        let mut yak = default_yak();
        yak.age = MIN_YAK_SHAVE_AGE;
        let products = yak.step_days(2).unwrap();

        assert_eq!(yak.age, MIN_YAK_SHAVE_AGE + 2);
        assert_ulps_eq!(products.milk(), 47.0 + 46.97);
        assert_eq!(products.wool(), 1);
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
        assert_eq!(products.wool(), 2);
    }
}
