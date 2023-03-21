use std::io::{stdout, Write};
use curl::easy::{Easy, WriteError};

fn write_data(data: &[u8]) -> Result<usize, WriteError> {
    stdout().write_all(data)
        .map_err(|e| WriteError::from((e, data.len())))
        .map(|_| data.len())
}

fn main() {
    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.write_function(|data| write_data(data)).unwrap();
    easy.perform().unwrap();
}