use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn main() {
    // Load data from .txt format
    let contents = load_file("./input.txt");
    let contents = contents.as_str().split("\n");
    // Iterate through the contents (games in this case)
    let mut total_pt1: i32 = 0;
    let mut total_pt2: i32 = 0;
    for game in contents {
        let (game_number, game_info) = parse_game(game);
        // Finds whether the number of cubes is appropriate
        let sets_of_cubes = game_info.split(";");
        // Part 1
        let keep_game = sets_of_cubes.clone().into_iter().all(|set_of_cubes| appropriate_num_cubes(set_of_cubes));
        if keep_game {
            total_pt1 += game_number;
        }
        
        // Part 2
        let sets_of_cubes = sets_of_cubes.into_iter().map(|set_of_cubes| count_cubes(set_of_cubes)).collect();
        let power_of_game = minimum_required_colors(sets_of_cubes).values().into_iter().product::<i32>();

        total_pt2 += power_of_game;
    }
    println!("The total of the appropriate game numbers: {}", total_pt1);
    println!("The total of the game powers: {}", total_pt2);
}

// Function name: minimum_required_colors
// Input: sets_of_cubes, a vector of HashMap
// Output: a HashMap that has the minimum values across all of the keys
fn minimum_required_colors(sets_of_cubes: Vec<HashMap<&str,i32>>) -> HashMap<&str, i32> {
    let mut color_counts: HashMap<&str, i32> = HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0),
    ]);
    for set_of_cubes in sets_of_cubes.iter() {
        for (key, x) in color_counts.iter_mut() {
            let current_count = *set_of_cubes.get(key).unwrap();
            if current_count >= *x {
                *x = current_count;
            }
        }
    }
    color_counts
}

// function name: parse_game
// input: a string containing game informatiojn
// output: a tuple with the game number and game information
fn parse_game(game_str: &str) -> (i32, &str) {
    // extract the game number 
    let re = regex::new(r"game ([0-9]*)\:(.*)").unwrap();
    let game_number = re.captures(game_str).expect("failed to parse regex pattern").get(1).unwrap().as_str().parse::<i32>().expect("failed to convert &str to i32");

    // Parse the game_string further.
    let game_info = re.captures(game_str).expect("Failed to parse regex pattern").get(2).unwrap().as_str();

    (game_number, game_info)
}

// Function name: count_cubes
// Input: a string containing all game information
// Output: a HashMap aggregating all colors and corresponding values per set
fn count_cubes(game_info: &str) -> HashMap<&str, i32> {
    // initalize the color_counts dictionary 
    let mut color_counts: HashMap<&str, i32> = HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0),
    ]);
    // First, split the set_of_cubes by comma and make that into an iter
    let set_of_cubes = game_info.split(",").into_iter();
    
    // Per item, parse the appropriate color and return that
    for cube in set_of_cubes { 
        // create re pattern to extract the number of cubes of said color
        let re = Regex::new(r"([0-9]*) ([a-z]*)").unwrap();

        let count = re.captures(cube.trim()).expect("Failed to parse regex pattern").get(1).unwrap().as_str().parse::<i32>().expect("Failed to convert &str to i32");
        let color = re.captures(cube.trim()).expect("Failed to parse regex pattern").get(2).unwrap().as_str();
        if let Some(x) = color_counts.get_mut(color) {
            *x = *x + count;
        }
    }
    color_counts
}

// Function name: appropriate_num_cubes 
// Input: game_info: a &str with information about each round of the game
// Output: true or false for whether the game has the appropriate number of cubes
fn appropriate_num_cubes(game_info: &str) -> bool {
    // the specified colors that we are interested in
    let max_colors: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);    
    let color_counts: HashMap<&str, i32> = count_cubes(game_info);
    // Compare the colors dictionary with the constructed colors
    for color in max_colors.keys() {
        if color_counts.get(color) > max_colors.get(color) {
            return false;
        }
    }
    return true;
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

