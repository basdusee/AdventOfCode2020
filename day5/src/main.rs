use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

// Some readfile thingy found somewhere on the interwebz
// better copied well than crafted badly yourself...
fn readfile<P>(filename: P) -> io::Result<io::Lines::<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn readboardingpass(file: &str) -> Vec<usize> {

    // We need a vec to store the id's
    let mut ids: Vec<usize> = Vec::new();

    // Let's read all the lines 
    if let Ok(lines) = readfile(file) {

        for one_line in lines {
            if let Ok(line) = one_line {

                // Reset all vars for every loop over row/serial
                let mut lowrow: usize = 0;
                let mut highrow: usize = 127;
                let mut lowseat: usize = 0;
                let mut highseat: usize = 7;

                // Just do a binary space partitioning thingy
                // need to add +1 to every addition or substraction
                // don't now why, too stupid to understand, but it works
                // is it because numbering is from 0 to 127?
                for letter in line.chars() {
                    match letter {
                        'F' => highrow -= ((highrow - lowrow) / 2) + 1,
                        'B' => lowrow += ((highrow - lowrow) / 2) + 1,
                        'L' => highseat -= ((highseat - lowseat) / 2) + 1,
                        'R' => lowseat += ((highseat - lowseat) / 2) + 1,
                        _ => println!("illegal char found in {}, it's {}", line, letter),
                    }
                }

                // let's calc and store the id of the found seat
                if (highrow == lowrow) & (highseat == lowseat) {
                    let id = lowrow * 8 + lowseat;
                    ids.push(id);
                } else {
                    println!("Code {} went wrong decoding, highrow {} is not equal to lowrow {} or highseat {} is not equal to lowseat {}",
                             line, highrow, lowrow, highseat, lowseat);
                }
            }
        }

    }

    // return the vec with ids 
    ids
}

fn main() -> Result<(), Error> {

    // process the file, returns a vec with all found boarding id's
    let boardingpass = readboardingpass("input.txt");

    // save the maxid for the range we use later to check
    // all available seats in the plane
    let maxid = boardingpass.iter().max().unwrap().clone();
    println!("Part One: highest id is {}", maxid);

    // start with 1 because 0 can never be your seat
    // and it causes a panic if starting from 0
    // it checks if the seat before and after exist before printing the result
    for looper in 1..=maxid {
        if !boardingpass.contains(&looper) {
           if (boardingpass.contains(&(looper - 1))) &
              (boardingpass.contains(&(looper + 1))) {
                  println!("Part Two, your seat id is {}", looper);
           }
        }
    }
    
    // Me main, me always happy
    Ok(())
}
