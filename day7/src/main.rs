use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::time::Instant;
use std::collections::HashSet;

// I'll put all bags in its own type/struct
// This is a bad idea because it overcomplicates stuff
#[derive(Debug, Default)]
struct ColorBag {
    color: String,
    count: usize,
    contains: Vec<ColorBag>,
}

// Recursive function, because Inception is a great movie
// this still is for-if-loop galore, should be fixed
fn check_gold(bags: &Vec<ColorBag>, color: &str) -> HashSet<String> {
    let mut allthebags: HashSet<String> = HashSet::new();
    for bag in bags {
        for innerbag in &bag.contains {
            if innerbag.color == color {
                allthebags.insert(bag.color.clone());
                // How do you properly merge/concat two HashMaps?
                for x in check_gold(&bags, &bag.color) {
                    allthebags.insert(x);
                }
            }
        }
    }
    // return a HashSet with all the (unique) bag color names
    allthebags
}

// Recursive function to add all the bags in the bags in the bags
fn dig_gold(bags: &Vec<ColorBag>, color: &str) -> usize {
    bags.iter()
        .find(|bag| bag.color == color)
        .and_then(|bag| {
                      Some(bag.contains.iter()
                              .fold(0, |total, innerbag| -> usize {
                                  total + innerbag.count + innerbag.count * dig_gold(&bags, &innerbag.color)
                               }))
                  })
         .unwrap_or_default()
}

fn main() -> Result<(), Error> {
    // new and improved way to load input files.
    // Don't know if it is faster, but it is waaay shorter and more comprehensible
    let mut input = String::new();
    let mut file = File::open("input.txt").expect("Could not read input.txt");
    file.read_to_string(&mut input).expect("Could not read input.txt");

    // start timing when the file is read in mem and we start to do stuff.
    let now = Instant::now();

    // need a place to store all our bags after parsing the textfile
    let mut baglist: Vec<ColorBag> = vec![];

    // There is al lot of ripped/copied code in this. Couldn't figure this out myself.
    // It's actually very hard to do string stuff in Rust, unless you totally understand it.
    // which I absolutely do not (yet).
    input.trim().split('\n').for_each(|x| {
                                let mut thisbag = ColorBag::default();
                                let mut x = x.split(|p| p == ' ' || p == ',' || p == '.');
                                let x = x.by_ref();
                                thisbag.color = x.take(2).collect::<Vec<&str>>().join(" ");
                                thisbag.count = 1usize;
                                assert_eq!(x.next(), Some("bags"));
                                assert_eq!(x.next(), Some("contain"));
                                loop {
                                    let ccount: usize = match x.next() {
                                        Some("no") => break,
                                        Some("bag") => continue,
                                        Some("bags") => continue,
                                        Some("") => continue,
                                        Some(count) => count.parse::<usize>().unwrap_or_default(),
                                        None => break,
                                    };
                                    thisbag.contains.push(ColorBag{ 
                                                              color: x.take(2).collect::<Vec<&str>>().join(" "),
                                                              count: ccount,
                                                              contains: vec![],
                                                          });
                                }
                                baglist.push(thisbag);
                            });

    println!("Part One: total bags which can contain at least one shiny gold bag: {}", check_gold(&baglist, "shiny gold").len());
    
    println!("Part Two: total bags needed for a shiny gold bag is: {}", dig_gold(&baglist, "shiny gold"));
    
    println!("It took me: {}ms", now.elapsed().as_millis());

    // Me main, me always happy
    Ok(())
}
