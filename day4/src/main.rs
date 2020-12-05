//#[macro_use]
//extern crate lazy_static;

use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use regex::Regex;

fn readfile<P>(filename: P) -> io::Result<io::Lines::<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<usize>,
}

impl Passport {
    fn new() -> Passport {
        Passport { 
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

fn readpassports(file: &str) -> Vec<Passport> {

    let mut passports: Vec<Passport> = Vec::new();

    if let Ok(lines) = readfile(file) {
        passports.push(Passport::new());

        for one_line in lines {
            if let Ok(line) = one_line {
                if line == "" {
                    passports.push(Passport::new());
                } else {
                    let fields = line.split_whitespace();
                    for field in fields {
                        let keyvalues: Vec<&str> = field.split(':').collect();
                        match keyvalues[0] {
                            "byr" => passports.last_mut().unwrap().byr = Some(keyvalues[1].parse::<usize>().unwrap()),
                            "iyr" => passports.last_mut().unwrap().iyr = Some(keyvalues[1].parse::<usize>().unwrap()),
                            "eyr" => passports.last_mut().unwrap().eyr = Some(keyvalues[1].parse::<usize>().unwrap()),
                            "hgt" => passports.last_mut().unwrap().hgt = Some(keyvalues[1].to_string()),
                            "hcl" => passports.last_mut().unwrap().hcl = Some(keyvalues[1].to_string()),
                            "ecl" => passports.last_mut().unwrap().ecl = Some(keyvalues[1].to_string()),
                            "pid" => passports.last_mut().unwrap().pid = Some(keyvalues[1].to_string()),
                            "cid" => passports.last_mut().unwrap().cid = Some(keyvalues[1].parse::<usize>().unwrap()),
                            _ => println!("Unknown identifier found in passport: {}", keyvalues[0]),
                        }
                    }
                }
            }
        }

    }

    // return the passports in a vect of password structs
    passports
}

fn comphgt(height: &str) -> bool {
    if height == "None" {
        false
    } else {
        let unit = &height[height.len()-2..];
        if unit == "cm" {
            let number = height[..height.len()-2].parse::<usize>().unwrap_or_default();
            if (number >= 150) & (number <= 193) {
                true
            } else {
                false
            }
        } else if unit == "in" {
            let number = height[..height.len()-2].parse::<usize>().unwrap_or_default();
            if (number >= 59) & (number <= 76) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn main() -> Result<(), Error> {

    let mut validpapers: usize = 0;
    let papers = readpassports("input.txt");

    for passport in papers {
       if (passport.byr != None) &
          (passport.iyr != None) &
          (passport.eyr != None) &
          (passport.hgt != None) &
          (passport.hcl != None) &
          (passport.ecl != None) &
          (passport.pid != None) { 
            validpapers += 1;
        }
    }

    println!("Part One: Found {} valid passwords", validpapers);

    let papers = readpassports("input.txt");
    let mut validpapers2: usize = 0;
    
    let hcl_re = Regex::new("^#[0-9a-f]{6}$").unwrap();
    let ecl_vec = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let pid_re = Regex::new("^[0-9]{9}$").unwrap();

    for passport in papers {
       if ((passport.byr.unwrap_or_default() >= 1920) & (passport.byr.unwrap_or_default() <= 2002) ) &
          ((passport.iyr.unwrap_or_default() >= 2010) & (passport.iyr.unwrap_or_default() <= 2020) ) &
          ((passport.eyr.unwrap_or_default() >= 2020) & (passport.eyr.unwrap_or_default() <= 2030) ) &
          (comphgt(&passport.hgt.as_ref().unwrap_or(&"None".to_string()))) &
          (hcl_re.is_match(&passport.hcl.as_ref().unwrap_or(&"None".to_string()))) & 
          (ecl_vec.iter().any(|&i| i == passport.ecl.as_ref().unwrap_or(&"None".to_string()))) &
          (pid_re.is_match(&passport.pid.as_ref().unwrap_or(&"None".to_string()))) { 
            validpapers2 += 1;
        }
    }
    
    println!("Part Two: Found {} valid passwords", validpapers2);

    // br.lines()
    //    .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    //   .collect()

    Ok(())
}
