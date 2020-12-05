use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

// still some stuff I ripped from the internet to read a file
fn readfile<P>(filename: P) -> io::Result<io::Lines::<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Error> {

    let mut valid: usize = 0;
    let mut invalid: usize = 0;

    // Iterate over the lines in the file
    // 7-10 h: phdkrrhkmhh
    // rule is: number of occurences of char must be in range 7-10
    if let Ok(lines) = readfile("input.txt") {
        for one_line in lines {
            if let Ok(line) = one_line {
                let mut elements = line.split(' ');
                let mut range = elements.next().unwrap().split('-');
                let low = range.next().unwrap().parse::<usize>().unwrap();
                let high = range.next().unwrap().parse::<usize>().unwrap();
                let character = elements.next().unwrap().strip_suffix(':').unwrap();
                let password = elements.next().unwrap();
                let count = password.matches(character).count();
                if count >= low && count <= high {
                    valid += 1;
                } else {
                    invalid += 1;
                }
            }
        }
    }

    println!("Old rules: Found {} valid passwords and {} invalid passwords", valid, invalid);

    let mut valid: usize = 0;
    let mut invalid: usize = 0;

    // Iterate over the lines in the file
    // 7-10 h: phdkrrhkmhh
    // rule is: 7-10 are positions
    // on _only one_ of the positions, char _must_ be present
    // and use way too many intermediate vars please (sigh what a mess...)
    if let Ok(lines) = readfile("input.txt") {
        for one_line in lines {
            if let Ok(line) = one_line {
                let mut elements = line.split(' ');
                let mut positions = elements.next().unwrap().split('-');
                let first = positions.next().unwrap().parse::<usize>().unwrap();
                let last = positions.next().unwrap().parse::<usize>().unwrap();
                let character = elements.next().unwrap().strip_suffix(':').unwrap().chars().nth(0).unwrap();
                let password: Vec<char> = elements.next().unwrap().chars().collect::<Vec<_>>();
                let test1: bool = password[first-1] == character;
                let test2: bool = password[last-1] == character;
                if test1 ^ test2 {
                    valid += 1;
                } else {
                    invalid += 1;
                }
            }
        }
    }

    println!("New rules: Found {} valid passwords and {} invalid passwords", valid, invalid);

    // main is allways happy, no matter what happens..
    Ok(())
}
