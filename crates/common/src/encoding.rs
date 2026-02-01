use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn open_input(path: &str) -> io::Result<impl BufRead> {
    let file = File::open(path)?;
    let reader = DecodeReaderBytesBuilder::new()
        .bom_sniffing(true)
        .build(file);
    Ok(BufReader::new(reader))
}

pub fn open_stdin() -> io::Result<impl BufRead> {
    let reader = DecodeReaderBytesBuilder::new()
        .bom_sniffing(true)
        .build(io::stdin());
    Ok(BufReader::new(reader))
}

pub fn open_input_or_stdin(path: Option<&str>) -> io::Result<Box<dyn BufRead>> {
    match path {
        Some(p) => {
            let file = File::open(p)?;
            let reader = DecodeReaderBytesBuilder::new()
                .bom_sniffing(true)
                .build(file);
            Ok(Box::new(BufReader::new(reader)))
        }
        None => {
            let reader = DecodeReaderBytesBuilder::new()
                .bom_sniffing(true)
                .build(io::stdin());
            Ok(Box::new(BufReader::new(reader)))
        }
    }
}
