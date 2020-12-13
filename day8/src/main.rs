use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::time::Instant;

// we need registers to be a proper computer
struct Computer {
    accumulator: isize,
    ipointer: usize,
    history: Vec<usize>,
}

// We need a computer, it's in the form of a function in this case
// output is (accumulator, crashed?)
fn compute(program: &Vec<(&str, isize)>) -> (isize, bool) {
    // start with a freshly booted computer
    let mut computer = Computer {
                           accumulator: 0,
                           ipointer: 0,
                           history: Vec::new(),
                       };

    // we are not crashed yet! (yay)
    let mut crash = false;

    // loop de loop, run the computer. 
    // ipointer controls which instruction is executed
    // only stops if program is finished 
    // or computer crashed (did same instruction twice)
    loop { 
            if computer.ipointer >= program.len() {
                break;
            } else if computer.history.contains(&computer.ipointer) {
                // yep, we crashed hard.
                crash = true;
                break;
            } else {
                // save execution history to check if we are in a crash loop
                computer.history.push(computer.ipointer);
                // the core of every modern high-end CPU is a match statement.. apparently
                // match on instruction and execute accordingly
                match program[computer.ipointer].0 {
                    "acc" => {computer.accumulator += program[computer.ipointer].1; computer.ipointer += 1;},
                    "jmp" => { if program[computer.ipointer].1 >= 0 {
                                 computer.ipointer += program[computer.ipointer].1.abs() as usize;
                             } else {
                                 computer.ipointer -= program[computer.ipointer].1.abs() as usize;
                             }},
                    "nop" => computer.ipointer += 1,
                    _ => panic!("wrong instruction found! {}", program[computer.ipointer].0),
               }
            }
    }
    
    // return the accumulator at the moment the computer stops.
    (computer.accumulator, crash)
}

fn main() -> Result<(), Error> {
    // new and improved way to load input files.
    // Don't know if it is faster, but it is waaay shorter and more comprehensible
    let mut input = String::new();
    let mut file = File::open("input.txt").expect("Could not read input.txt");
    file.read_to_string(&mut input).expect("Could not read input.txt");

    // start timing when the file is read in mem and we start to do stuff.
    let now = Instant::now();

    // we need a place to store the parsed input.txt as a program
    let mut program: Vec<(&str, isize)> = Vec::new();

    // let's process the input and store it as tuples in a vec, calling it a "program".
    input.trim().split('\n').for_each(|x| {
                                let execline = x.split_whitespace().collect::<Vec<&str>>();
                                let instruction = execline[0];
                                let argument = execline[1].trim().parse::<isize>().unwrap();
                                program.push((instruction, argument));
                            });

    println!("Part One: accumulator at start of infinite loop is: {}", compute(&program).0);  
    
    // ok check the stopwatch, was this fast in any way? And reset stopwatch.
    println!("Part One took me: {}ms", now.elapsed().as_millis());
    let now = Instant::now();

    // the bug is either a jmp should be a nop or a nop should be a jmp
    // it's only one of these
    // so just change all the jmp to nop and nop for jmp
    // one by one (not simultaneouslt)
    // and run the program and check if we reach the end.
    // the answer is in the accumulator, if we didn't crash
    for index in 0..program.len() {
        let mut altprog = program.clone();
        
        // swap jmp/nop.
        // skip program execution/testing with "continue" if it's "acc"
        // that saves time executing from 246ms to 104ms on my old laptop
        match altprog[index].0 {
            "jmp" => altprog[index] = ("nop", altprog[index].1),
            "nop" => altprog[index] = ("jmp", altprog[index].1),
            _ => continue,
        }
        
        // if not crashed, print accumulator, which is the answer
        let result = compute(&altprog);
        if !result.1 {
            println!("Part Two: program finished and accumulator is: {}", result.0);
            break;
        } 
    }
            
    // ok check the stopwatch, was this fast in any way?
    println!("Part Two took me: {}ms", now.elapsed().as_millis());

    // Me main, me always happy
    Ok(())
}
