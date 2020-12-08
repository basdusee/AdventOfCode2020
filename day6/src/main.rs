use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::collections::HashSet;

// Some readfile thingy found somewhere on the interwebz
// better copied well than crafted badly yourself...
fn readfile<P>(filename: P) -> io::Result<io::Lines::<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// I put it all in a struct, because I love structs.
// count is the number of people in a group
// anyone is the unique answers all people gave (it's a set so unique)
// everyone is a vector of answers all people gave simultaneously
struct CustomsForm {
    count: usize,
    anyone: HashSet<char>,
    everyone: Vec<char>,
}

// Small impl to create a new empty struct
impl CustomsForm {
    fn new() -> CustomsForm {
        CustomsForm {
            count: 0,
            anyone: HashSet::new(),
            everyone: Vec::new(),
        }
    }
}

fn readcustomforms(file: &str) -> Vec<CustomsForm> {

    // We need a vec to store the structs in
    let mut customs: Vec<CustomsForm> = Vec::new();

    // Let's read all the lines 
    if let Ok(lines) = readfile(file) {
        // push the first (empty) struct in the vector
        // and create some persistent vars for the loop
        customs.push(CustomsForm::new());
        let mut groupmembercount: usize = 0;
        let mut retainvec: Vec<char> = Vec::new();

        for one_line in lines {
            if let Ok(line) = one_line {
                if line == "" {
                    // empty line indicates new form, so save the count, 
                    // push the data and start fresh
                    customs.last_mut().unwrap().everyone = retainvec;
                    customs.last_mut().unwrap().count = groupmembercount;
                    customs.push(CustomsForm::new());
                    groupmembercount = 0;
                    retainvec = vec![];
                } else {
                    let characters: Vec<char> = line.trim().chars().collect();
                    // For Part 2
                    // if this is the first line, store all the answers
                    // for every 2nth+ person, leave all chars in both vectors and delete the rest
                    if groupmembercount == 0 {
                        retainvec = characters.clone();
                    } else {
                        retainvec.retain(|&x| characters.contains(&x));
                    }
                    // For Part 1
                    // store all answers of this person in the set
                    // and add one person to the group counter
                    for answer in characters {
                        customs.last_mut().unwrap().anyone.insert(answer);
                    }
                    groupmembercount += 1;
                }
                
            }
        }
    }

    // return the struct with counts and hashsets
    customs
}

fn main() -> Result<(), Error> {

    // process the file,
    let customsdb = readcustomforms("input.txt");
    let mut total: usize = 0;

    for customsform in &customsdb {
        total += customsform.anyone.len();
    }
    println!("Part One: total count is {}", total);

    total = 0;

    for customsform in &customsdb {
        total += customsform.everyone.len();
    }
    println!("Part Two: total count is {}", total);
    
    // Me main, me always happy
    Ok(())
}
