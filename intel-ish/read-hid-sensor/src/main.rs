#![allow(unused)]

use std::{fs::File, io, mem::size_of};
use clap::{Parser, ValueEnum};

/// Supported sensor types
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
#[clap(rename_all = "lower")]
enum SensorType {
    /// raw data
    Raw,
    /// light level
    Als,
    /// human presence + proximity
    Hpd,
    /// wake on approach
    Woa,
}

/// Clap CLI interface
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// the sensor to read
    #[clap(value_enum)]
    sensor_type: SensorType,
    /// the device to open
    dev: String,
}

struct Header {
    usage: u32,
    timestamp: u64,
    length: u32,
}

struct Report {
    header: Header,
    data: Vec<u8>,
}

fn read_header(dev: &mut impl io::Read) -> io::Result<Header> {
    let usage = {
        let mut usage_buf = [0u8; size_of::<u32>()];
        dev.read_exact(&mut usage_buf)?;
        u32::from_le_bytes(usage_buf)
    };

    let timestamp = {
        let mut timestamp_buf = [0u8; size_of::<u64>()];
        dev.read_exact(&mut timestamp_buf)?;
        u64::from_le_bytes(timestamp_buf)
    };

    let length = {
        let mut length_buf = [0u8; size_of::<u32>()];
        dev.read_exact(&mut length_buf)?;
        u32::from_le_bytes(length_buf)
    };

    Ok(Header {
        usage,
        timestamp,
        length,
    })
}

fn read_report(dev: &mut impl io::Read) -> io::Result<Report> {
    let header = read_header(dev)?;
    let mut data = vec![0u8; header.length as usize];
    dev.read_exact(&mut data)?;
    Ok(Report { header, data })
}

fn main() -> ! {
    let args = Args::parse();

    let dev = File::open(args.dev).expect("can't open dev file");
    let mut dev_buffered = io::BufReader::new(dev);

    loop {
        let Report { header, data } = read_report(&mut dev_buffered).expect("can't read report");
        let Header { timestamp, usage, .. } = header;

        print!("{timestamp}: 0x{usage:x} ");
        match args.sensor_type {
            SensorType::Als => {
                let illuminance = u32::from_le_bytes(data[0x1a..0x1e].try_into().unwrap());
                println!("illuminance: {illuminance}");
            }
            SensorType::Hpd => {
                let distance = u16::from_le_bytes(data[0x1a..=0x1b].try_into().unwrap());
                let certainty = u8::from(data[0x1e]);
                let presence = 0 != u8::from(data[0x1f]);
                println!("distance: {distance}mm, certainty: {certainty:3}%, presence: {presence}");
            }
            _ => {
                println!("raw: {data:02x?}");
            }
        }
    }
}
