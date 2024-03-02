use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

const DAYS_IN_YAK_YEAR: f64 = 100.0;
const MAX_YAK_AGE: f64 = 10.0;

#[derive(Deserialize, Debug)]
#[allow(never_read)]
pub struct Yak {
    name: String,
    /// The age of the yak in yak years
    age: f64,
    /// The age of the yak when it was last shaved
    #[serde(skip_deserializing)]
    age_last_shaved: f64,
}

impl Display for Yak {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} years old", self.name, self.age)?;

        if self.age >= MAX_YAK_AGE {
            write!(f, " (dead)")?;
        }

        Ok(())
    }
}

impl Yak {
    pub fn step_days(&mut self, days: u32) {
        if self.age >= MAX_YAK_AGE {
            return;
        }

        self.age += f64::from(days) / DAYS_IN_YAK_YEAR;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_yak() -> Yak {
        Yak {
            name: "Yak".to_string(),
            age: 0.0,
            age_last_shaved: 0.0,
        }
    }

    #[test]
    fn test_default_yak() {
        let error_margin = f64::EPSILON;
        let yak = default_yak();
        assert_eq!(yak.name, "Yak");
        assert!(yak.age - 0.0f64 < error_margin);
        assert!(yak.age_last_shaved - 0.0f64 < error_margin);
    }

    #[test]
    fn test_display_alive() {
        let yak = default_yak();
        assert_eq!(format!("{yak}"), "Yak 0 years old");
    }

    #[test]
    fn test_display_dead() {
        let mut yak = default_yak();
        yak.age = 10.1;
        assert_eq!(format!("{yak}"), "Yak 10.1 years old (dead)");
    }
}
