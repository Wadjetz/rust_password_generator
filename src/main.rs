extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

use rand::Rng;

fn main() {
    let path = Path::new("liste_francais.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().to_lowercase())
        .collect::<Vec<String>>();

    let password = (0..4).map(|_| {
        let r: usize = rand::thread_rng().gen_range(0, lines.len());
        lines[r].clone()
    }).collect::<Vec<String>>();

    println!("{}", password.join("-"));
}
