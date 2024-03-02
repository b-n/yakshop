use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

const DAYS_IN_YAK_YEAR: f64 = 100.0;
const MAX_YAK_AGE: f64 = 10.0;

// The default value for the `alive` field
fn default_alive() -> bool {
    true
}

#[derive(Deserialize, Debug)]
#[allow(never_read)]
pub struct Yak {
    name: String,
    /// The age of the yak in yak years
    age: f64,
    /// The age of the yak when it was last shaved
    #[serde(skip_deserializing)]
    age_last_shaved: f32,
    #[serde(skip_deserializing, default = "default_alive")]
    alive: bool,
}

impl Display for Yak {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} years old", self.name, self.age)?;

        if !self.alive {
            write!(f, " (dead)")?;
        }

        Ok(())
    }
}

impl Yak {
    pub fn step_days(&mut self, days: u32) {
        if !self.alive {
            return;
        }

        self.age += f64::from(days) / DAYS_IN_YAK_YEAR;

        if self.age > MAX_YAK_AGE {
            self.alive = false;
        }
    }
}
