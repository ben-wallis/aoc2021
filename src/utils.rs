use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.ok().unwrap())
}
