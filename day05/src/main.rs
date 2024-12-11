use std::fs;
use std::cmp;
use regex::Regex;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let contents = load_file("./input.txt");
    //let seeds = extract_seeds(contents.as_str());

    // Uncomment for Q2
    // Need to find a way to backwards engineer the shortest length without unraveling the full range
    // of seeds into memory
    // Maybe we can work backwards. Work from the smallest location until we get a "hit"
    let mut seed_ranges:Vec<Range>  = extract_seeds_q2(contents.as_str()).iter().map(|(seed_num, len)| Range::new(*seed_num, *len + *seed_num)).collect::<Vec<Range>>();
    seed_ranges.sort_by(|a, b| a.start.partial_cmp(&b.end).unwrap());
    let mut seeds = State::new(seed_ranges);
    let maps: Vec<Map> = extract_maps(contents.as_str());
    
    let goal_state = "location";
    while seeds.state != goal_state {
        seeds.update(&maps);
    } 
    println!("{:#?}", seeds);

    let min_value = seeds.ranges.iter().map(|r| r.start).min().expect("No minimum found");
    println!("{:#?}", min_value);
}

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Range {
            start,
            end
        }
    }
    
    pub fn map_range(&self, maps: &Map) -> Vec<Range> {
        let mut checkpoints: Vec<(i64, IntervalType)> = Vec::new();
        // Add the start and end index
        checkpoints.push((self.start, IntervalType::Identity));
        checkpoints.push((self.end, IntervalType::Identity));
        // Add the points in the MapRange's to `checkpoints`
        maps.codes.iter().filter(|(r_map, _)|
            !((r_map.end < self.start) | (r_map.start > self.end))
            ).for_each(|(r, start_index)| {
                checkpoints.push((r.start, IntervalType::Map(start_index - r.start)));
                checkpoints.push((r.end, IntervalType::Identity));
        });

        let mut checkpoints = checkpoints.iter().map(|(i, interval_type)|
            if *i < self.start {
                (self.start, interval_type.clone()) 
            } else {
                (*i, interval_type.clone())
            }
        ).filter(
            |(i, _)| (*i >= self.start) & (*i <= self.end)
        ).collect::<Vec<(i64, IntervalType)>>();

        checkpoints.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        
        checkpoints.iter().tuple_windows().map(|(a, b)| 
            match a.1 {
                IntervalType::Identity => Range::new(a.0, b.0),
                IntervalType::Map(offset) => Range::new(a.0 + offset, b.0 + offset),
            }
        ).filter(|r| r.start != r.end).collect()
        }
}

#[derive(Clone, Debug)]
enum IntervalType {
    Map(i64),
    Identity,
}


#[derive(Debug)]
struct State {
    state: String,
    ranges: Vec<Range>, 
}

impl State {
    pub fn new(ranges: Vec<Range>) -> Self {
        State {
            state: String::from("seed"),
            ranges,
        }
    }

    pub fn update(&mut self, map: &Vec<Map>) {
        // Get the value from the map
        let map = *map.iter().filter(|m| m.from == self.state).collect::<Vec<&Map>>().get(0).expect("Dimension of filtered map is 0");
        self.state = map.to.clone();
        let remapped_range: Vec<Range> = self.ranges.iter().map(|r| r.map_range(map)).flatten().collect();

        if remapped_range.len() != 0 {
            self.ranges = remapped_range
        }

    }
}

fn extract_seeds_q2(s: &str) -> Vec<(i64, i64)> {
    let re = Regex::new(r"seeds: ([\n\s\S]*?)\n\n").unwrap();
    let cap = re.captures(s).unwrap();
    let cap_vec = cap.get(1).unwrap().as_str().split(&[' ', '\n'][..]).map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    cap_vec.into_iter().batching(|it| {
           match it.next() {
               None => None,
               Some(x) => match it.next() {
                   None => None,
                   Some(y) => Some((x, y)),
               }
           }
       }).collect()
}

fn extract_seeds(s: &str) -> Vec<i64> {
    let re = Regex::new(r"seeds: ([\n\s\S]*?)\n\n").unwrap();
    let cap = re.captures(s).unwrap();
    cap.get(1).unwrap().as_str().split(&[' ', '\n'][..]).map(|s| s.parse::<i64>().unwrap()).collect()
}

/// Define Map struct
#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    codes: Vec<(Range, i64)>
}

impl Map {
    pub fn new(from: &str, to: &str, codes: Vec<(i64, i64, i64)>) -> Self {
        Map {
            from: from.to_string(),
            to: to.to_string(),
            codes: {
                let mut unsorted_codes = codes.iter().map(|(dest_start, input_start, length)| (Range::new(*input_start, *input_start + *length), *dest_start)).collect::<Vec<(Range, i64)>>();
                unsorted_codes.sort_by(|(a, _), (b, _)| a.start.partial_cmp(&b.end).unwrap());
                unsorted_codes
                }
            }
    }
}

fn extract_maps(s: &str) -> Vec<Map> {
    let re = Regex::new(r"(?<i>.*?)-to-(?<o>.*?) map:\n(?<c>[\n\s\S]*?)\n\n").unwrap();
    let mut maps = Vec::new();
    re.captures_iter(s).for_each(|c| {
        let input = c.name("i").unwrap().as_str();
        let output = c.name("o").unwrap().as_str();
        let codes: Vec<&str> = c.name("c").unwrap().as_str().split(&['\n'][..]).collect();    
        let structured_codes: Vec<(i64, i64, i64)> = codes.iter().map(|s| {
            let conversion_instructions: Vec<i64> = s.split(" ").map(|i| i.parse::<i64>().unwrap()).collect();
            (conversion_instructions[0], conversion_instructions[1], conversion_instructions[2])
        }).collect();
        maps.push(Map::new(input, output, structured_codes));
    });
    maps
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
