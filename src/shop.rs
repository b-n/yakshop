use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

use crate::{Products, Yak, YakShopError};

#[derive(Default, Deserialize, Debug, Clone)]
pub struct Shop {
    #[serde(rename = "$value")]
    yaks: Vec<Yak>,
    #[serde(skip_deserializing)]
    pub elapsed_days: u32,
    #[serde(skip_deserializing)]
    pub produced_products: Products,
}

impl Display for Shop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"In Stock:
    {:.3} liters of milk
    {} skins of wool
Herd:"#,
            self.produced_products.milk(),
            self.produced_products.wool()
        )?;

        for yak in &self.yaks {
            write!(f, "\n    {yak}")?;
        }

        Ok(())
    }
}

impl TryFrom<&PathBuf> for Shop {
    type Error = YakShopError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let herd_config = path.as_path();
        if !herd_config.exists() {
            return Err(YakShopError::ConfigFileNotFound(path.clone()));
        }

        let herd_xml =
            std::fs::read_to_string(herd_config).expect("Could not read herd.xml file to string");

        // TODO: These serde error messages could be a lot nicer
        serde_xml_rs::from_str(&herd_xml)
            .map_err(|err| YakShopError::ConfigFileParseError(format!("{err:?}")))
    }
}

impl Shop {
    pub fn step_days(&mut self, days: u32) {
        for yak in &mut self.yaks {
            // Add the products if the yak is still producing products
            if let Some(products) = yak.step_days(days) {
                self.produced_products += products;
            }
        }

        self.elapsed_days += days;
    }

    #[must_use]
    pub fn yaks(&self) -> &[Yak] {
        &self.yaks
    }

    #[must_use]
    pub fn consume_products(
        &mut self,
        milk: Option<f64>,
        wool: Option<u32>,
    ) -> (Option<f64>, Option<u32>) {
        // A naive, but working approach of product consumption
        let mut consumed_milk: Option<f64> = None;
        if let Some(milk) = milk {
            if self.produced_products.milk() >= milk {
                consumed_milk = Some(milk);
            }
        }

        let mut consumed_wool: Option<u32> = None;
        if let Some(wool) = wool {
            if self.produced_products.wool() >= wool {
                consumed_wool = Some(wool);
            }
        }

        (consumed_milk, consumed_wool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures_path() -> PathBuf {
        PathBuf::from("./tests/fixtures")
    }

    #[test]
    fn test_try_from_valid_single() {
        let herd_xml = fixtures_path().join("valid_single.xml");
        let shop = Shop::try_from(&herd_xml).unwrap();
        assert_eq!(shop.yaks.len(), 1);
    }

    #[test]
    fn test_try_from_valid_multiple() {
        let herd_xml = fixtures_path().join("valid_multi.xml");
        let shop = Shop::try_from(&herd_xml).unwrap();
        assert_eq!(shop.yaks.len(), 3);
    }

    #[test]
    fn test_try_from_invalid_age() {
        let herd_xml = fixtures_path().join("invalid_age.xml");
        let result = Shop::try_from(&herd_xml);
        assert!(matches!(result, Err(YakShopError::ConfigFileParseError(_))));
    }

    #[test]
    fn test_try_from_invalid_path() {
        let herd_xml = fixtures_path().join("invalid_path.xml");
        let result = Shop::try_from(&herd_xml);
        assert!(matches!(result, Err(YakShopError::ConfigFileNotFound(_))));
    }

    #[test]
    fn test_13_days() {
        let herd_xml = fixtures_path().join("valid_multi.xml");
        let mut shop = Shop::try_from(&herd_xml).unwrap();
        shop.step_days(13);

        assert_eq!(shop.elapsed_days, 13);
        assert_ulps_eq!(shop.produced_products.milk(), 1104.480);
        assert_eq!(shop.produced_products.wool(), 3);
    }

    #[test]
    fn test_14_days() {
        let herd_xml = fixtures_path().join("valid_multi.xml");
        let mut shop = Shop::try_from(&herd_xml).unwrap();
        shop.step_days(14);

        assert_eq!(shop.elapsed_days, 14);
        assert_ulps_eq!(shop.produced_products.milk(), 1188.810);
        assert_eq!(shop.produced_products.wool(), 4);
    }
}
