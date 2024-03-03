use std::ops::{Add, AddAssign};

/// Milk production suffers from floating point errors. Do all math without floating point and
/// adjust with this constant at the end to get the correct result.
const MILK_FLOATING_POINT_ADJUSTMENT: u32 = 100;
/// A yak can only be shaved after it is 100 days (1 year) old.
pub const MIN_YAK_SHAVE_AGE: u32 = 100;

pub fn yak_milk_production(age: u32) -> u32 {
    5_000 - age * 3
}

pub fn yak_can_produce_wool(age: u32, age_last_shaved: u32) -> bool {
    if age < MIN_YAK_SHAVE_AGE {
        return false;
    }

    let float_age = f64::from(age);
    let float_age_last_shaved = f64::from(age_last_shaved);

    // The next shave date is 8 + (0.01 * age years after the last shave)
    let next_shave_date: f64 = float_age_last_shaved + 8.0 + (float_age * 0.01);

    f64::from(age) >= next_shave_date
}

#[derive(Default, Debug)]
pub struct Products {
    milk: u32,
    wool: u32,
}

impl Add<Products> for Products {
    type Output = Products;

    fn add(self, rhs: Products) -> Products {
        Products {
            milk: self.milk + rhs.milk,
            wool: self.wool + rhs.wool,
        }
    }
}

impl AddAssign<Products> for Products {
    fn add_assign(&mut self, rhs: Products) {
        *self = Products {
            milk: self.milk + rhs.milk,
            wool: self.wool + rhs.wool,
        };
    }
}

impl Products {
    pub fn add_milk(&mut self, milk: u32) {
        self.milk += milk;
    }

    pub fn add_wool(&mut self, wool: u32) {
        self.wool += wool;
    }

    pub fn milk(&self) -> f64 {
        f64::from(self.milk) / f64::from(MILK_FLOATING_POINT_ADJUSTMENT)
    }

    pub fn wool(&self) -> u32 {
        self.wool
    }
}
