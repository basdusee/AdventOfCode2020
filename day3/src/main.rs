use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

// Some random "read a file from disk" code I scraped from the web
fn readfile<P>(filename: P) -> io::Result<io::Lines::<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Let's use a struct, that looks really Rustacean l33t codish
struct Mapwalker {
    tree: usize,
    open: usize,
    cursor: [usize; 2],
}

fn checktrees(cursup: [usize; 2]) -> Mapwalker {
    let mut walker = Mapwalker{tree: 0, open: 0, cursor: [0, 0]};

    if let Ok(lines) = readfile("input.txt") {
        for one_line in lines {
            if let Ok(line) = one_line {
                if walker.cursor[1] % cursup[1] == 0 { 
                    let spot = line.chars().nth(walker.cursor[0]).unwrap();
                    if spot == '#' {
                        walker.tree += 1;
                    } else {
                        walker.open += 1;
                    }
                    walker.cursor[0] += cursup[0];
                    if walker.cursor[0] >= line.len() {
                        walker.cursor[0] -= line.len()
                    }
                }

                // allways increase line walker....
                walker.cursor[1] += 1;
            }
        }
    }
    // skip starting point
    walker.open -= 1;

    walker
}

fn main() -> Result<(), Error> {

    // This are the input paths, in a vec of arrays.
    let slopes: Vec<[usize; 2]> = vec![ [1, 1], [3, 1], [5, 1], [7, 1], [1, 2] ];

    let mut trees: Vec<usize> = Vec::with_capacity(5);

    // Push the found trees of the combination onto the trees vector, for later multiplication,
    // and print some stuff to make your programmer daddy happy now he knows what you are 
    // doing behind the scenes, my little program.
    for slope in slopes {
        let result = checktrees(slope);
        trees.push(result.tree);
        println!("for left {} down {}: Found {} trees and {} open spots", slope[0], slope[1], result.tree, result.open);
    }

    // multiply the heck out of the found trees, and print
    let total = trees[0] * trees[1] * trees[2] * trees[3] * trees[4];
    println!("All number of trees multiplied together = {}", total);

    // main is a happy camper, allways. keep up the positive vibe.
    Ok(())
}
