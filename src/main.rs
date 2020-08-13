use std::str::FromStr;
use std::fmt::{self, Display};

fn main() {
    let bytes = [0x10, 0x20, 0x30, 0x40, 0x10, 0x20, 0x50, 0x50, 0x90, 0x90, 0x80];
    let pattern = "90 ? 80";
    let matches = scan(&bytes, &pattern);

    println!("matches -> {:?}", matches);
}

pub fn scan(bytes: &[u8], pattern: &str) -> Result<Option<Vec<usize>>, Error> {
    let pattern = Pattern::from_str(pattern)?;
    let mut matches = Vec::new();

    for i in 0..bytes.len() {
        if pattern_matches(&bytes[i..], &pattern) {
            matches.push(i);
        }
    }

    if matches.is_empty() {
        Ok(None)
    } else {
        Ok(Some(matches))
    }
}

fn pattern_matches(bytes: &[u8], pattern: &Pattern) -> bool {
    if bytes.len() < pattern.len() {
        false
    } else {
        pattern == bytes
    }
}

// Error
#[derive(Debug)]
pub struct Error {
    e: String,
}

impl Error {
    pub fn new(e: String) -> Self {
        Self { e }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Pattern scan error: {}", self.e)
    }
}

impl std::error::Error for Error {}

// PatternByte
enum PatternByte {
    Byte(u8),
    Any,
}

impl FromStr for PatternByte {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "?" {
            Ok(Self::Any)
        } else {
            let n = match u8::from_str_radix(s, 16) {
                Ok(n) => Ok(n),
                Err(e) => Err(Error::new(format!("from_str_radix failed: {}", e))),
            }?;

            Ok(Self::Byte(n))
        }
    }
}

impl PartialEq<u8> for PatternByte {
    fn eq(&self, other: &u8) -> bool {
        match self {
            PatternByte::Any => true,
            PatternByte::Byte(b) => b == other,
        }
    }
}

// Pattern
struct Pattern {
    bytes: Vec<PatternByte>,
}

impl Pattern {
    fn new(bytes: Vec<PatternByte>) -> Self {
        Self { bytes }
    }

    fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut bytes = Vec::new();

        for segment in s.split_ascii_whitespace() {
            bytes.push(PatternByte::from_str(segment)?);
        }

        Ok(Self::new(bytes))
    }
}

impl PartialEq<[u8]> for Pattern {
    fn eq(&self, other: &[u8]) -> bool {
        Iterator::zip(self.bytes.iter(), other.iter()).all(|(pb, b)| pb == b)
    }
}
