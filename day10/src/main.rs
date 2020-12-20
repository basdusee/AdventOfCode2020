use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::time::Instant;
use std::collections::HashMap;

fn differences(adapters: &Vec<usize>) -> usize {
    // "stack" of value tracking vars
    // threejolts starts at 1 because our builtin adapter
    let mut onejolts: usize = 0;
    let mut threejolts: usize = 1;

    // need to rate the first adapter
    // it's "adapter jolts" - 0
    match adapters[0] {
            1 => onejolts += 1,
            3 => threejolts += 1,
            _ => println!("found weird first number"),
    }

    // for every consecutive pairs, just match on the difference.
    // the list is sorted, so this should work out.
    for cursor in 1..adapters.len() {
        match adapters[cursor] - adapters[cursor-1]  {
            1 => onejolts += 1,
            3 => threejolts += 1,
            _ => println!("found weird difference"),
        }
    }
    onejolts * threejolts
}

// this was my first attempt at part 2. It works on the test data
// but was waaaaaaaaaay too slow for the real dataset, as in never got there.
// It's a really simple recursive thingy, just do three recursive calls unless the index 
// is not in the dataset (return 0 then), or if this is the last (max) number in the dataset. 
#[allow(unused)]
fn combinations_recursive(adapters: &Vec<usize>, index: usize) -> usize {
    if !adapters.contains(&index) {
        return 0;
    }
    if index ==  *adapters.iter().max().unwrap() {
        return 1;
    }
    combinations_recursive(&adapters, index + 1) +
    combinations_recursive(&adapters, index + 2) +
    combinations_recursive(&adapters, index + 3)
}

// So I crashed and burned. did some peeping at solutions, which didn't help much
// but hinted me at things called "memoization" (not a spelling error) and
// "dynamic programming" which sounds really good.

// After some Youtubing on memoization I created the following code.
// and it did find the solution in ~1ms from unoptimized (debug) code...wow. memoization works...
// with memoization you basically cache the intermediate results.
// This is called "top down dynamic programming" or something fancy like that.
#[allow(unused)]
struct Memoize<'a> {
    adapters: &'a Vec<usize>,
    cache: &'a mut HashMap<usize, usize>,
    max: usize,
}

fn combinations_memoize(adaptstruct: &mut Memoize, index: usize) -> usize {
    // if this index is not in the dataset at all, just give back zero
    if !adaptstruct.adapters.contains(&index) {
        return 0;
    }
    // if this is the last number, just give back one last path.
    if index == adaptstruct.max {
        return 1;
    }
    // if this index is not in cache, fire away three recursive calls
    // and cache the result
    if !adaptstruct.cache.contains_key(&index) {
        let ans = combinations_memoize(adaptstruct, index + 1) +
                  combinations_memoize(adaptstruct, index + 2) +
                  combinations_memoize(adaptstruct, index + 3);
        adaptstruct.cache.insert(index, ans);
    }
    // if it is in cache, just feast yourself on the juicyness of cached results
    adaptstruct.cache[&index] 
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
    let mut adapters: Vec<usize> = input.trim().split('\n')
                                    .map(|x| x.parse::<usize>().unwrap_or_default()) 
                                    .collect::<Vec<usize>>();


    adapters.sort_unstable();
    
    println!("Part One: Onejolts and Threejolts counts multiplied together is: {}", differences(&adapters));

    println!("It took me: {} microseconds", now.elapsed().as_micros());

    // reset timing for part two.
    let now = Instant::now();
 
    // insert the "outlet" and the "laptop adapter" to the input list                             
    adapters.push(0);
    adapters.push(*adapters.iter().max().unwrap() + 3);
    adapters.sort(); /* adapters is almost sorted, so stable timsort-ish is probably faster than unstable qicksort*/

    let mut adapter_cache: HashMap<usize, usize> = HashMap::new();

    // fill in the struct before the run
    let mut adaptstruct = Memoize { 
        adapters: &adapters, 
        cache: &mut adapter_cache,
        max: *adapters.iter().max().unwrap(),
    };
    
    println!("Part Two: number of adapter combinations is {}", combinations_memoize(&mut adaptstruct, 0));

    println!("It took me: {} microseconds", now.elapsed().as_micros());

    // Me main, me always happy
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn parttwo_recursive_small_testset() {
        let mut testdata: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        // insert the "outlet" and the "laptop adapter" to the input list                             
        testdata.push(0);
        testdata.push(*testdata.iter().max().unwrap() + 3);

        testdata.sort_unstable();

        assert_eq!(combinations_recursive(&testdata, 0), 8);
    }

    #[test]
    fn parttwo_recursive_big_testset() {
        let mut testdata: Vec<usize> = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];

        // insert the "outlet" and the "laptop adapter" to the input list                             
        testdata.push(0);
        testdata.push(*testdata.iter().max().unwrap() + 3);

        testdata.sort_unstable();

        assert_eq!(combinations_recursive(&testdata, 0), 19208);
    }

    #[test]
    fn parttwo_memoize_small_testset() {
        let mut testdata: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        // insert the "outlet" and the "laptop adapter" to the input list                             
        testdata.push(0);
        testdata.push(*testdata.iter().max().unwrap() + 3);

        testdata.sort_unstable();
        
        let mut cache: HashMap<usize, usize> = HashMap::new();
        
        let mut teststruct = Memoize { 
            adapters: &testdata, 
            cache: &mut cache,
            max: *testdata.iter().max().unwrap(),
        };

        assert_eq!(combinations_memoize(&mut teststruct, 0), 8);
    }

    #[test]
    fn parttwo_memoize_big_testset() {
        let mut testdata: Vec<usize> = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];

        // insert the "outlet" and the "laptop adapter" to the input list                             
        testdata.push(0);
        testdata.push(*testdata.iter().max().unwrap() + 3);

        testdata.sort_unstable();
        
        let mut cache: HashMap<usize, usize> = HashMap::new();
        
        let mut teststruct = Memoize { 
            adapters: &testdata, 
            cache: &mut cache,
            max: *testdata.iter().max().unwrap(),
        };

        assert_eq!(combinations_memoize(&mut teststruct, 0), 19208);
    }

}

