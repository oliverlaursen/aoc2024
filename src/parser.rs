use std::{io::stdin, str::FromStr};

pub trait Parsing {
    fn to_vec<T: FromStr>(self, delimiter: &str) -> Vec<T>;
}

impl Parsing for String {
    fn to_vec<T: FromStr>(self, delimiter: &str) -> Vec<T> {
        self.split(delimiter)
            .map(|s| s.parse().ok().unwrap())
            .collect()
    }
}

fn read_line() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}