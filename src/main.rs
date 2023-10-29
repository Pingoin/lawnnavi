#![warn(rust_2018_idioms)]

use futures::stream::StreamExt;
use std::{io, str};
use tokio_util::codec::Decoder;

use bytes::BytesMut;
use nmea::SentenceType::RMC;
use serde_json::to_string_pretty;
use tokio_serial::SerialPortBuilderExt;

mod gnss;
mod mutex_box;

const DEFAULT_TTY: &str = "/dev/ttyS0";

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {
    let tty_path = DEFAULT_TTY;

    let mut port = tokio_serial::new(tty_path, 9600).open_native_async()?;

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let mut reader = LineCodec.framed(port);

    let mut gnss = gnss::Gnss::new();

    while let Some(line_result) = reader.next().await {
        let mut line = line_result.expect("Failed to read line");
        line.retain(|c| (c != '\r' && c != '\n'));

        if let Some(data) = gnss.parse(&line) {
            if data == RMC {
                if let Ok(string) = to_string_pretty(&gnss) {
                    println!("{}", string);
                }
            }
        }
    }
    Ok(())
}
