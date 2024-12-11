use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let contents: String = load_file("./input.txt");

    let mut network_map: HashMap<&str, Network> = HashMap::new();
    let re: Regex = Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();
    re.captures_iter(contents.as_str()).map(|c| c.extract()).for_each(|(_, [key, left, right])| { network_map.insert(key, Network::new(left, right)); }
    );
    let re: Regex = Regex::new(r"([LR]*)\n").unwrap();
    let instructions = re.captures(contents.as_str()).unwrap().get(1).unwrap().as_str().chars().collect::<Vec<char>>();

    // Also need to change our input into a vector/array of all instructions with `A` at the end
    // We can do this by going over network_map.keys()
    //let mut input = String::from("AAA");

    // Q2
    let mut inputs = network_map.keys().filter(|&&k| k.ends_with("A")).map(|v| *v).map(|v| v.to_string()).collect::<Vec<String>>();
   
    // Q2: need to change the stopping condition
    // Use the group theory lmao. Symmetric group I'm goated
    let min_steps_to_goal = inputs.iter_mut().map(|input| {
        let mut counter = 0;
        let mut i: usize = 0;
        while !input.ends_with("Z") {
            if i == instructions.len() {
                i = 0;
            }
            
            let network = network_map.get(input.as_str()).expect("Could not find input in network_map");
            *input = network.read_instruction(instructions[i]).unwrap(); 
            
            counter += 1;
            i += 1;
        }
        counter
    }).collect::<Vec<usize>>();
    println!("{:#?}", lcm(&min_steps_to_goal[..]));
    
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn load_file(path: &str) -> String {
    // Load data from .txt format
    let file_path: String = String::from(path);
    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents
}

#[derive(Debug)]
struct Network {
    left: String,
    right: String,
}

impl Network {
    pub fn new(left: &str, right: &str) -> Self {
        Network {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
    pub fn read_instruction(&self, instruction: char) -> Result<String, &'static str> {
        if instruction == 'L' {
            Ok(self.left.to_string())
        } else if instruction == 'R' {
            Ok(self.right.to_string())
        } else {
            Err("Instruction not `L` or `R`")
        }
    }
}
