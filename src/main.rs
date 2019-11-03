use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use structopt::StructOpt;
use rand::Rng;

#[derive(StructOpt, Debug)]
#[structopt(name = "password_generator")]
struct Opt {
    #[structopt(short = "w", long = "words", default_value = "4")]
    words: usize,
}

fn main() {
    let opt = Opt::from_args();
    let path = Path::new("liste_francais.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().trim().to_lowercase())
        .collect::<Vec<String>>();

    let password = (0..opt.words).map(|_| {
        let r: usize = rand::thread_rng().gen_range(0, lines.len());
        lines[r].clone()
    }).collect::<Vec<String>>();

    let password = password.join("-");
    println!("{}", password);
}
