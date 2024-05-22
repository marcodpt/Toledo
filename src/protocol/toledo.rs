use ascii_converter::decimals_to_string;
use std::error::Error;

#[derive(Debug)]
pub struct Toledo {
    pub exponent: i32,
    pub weight: u64,
    pub tare: u64,
    pub net: bool,
    pub negative: bool,
    pub error: bool,
    pub moviment: bool,
    pub unit: String,
    _energy: bool,
    _check: u8,
    _a: u8,
    _b: u8,
    _c: u8
}

fn bit (num: u8, index: u8) -> bool {
    let mask = 1 << index;
    (mask & num) > 0
}

impl Toledo {
    pub fn new(raw: &Vec<u8>) -> Result<Toledo, Box<dyn Error>> {
        let mut data: Vec<u8> = Vec::new();
        let mut check: u16 = 0;
        for v in raw {
            check = check + (*v as u16);
            data.push(*v % 128);
        }
        check = check % 128;

        let stx = data[0];
        let cr = data[16];

        if stx != 2 || cr != 13 || check != 0 {
            Err("ERR_INTEGRITY".into())
        } else {
            let weight = decimals_to_string(&data[4..10].to_vec())
                .ok().ok_or("ERR_INTEGRITY")?;
            let tare = decimals_to_string(&data[10..16].to_vec())
                .ok().ok_or("ERR_INTEGRITY")?;

            Ok(Toledo {
                exponent: 2 - ((data[1] as i32) % 8),
                weight: str::parse::<u64>(&weight)
                    .ok().ok_or("ERR_INTEGRITY")?,
                tare: str::parse::<u64>(&tare)
                    .ok().ok_or("ERR_INTEGRITY")?,
                net: bit(data[2], 0),
                negative: bit(data[2], 1),
                error: bit(data[2], 2),
                moviment: bit(data[2], 3),
                unit: if bit(data[2], 4) {
                    String::from("Kg")
                } else {
                    String::from("Lb")
                },
                _energy: bit(data[2], 6),
                _check: data[17],
                _a: data[1],
                _b: data[2],
                _c: data[3]
            })
        }
    }
}
