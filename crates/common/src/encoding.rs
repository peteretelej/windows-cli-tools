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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn write_temp_file(content: &[u8]) -> tempfile::NamedTempFile {
        use std::io::Write;
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(content).unwrap();
        f.flush().unwrap();
        f
    }

    #[test]
    fn utf16le_bom_transcodes_to_utf8() {
        let mut data = vec![0xFF, 0xFE]; // BOM
        data.extend("hello\n".encode_utf16().flat_map(|u| u.to_le_bytes()));
        let f = write_temp_file(&data);
        let mut reader = open_input(f.path().to_str().unwrap()).unwrap();
        let mut output = String::new();
        reader.read_to_string(&mut output).unwrap();
        assert_eq!(output, "hello\n");
    }

    #[test]
    fn no_bom_passes_through() {
        let f = write_temp_file(b"plain text\n");
        let mut reader = open_input(f.path().to_str().unwrap()).unwrap();
        let mut output = String::new();
        reader.read_to_string(&mut output).unwrap();
        assert_eq!(output, "plain text\n");
    }

    #[test]
    fn missing_file_returns_error() {
        let result = open_input("/nonexistent_path_xyz/file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn utf16le_bom_multiline() {
        let mut data = vec![0xFF, 0xFE]; // BOM
        for c in "line one\nline two\n".encode_utf16() {
            data.extend(c.to_le_bytes());
        }
        let f = write_temp_file(&data);
        let reader = open_input(f.path().to_str().unwrap()).unwrap();
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        assert_eq!(lines, vec!["line one", "line two"]);
    }
}
