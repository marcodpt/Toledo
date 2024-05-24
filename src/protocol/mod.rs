mod toledo;

use serde::{Serialize};
use serde_json::{json};
use toledo::Toledo;
use std::error::Error;

#[derive(Serialize)]
pub struct Data {
    pub weight: f64,
    pub tare: f64,
    pub unit: String
}

impl Data {
    pub fn from_toledo(
        raw: &Vec<u8>,
        debug: bool
    ) -> Result<Data, Box<dyn Error>> {
        let toledo = Toledo::new(raw)?;
        if debug {
            println!("{:#?}", &toledo);
        }
        if toledo.moviment {
            Err("ERR_MOVIMENT".into())
        } else if toledo.error {
            Err("ERR_SCALE".into())
        } else {
            let base: f64 = 10.0;
            let base = base.powi(toledo.exponent);
            let weight: f64 = toledo.weight as f64;
            let weight = if toledo.negative {-1.0} else {1.0} * weight * base;

            let tare: f64 = if toledo.net {
                (toledo.tare as f64) * base
            } else {0.0};

            Ok(Data {
                weight,
                tare,
                unit: toledo.unit.clone()
            })
        }
    }

    pub fn check_unit(&self,
        unit: Option<&String>
    ) -> Result<&Self, Box<dyn Error>> {
        if let Some(unit) = unit {
            if unit != &self.unit {
                return Err("ERR_UNIT".into());
            }
        }

        Ok(self)
    }

    pub fn check_min_weight(&self,
        min_weight: Option<f64>
    ) -> Result<&Self, Box<dyn Error>> {
        if let Some(min_weight) = min_weight {
            if self.weight < min_weight {
                return Err("ERR_WEIGTH".into());
            }
        }

        Ok(self)
    }

    pub fn check_max_weight(&self,
        max_weight: Option<f64>
    ) -> Result<&Self, Box<dyn Error>> {
        if let Some(max_weight) = max_weight {
            if self.weight > max_weight {
                return Err("ERR_WEIGTH".into());
            }
        }

        Ok(self)
    }

    pub fn check_min_tare(&self,
        min_tare: Option<f64>
    ) -> Result<&Self, Box<dyn Error>> {
        if let Some(min_tare) = min_tare {
            if self.tare < min_tare {
                return Err("ERR_TARE".into());
            }
        }

        Ok(self)
    }

    pub fn check_max_tare(&self,
        max_tare: Option<f64>
    ) -> Result<&Self, Box<dyn Error>> {
        if let Some(max_tare) = max_tare {
            if self.tare > max_tare {
                return Err("ERR_TARE".into());
            }
        }

        Ok(self)
    }

    pub fn to_json_string(&self) -> String {
        json!(self).to_string()
    } 
}
