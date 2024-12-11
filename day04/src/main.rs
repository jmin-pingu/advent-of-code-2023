use std::fs;
use std::cmp;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let contents = load_file("./input.txt");
    let contents = contents.split("\n");
    
    let re = Regex::new(r"Card *?([0-9]*):(.*?)\| (.*)").expect("Failed to parse regex expr");
    let re_whitespace = Regex::new(r" {2,}").expect("Failed to parse regex expr");
    //let mut total = 0;
    let mut cards: Vec<Card> = vec![]; 
    //const FACTOR: i32 = 2;
    for game in contents {
        let game_contents = re.captures(game).expect("Failed to parse game_contents");
        let game_number = game_contents.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let winning_numbers: HashSet<i32> = re_whitespace.replace_all(game_contents.get(2).unwrap().as_str().trim(), " ").split(" ").into_iter().map(|n| n.parse::<i32>().unwrap()).collect();
        let player_numbers: HashSet<i32>= re_whitespace.replace_all(game_contents.get(3).unwrap().as_str().trim(), " ").split(" ").into_iter().map(|n| n.parse::<i32>().unwrap()).collect();
        //let multiplier = cmp::max(0, winning_numbers.intersection(&player_numbers).collect::<HashSet<&i32>>().len()) as u32;
     //   let points = if multiplier == 0 {
     //       0
     //   } else {
     //       FACTOR.pow(multiplier - 1)
     //   };
        
     //   total += points; 
        let num_wins = cmp::max(0, winning_numbers.intersection(&player_numbers).collect::<HashSet<&i32>>().len()) as u32;
        cards.push(Card::new(game_number as usize, num_wins as usize)) 
    }
    println!("The list of cards is {:#?}", cards);
    // Should use some sort of recursion

    let output: Vec<i32> = cards.iter().map(|c| {
        1 + count_wins(c, &cards)
    }).collect();
    println!("output {:#?}", output);
    println!("output {:#?}", output.into_iter().sum::<i32>());
}

fn count_wins(current_card: &Card, cards: &Vec<Card>) -> i32 {
    let index = current_card.card_number;
    let upper_bound = current_card.card_number + current_card.num_wins; 
    if index < upper_bound {
        cards[index..upper_bound].iter().map(|c| 1 + count_wins(c , cards)).sum()
    } else {
        0
    }
}
// want to recursively get the answer. 
// The input should be a Vec of winning_numbers and player_numbers and an index
#[derive(Debug)]
struct Card {
    card_number: usize,
    num_wins: usize,
}

impl Card {
    pub fn new(card_number: usize, num_wins: usize) -> Self {
        Card {
            card_number,
            num_wins,
        }
    }
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

