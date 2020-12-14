use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::time::Instant;
//use std::collections::HashSet;

fn first_broken_number(code: &Vec<usize>, preamble: usize) -> usize {
    // being nifty, this is probably not needed
    assert!(code.len() > preamble);
    
    let mut answer: usize = 0;
    
    // use a moving "window" (slice) and check if there are sums 
    // using a temp var called "found", which is ugly, but works.
    for cursor in preamble+1..code.len() {
        let mut found: bool = false;
        let sum = code.get(cursor).unwrap();
        let current_slice = code.get(cursor-preamble..cursor).unwrap(); 
        current_slice.iter().for_each(|acc| {
                        if acc < sum {
                            if current_slice.contains(&(sum-acc)) & (sum - acc != *acc) {
                                found = true;
                            }
                        }
        });
        if !found {
            answer = *sum; 
            break;
        }
    }
    // return the first found broken number
    answer
}

fn get_contiguous_numbers(code: &Vec<usize>, broken: usize) -> usize {
    
    // define two cursors to create a slice
    // which we can sum up and compare to "broken"
    let mut bottom: usize = 0;
    let mut top: usize = 1;

    // loop-de-loop, .fold adds the numbers. stop if it's identical to "broken".
    loop {
        let current_slice = code.get(bottom..=top).unwrap();
        match current_slice.iter().fold(0, |acc, x| acc + x) {
            x if x < broken => top += 1,
            x if x > broken => {bottom += 1; top = bottom + 1},
            x if x == broken => break current_slice.iter().min().unwrap() + 
                                      current_slice.iter().max().unwrap(),
            _ => panic!("weird return, no match"),
        }
    }
}

fn main() -> Result<(), Error> {
    // new and improved way to load input files.
    // Don't know if it is faster, but it is waaay shorter and more comprehensible
    let mut input = String::new();
    let mut file = File::open("input.txt").expect("Could not read input.txt");
    file.read_to_string(&mut input).expect("Could not read input.txt");

    // start timing when the file is read in mem and we start to do stuff.
    let now = Instant::now();

    // let's parse all the numbers into a vector
    let code: Vec<usize> = input.trim().split('\n')
                                .map(|x| x.parse::<usize>().unwrap_or_default()) 
                                .collect::<Vec<usize>>();

    // save the result from Part One, we need it in Part Two
    let broken_number = first_broken_number(&code, 25);
    println!("Part One: the first broken number is: {}", broken_number);
    
    println!("Part Two: the secret code is: {}", get_contiguous_numbers(&code, broken_number));
    
    println!("It took me: {}ms", now.elapsed().as_millis());

    // Me main, me always happy
    Ok(())
}
