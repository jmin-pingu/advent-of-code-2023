#![allow(dead_code)]
use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::cmp;
// INSTRUCTIONS
// Count all numbers of parts. Any number adjacent to a symbol (even diagonally)
// is a part number (include it into the sum). Periods don't count as symbols
//
// IDEAS
// Define two structs/objects
// ContextWindow + Parts
// For a given ContextWindow, there is the mainline, topline, bottomline
// We can let topline and bottomline be Option<&str>
// For each ContextWindow, there are several parts with appropriate values
//
// Example:
// ..........
// .###......
// .#*#..###.
// .###..#@#.
// ......###.
// ..........

// I implemented this inefficiently by putting the position data in each of the parts and positions
// If I used "metadata" for lines or context windows, that could reduce the number of iterations
// I guess there is a trade-off here. My data structures would be larger if I used a context
// window, but parsing through/filtering on this data structure would be much faster than iterating
// through all of the Count objects.
fn main() {
    // Read in input.txt    
    let contents = load_file("/Users/ymin/Downloads/AdventOfCode/AdventOfCodeDay3/input.txt");
    let contents: Vec<&str> = contents.split("\n").collect();   
    
    let mut parts: Vec<Part> = contents.iter().enumerate().map(|(i, c)| find_parts(c, i)).flatten().collect();

    let counts: Vec<Count> = contents.iter().enumerate().map(|(i, c)| find_counts(c, i)).flatten().collect();

    let parts: Vec<&mut Part> = parts.iter_mut().map(|p| {
        counts.iter().for_each(|c| check_count(p, c));
        p
    }).collect();

    let mut total = 0;
    parts.iter().filter(|p| p.num_adjacent == 2 && p.part == '*').for_each(|p| {
        total += p.count; 
    });
    println!("Parts are :{:#?}", parts);
    println!("Total parts {:#?}", total);
}

fn check_count(part: &mut Part, count: &Count) {
    if count.bound.upper >= part.position.x - 1 && count.bound.lower <= part.position.x + 1 && count.y_position <= part.position.y + 1 && count.y_position >= part.position.y - 1{
        if part.count != 0 {
            part.count *= count.count;
        } else {
            part.count += count.count;
        }
        part.num_adjacent += 1;
    }
}

// Later on, use a match to go through the long vector of parts and convert the values to
// a HashMap
#[derive(Debug)]
struct Point {
    x: usize, 
    y: usize,
}

#[derive(Debug)]
struct Bound {
    upper: usize,
    lower:usize,
}

impl Bound {
    pub fn contains(&self, digit: usize) -> bool {
        self.upper >= digit && self.lower <= digit
    }
}

#[derive(Debug)]
struct Part {
    part: char,
    position: Point,
    count: i32,
    num_adjacent: usize
}

#[derive(Debug)]
struct Count {
    count: i32,
    bound: Bound,
    y_position: usize,
}


fn find_counts(s1: &str, line_number: usize) -> Vec<Count> {
    let re = Regex::new(r"([0-9]*)").unwrap();
    // Get the position and corresponding "index" and then 
    re.find_iter(s1).filter(|digits| digits.as_str() != "").map(
        |digits| 
        Count { 
            count: digits.as_str().parse::<i32>().expect("Failed to convert &str to usize"), 
            bound: Bound {upper: digits.end() - 1, lower: digits.start() },
            y_position: line_number
        }
        ).collect()
}
   
fn find_parts(s1: &str, line_number: usize) -> Vec<Part> {
    let re = Regex::new(r"[^0-9\.]").unwrap();
    // Get the position and corresponding "index" and then 
    re.find_iter(s1).map(
        |m| Part { 
                            part: m.as_str().chars().collect::<Vec<char>>()[0], 
                            position: Point {x: m.start(), y: line_number}, 
                            count: 0,
                            num_adjacent: 0,
                        }
    ).collect()
}   

// load_file
// input: 
//  -path: a &str that is the path to the file that we want to parse
// output: the loaded contents of the file that is parsed
fn load_file(path: &str) -> String {
    // Load data from .txt format
    let file_path: String = String::from(path);
    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents
}

