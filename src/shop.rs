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

        let shop: Shop = serde_xml_rs::from_str(&herd_xml).expect("Could not parse herd.xml file");

        Ok(shop)
    }
}
