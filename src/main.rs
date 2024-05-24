mod protocol;
mod message;
mod reader;

use protocol::Data;
use message::Message;
use reader::Reader;

use clap::Parser;
use tiny_http::{Server, Response, Header};
use std::error::Error;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::Write;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    ///Device filepath.
    #[clap(required=true)]
    path: PathBuf,

    ///Save the raw serial port data to a file for reference and testing.
    #[clap(short, long)]
    save: Option<PathBuf>,

    ///Baud rate. Set to zero for test mode where data is read from a file one line per request.
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

fn read_scale(
    reader: &mut Reader,
    cli: &Cli
) -> Result<String, Box<dyn Error>> {
    let data = reader.read()?;
    if let Some(ref file) = cli.save {
        OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file)?
            .write_all(&data)?;
    }
    Ok(Data::from_toledo(&data, cli.debug)?
        .check_unit(cli.unit.as_ref())?
        .check_min_weight(cli.min_weight)?
        .check_max_weight(cli.max_weight)?
        .check_min_tare(cli.min_tare)?
        .check_max_tare(cli.max_tare)?
        .to_json_string()
    )
}

fn read_scale_attempts(
    reader: &mut Reader,
    cli: &Cli,
    attempts: u8
) -> Result<String, Box<dyn Error>> {
    match read_scale(reader, cli) {
        Ok(data) => Ok(data),
        Err(err) => {
            if attempts > 1 && cli.baud_rate > 0 {
                read_scale_attempts(reader, cli, attempts - 1)
            } else {
                Err(err)
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let msg = Message::new(&cli.lang)?;
    let mut reader = Reader::new(&cli.path, cli.baud_rate)?;

    let host = format!("localhost:{}", cli.port);

    let header = Header::from_bytes(
        &b"Content-Type"[..],
        &b"application/json; charset=UTF-8"[..]
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
        request.respond(
            if &method != "GET" || &url != "/" {
                Response::from_string("").with_status_code(404)
            } else {
                match read_scale_attempts(&mut reader, &cli, 4) {
                    Ok(json) => {
                        Response::from_string(json)
                            .with_header(header.clone())
                    },
                    Err(err) => {
                        let err = err.to_string();
                        Response::from_string(msg.err(&err))
                            .with_status_code(500)
                    }
                }
            }
        )?;
    }
    Ok(())
}
