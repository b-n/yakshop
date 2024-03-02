use crate::Yak;
use crate::YakShopError;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

#[derive(Default, Deserialize, Debug)]
pub struct Shop {
    #[serde(rename = "$value")]
    yaks: Vec<Yak>,
    #[serde(skip_deserializing)]
    milk: f64,
    #[serde(skip_deserializing)]
    skins: usize,
    #[serde(skip_deserializing)]
    pub elapsed_days: u32,
}

impl Display for Shop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"In Stock:
    {} liters of milk
    {} skins of wool
Herd:"#,
            self.milk, self.skins
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
            yak.step_days(days);
        }

        self.elapsed_days += days;
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
}
