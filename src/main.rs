use clap::{Parser};
use tiny_http::{Server, Response, Header};
use serde_json::{json, Value};
use toml;
use std::error::Error;
use std::collections::HashMap;
use ascii_converter::decimals_to_string;
use std::fs::{read, write};
use std::path::{PathBuf};
use serialport;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    ///Device filepath.
    #[clap(required=true)]
    path: PathBuf,

    ///Test mode, data is read from a file instead of the serial port.
    #[clap(short, long, default_value = "false")]
    test: bool,

    ///Save the raw serial port data to a file for reference and testing.
    #[clap(short, long)]
    save: Option<PathBuf>,

    ///Baud rate.
    #[clap(short, long, default_value = "4800")]
    baud_rate: u32,

    ///HTTP port to listen to.
    #[clap(short, long, default_value = "8002")]
    port: u16,

    ///Error message language (en, pt).
    #[clap(short, long, default_value = "en")]
    lang: String,

    ///Print information on the terminal in real time.
    #[clap(short, long, default_value = "false")]
    debug: bool,

    ///Force weighing to be in the specified unit (Kg, Lb).
    #[clap(short, long)]
    unit: Option<String>,

    ///Minimum acceptable weight.
    #[clap(long)]
    min_weight: Option<f64>,

    ///Maximum acceptable weight.
    #[clap(long)]
    max_weight: Option<f64>,

    ///Minimum acceptable tare.
    #[clap(long)]
    min_tare: Option<f64>,

    ///Maximum acceptable tare.
    #[clap(long)]
    max_tare: Option<f64>,
}

#[derive(Debug)]
struct Protocol {
    exponent: i32,
    weight: u64,
    tare: u64,
    net: bool,
    negative: bool,
    error: bool,
    moviment: bool,
    unit: String,
    _energy: bool,
    stx: u8,
    cr: u8,
    _cs: u8,
    _check: u16,
    _a: u8,
    _b: u8,
    _c: u8
}

fn bit (num: u8, index: u8) -> bool {
    let mask = 1 << index;
    (mask & num) > 0
}

fn parse(raw: &Vec<u8>, cli: &Cli) -> Result<Value, Box<dyn Error>> {
    let mut data: Vec<u8> = Vec::new();
    for v in raw {
        data.push(v % 128);
    }
    let mut check: u16 = 0;
    for i in 0..18 {
        check = (check + (data[i] as u16)) % 128;
    }

    let weight = decimals_to_string(&data[4..10].to_vec())?;
    let tare = decimals_to_string(&data[10..16].to_vec())?;

    let p = Protocol {
        exponent: 2 - ((data[1] as i32) % 8),
        weight: str::parse::<u64>(&weight)?,
        tare: str::parse::<u64>(&tare)?,
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
        stx: data[0],
        cr: data[16],
        _cs: data[17],
        _check: check,
        _a: data[1],
        _b: data[2],
        _c: data[3]
    };

    if cli.debug {
        println!("{:#?}", &p);
    }

    if p.stx != 2 || p.cr != 13 /*|| p._check != p._cs as u16*/ {
        Err("ERR_INTEGRITY".into())
    } else if p.moviment {
        Err("ERR_MOVIMENT".into())
    } else if p.error {
        Err("ERR_SCALE".into())
    } else {
        if let Some(unit) = &cli.unit {
            if unit != &p.unit {
                return Err("ERR_UNIT".into());
            }
        }

        let base: f64 = 10.0;
        let base = base.powi(p.exponent);
        let weight: f64 = p.weight as f64;
        let weight = if p.negative {-1.0} else {1.0} * weight * base;

        let tare: f64 = if p.net {
            (p.tare as f64) * base
        } else {0.0};

        if let Some(min_weight) = cli.min_weight {
            if weight < min_weight {
                return Err("ERR_WEIGTH".into());
            }
        }

        if let Some(max_weight) = cli.max_weight {
            if weight > max_weight {
                return Err("ERR_WEIGTH".into());
            }
        }

        if let Some(min_tare) = cli.min_tare {
            if tare < min_tare {
                return Err("ERR_TARE".into());
            }
        }

        if let Some(max_tare) = cli.max_tare {
            if tare > max_tare {
                return Err("ERR_TARE".into());
            }
        }

        Ok(json!({
            "weight": weight,
            "tare": tare,
            "unit": p.unit.clone()
        }))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let en = include_str!("lang/en.toml");
    let pt = include_str!("lang/pt.toml");
    let lang = if &cli.lang == "pt" { pt } else { en };

    let lang: HashMap<String, String> = toml::from_str(lang)?;
    let mut scale = match cli.test {
        false => Some(serialport::new(
            cli.path.as_path().as_os_str().to_str().ok_or("unreachable")?,
            cli.baud_rate
        ).open()?),
        true => None
    };

    let host = format!("localhost:{}", cli.port);

    let header = Header::from_bytes(
        &b"Content-Type"[..],
        &b"application/json"[..]
    ).ok().ok_or("unreachable")?;
    let server = match Server::http(&host) {
        Ok(server) => server,
        Err(msg) => {
            return Err(msg.to_string().into());
        }
    };

    println!("Toledo scale server running at: http://{}", host);

    for request in server.incoming_requests() {
        let method = request.method().to_string();
        let url = request.url().to_string();
        request.respond(
            if &method != "GET" || &url != "/" {
                Response::from_string("").with_status_code(404)
            } else if let Some(ref mut scale) = scale {
                let mut data: Vec<u8> = vec![0; 18];
                match scale.read(data.as_mut_slice()) {
                    Ok(_) => {
                        if let Some(ref file) = cli.save {
                            write(file, &data)?;
                        };
                        match parse(&data, &cli) {
                            Ok(data) => {
                                let data = json!(data);
                                Response::from_string(data.to_string())
                                    .with_header(header.clone())
                            },
                            Err(err) => {
                                let err = err.to_string();
                                let body = lang.get(&err).unwrap_or(&err);
                                Response::from_string(body)
                                    .with_status_code(500)
                            }
                        }
                    },
                    Err(err) => {
                        if cli.debug {
                            println!("{}", err.to_string());
                        }
                        let err = String::from("ERR_BYTES");
                        let body = lang.get(&err).unwrap_or(&err);
                        Response::from_string(body).with_status_code(500)
                    }
                }
            } else {
                match read(&cli.path) {
                    Ok(data) => {
                        match parse(&data, &cli) {
                            Ok(data) => {
                                let data = json!(data);
                                Response::from_string(data.to_string())
                                    .with_header(header.clone())
                            },
                            Err(err) => {
                                let err = err.to_string();
                                let body = lang.get(&err).unwrap_or(&err);
                                Response::from_string(body)
                                    .with_status_code(500)
                            }
                        }
                    },
                    Err(err) => {
                        if cli.debug {
                            println!("{}", err.to_string());
                        }
                        let err = String::from("ERR_BYTES");
                        let body = lang.get(&err).unwrap_or(&err);
                        Response::from_string(body).with_status_code(500)
                    }
                }
            }
        )?;
    }
    Ok(())
}
