extern crate clap;

use clap::{App, Arg};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_hash<T: Hash>(t: &T, ppar: u64) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    ppar.hash(&mut s);
    s.finish()
}

fn main() {
    let matches = App::new("homework0")
        .version("1.0")
        .author("Lel")
        .about("Kek")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("path to a file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .value_name("UNSIGNED INTEGER")
                .help("number of tickets")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ppar")
                .short("p")
                .long("ppar")
                .value_name("UNSIGNED INTEGER")
                .help("parameter")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let count = u32::from_str(matches.value_of("count").unwrap()).unwrap() as u64;
    let ppar = u64::from_str(matches.value_of("ppar").unwrap()).unwrap();

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(line_content) = line {
                println!(
                    "{}: {}",
                    line_content,
                    calculate_hash(&line_content, ppar) % count + 1
                );
            }
        }
    }
}