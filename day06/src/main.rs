use std::fs;
use itertools::zip;
use regex::Regex;

fn main() {
    let contents: String = load_file("./input.txt");
    let races: Vec<Race> = parse_races_kerned(contents);
    let num_successful_boats = races.iter().map(|r| Walker::new(1, r.time).num_successful_boats(r)).next().unwrap();
    println!("{}", num_successful_boats);
}

// Go forward and then backwards. That should give us the count. 
#[derive(Debug)]
struct Walker {
    start: i64,
    end: i64,
}

impl Walker {
    pub fn new(start: i64 , end: i64) -> Self {
        Walker {
            start,
            end
        }
    }
    
    fn walk_forward_greedy(&mut self, race: &Race) {
        while !race.finishes_race(self.start) {
            self.start += 1;
        }
    }
    
    fn walk_backwards_greedy(&mut self, race: &Race) {
        while !race.finishes_race(self.end) {
            self.end -= 1;
        }
    }

    pub fn num_successful_boats(&mut self, race: &Race) -> i64 {
        self.walk_forward_greedy(race);
        self.walk_backwards_greedy(race);
        self.end + 1 - self.start
    }
}

#[derive(Debug, Clone)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    pub fn new(time: i64, distance: i64) -> Self {
        Race {
            time, 
            distance
        }
    }
    
    pub fn finishes_race(&self, charging_time: i64) -> bool {
        // Don't travel any distance when you charge
        let time_left: i64 = self.time - charging_time; 
        // It's go time (NOTE: you have to beat the record)
        time_left * charging_time > self.distance
    }

}


fn parse_races_kerned(contents: String) -> Vec<Race> {
    let re: Regex = Regex::new(r"[a-zA-Z]*: *?([\n\s\S]*?)\n").unwrap();
    let mut unparsed_races = vec![];

    for (_, [times]) in re.captures_iter(contents.as_str()).map(|c| c.extract()) {
        let re: Regex = Regex::new(" ").unwrap();
        // Q2
        let unkerned_string = String::from(re.replace_all(times.trim(), ""));
        unparsed_races.push(vec![unkerned_string]);
    }

    let mut races = vec![];
    for (time, distance) in zip(&unparsed_races[0], &unparsed_races[1]) {
        races.push(Race::new(time.as_str().parse::<i64>().unwrap(), distance.as_str().parse::<i64>().unwrap()));
    }
    races
}

fn parse_races(contents: String) -> Vec<Race> {
    let re: Regex = Regex::new(r"[a-zA-Z]*: *?([\n\s\S]*?)\n").unwrap();
    let mut unparsed_races = vec![];

    for (_, [times]) in re.captures_iter(contents.as_str()).map(|c| c.extract()) {
        let re: Regex = Regex::new(" {2,}").unwrap();
        unparsed_races.push(re.split(times.trim()).collect::<Vec<&str>>());
    }

    let mut races = vec![];
    for (&time, &distance) in zip(&unparsed_races[0], &unparsed_races[1]) {
        races.push(Race::new(time.parse::<i64>().unwrap(), distance.parse::<i64>().unwrap()));
    }
    races
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
