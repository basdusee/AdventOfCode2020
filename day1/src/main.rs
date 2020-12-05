use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

// Just code I ripped somewhere of the internet to read a file
fn readfile<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {

    // Create a path to the file
    let vec = readfile(File::open("input.txt")?)?;

    // Do some very ugly for-loop nesting (this can be optimized as Fck)
    for x in &vec {
        for y in &vec {
            if x + y == 2020 {
                println!("Twosome: Found an answer! x = {}, y = {} and x * y = {}", x, y, x*y);
            };
        };
    };

    // Do some very ugly for-loop nesting again (this can be optimized even more)
    for x in &vec {
        for y in &vec {
            for z in &vec {
                if x + y + z == 2020 {
                    println!("Threesome: Found an answer! x = {}, y = {}, z = {}  and x * y * z = {}", x, y, z, x*y*z);
                };
            };
        };
    };

    // This is useful, a main which always returns Ok...
    Ok(())

}
