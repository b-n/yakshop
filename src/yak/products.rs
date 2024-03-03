use std::ops::{Add, AddAssign};

/// Milk production suffers from floating point errors. Do all math without floating point and
/// adjust with this constant at the end to get the correct result.
const MILK_FLOATING_POINT_ADJUSTMENT: u32 = 100;
/// A yak can only be shaved after it is 100 days (1 year) old.
pub const MIN_YAK_SHAVE_AGE: u32 = 100;

const BASE_MILK_PRODUCTION: u32 = 5_000;
const MINIMUM_WOOL_SHAVING_PERIOD: f64 = 8.00;
const WOOL_SHAVING_AGE_ADJUSTMENT: f64 = 0.01;

pub fn yak_milk_production(age: u32) -> u32 {
    // SAFETY: yaks should die before they 1667 days old, however if technology advances and they
    // are able to stay alive past that, we should ensure they don't start consuming milk instead.
    BASE_MILK_PRODUCTION.saturating_sub(age * 3)
}

pub fn yak_can_produce_wool(age: u32, age_last_shaved: u32) -> bool {
    if age < MIN_YAK_SHAVE_AGE {
        return false;
    }

    let float_age = f64::from(age);

    // The next shave date is 8 + (0.01 * age years after the last shave)
    let next_shave_date: f64 = f64::from(age_last_shaved)
        + MINIMUM_WOOL_SHAVING_PERIOD
        + (float_age * WOOL_SHAVING_AGE_ADJUSTMENT);

    float_age >= next_shave_date
}

#[derive(Default, Debug, Clone)]
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

    #[must_use]
    pub fn milk(&self) -> f64 {
        f64::from(self.milk) / f64::from(MILK_FLOATING_POINT_ADJUSTMENT)
    }

    #[must_use]
    pub fn wool(&self) -> u32 {
        self.wool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yak_milk_production() {
        assert_eq!(yak_milk_production(0), 5_000);
        assert_eq!(yak_milk_production(100), 5_000 - 3 * 100);
        // Last day of milk production
        assert_eq!(yak_milk_production(1666), 2);
        // Every day after that is 0
        assert_eq!(yak_milk_production(1667), 0);
        assert_eq!(yak_milk_production(1668), 0);
    }
}
