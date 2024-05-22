mod protocol;
mod message;

use protocol::Data;
use message::Message;

use clap::{Parser};
use tiny_http::{Server, Response, Header};
use std::error::Error;
use std::fs::{read, write};
use std::path::{PathBuf};
use std::io::Read;
use std::{thread, time::Duration};
use serialport;
use serialport::{SerialPort, ClearBuffer::Input};

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

fn parse(raw: &Vec<u8>, cli: &Cli) -> Result<String, Box<dyn Error>> {
    Ok(Data::from_toledo(raw, cli.debug)?
        .check_unit(cli.unit.as_ref())?
        .check_min_weight(cli.min_weight)?
        .check_max_weight(cli.max_weight)?
        .check_min_tare(cli.min_tare)?
        .check_max_tare(cli.max_tare)?
        .to_json_string()
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let msg = Message::new(&cli.lang)?;

    let mut scale = match cli.test {
        false => Some(serialport::new(
            cli.path.as_path().as_os_str().to_str().ok_or("unreachable")?,
            cli.baud_rate
        ).open_native()?),
        true => None
    };

    let host = format!("localhost:{}", cli.port);

    let header = Header::from_bytes(
        &b"Content-Type"[..],
        &b"application/json"[..]
    ).ok().ok_or("unreachable")?;
    let server = match Server::http(&host) {
        Ok(server) => server,
        Err(err) => {
            return Err(err.to_string().into());
        }
    };

    println!("Serial scale server running at: http://{}", host);

    for request in server.incoming_requests() {
        let method = request.method().to_string();
        let url = request.url().to_string();
        println!("{} {}", method, url);
        request.respond(
            if &method != "GET" || &url != "/" {
                Response::from_string("").with_status_code(404)
            } else if let Some(ref mut scale) = scale {
                let mut data: Vec<u8> = vec![0; 18];

                scale.clear(Input)?;
                thread::sleep(Duration::from_millis(500));

                match scale.read(data.as_mut_slice()) {
                    Ok(18) => {
                        if let Some(ref file) = cli.save {
                            write(file, &data)?;
                        };
                        match parse(&data, &cli) {
                            Ok(data) => {
                                Response::from_string(data)
                                    .with_header(header.clone())
                            },
                            Err(err) => {
                                let err = err.to_string();
                                Response::from_string(msg.err(&err))
                                    .with_status_code(500)
                            }
                        }
                    },
                    Ok(_) => {
                        Response::from_string(msg.err("ERR_INTEGRITY"))
                            .with_status_code(500)
                    },
                    Err(err) => {
                        if cli.debug {
                            println!("{}", err.to_string());
                        }
                        Response::from_string(msg.err("ERR_PORT"))
                            .with_status_code(500)
                    }
                }
            } else {
                match read(&cli.path) {
                    Ok(data) => {
                        match parse(&data, &cli) {
                            Ok(data) => {
                                Response::from_string(data)
                                    .with_header(header.clone())
                            },
                            Err(err) => {
                                let err = err.to_string();
                                Response::from_string(msg.err(&err))
                                    .with_status_code(500)
                            }
                        }
                    },
                    Err(err) => {
                        if cli.debug {
                            println!("{}", err.to_string());
                        }
                        Response::from_string(msg.err("ERR_PORT"))
                            .with_status_code(500)
                    }
                }
            }
        )?;
    }
    Ok(())
}
