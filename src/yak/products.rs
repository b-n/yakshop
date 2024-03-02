use std::ops::{Add, AddAssign};

#[derive(Default, Debug)]
pub struct Products {
    pub milk: f64,
    pub wool: u32,
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
