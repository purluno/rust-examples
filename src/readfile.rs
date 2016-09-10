use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn i64_vector(path: &str) -> Vec<i64> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    reader.lines()
        .filter_map(|x| x.ok())
        .flat_map(|x| -> Vec<i64> {
            x.split_whitespace()
                .filter_map(|x| i64::from_str_radix(&x, 10).ok())
                .collect()
        })
        .collect()
}
