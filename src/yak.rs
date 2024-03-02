use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Debug)]
#[allow(never_read)]
pub struct Yak {
    name: String,
    age: f32,
}

impl Display for Yak {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} years old", self.name, self.age,)
    }
}
