use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let contents: String = load_file("./input.txt");
    let mut hands: Vec<Hand> = contents.split("\n").map(|row| {
        let row = row.split(" ").collect::<Vec<&str>>();
        Hand::new(row[1].parse::<i64>().unwrap(), row[0])
    }
    ).collect::<Vec<Hand>>();
    // Note that .sort() sorts least to greatest
    hands.sort();
    println!("{:#?}", hands);
    let total_winnings = hands.iter().enumerate().map(|(i, h)| {
        h.bid * ((i as i64) + 1)
    }).sum::<i64>();
    println!("{}", total_winnings);
}


#[derive(Debug, PartialEq, Eq)]
struct Hand {
    bid: i64,
    hand: String,
}

impl Hand {
    pub fn new(bid: i64, hand: &str) -> Self {
        Hand { 
            bid,
            hand: hand.to_string(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       Some(self.cmp(other)) 
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Need to implement the ordering here
        let mut processed_hand: HashMap<char, i8> = HashMap::new();
        let mut other_processed_hand: HashMap<char, i8> = HashMap::new();
        let card_values: HashMap<char, i8> = HashMap::from([
            ('2', 2),
            ('3', 3),
            ('4', 4), 
            ('5', 5), 
            ('6', 6), 
            ('7', 7), 
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('J', 1),
            ('Q', 12),
            ('K', 13), 
            ('A', 14),
        ]);
        self.hand.as_str().chars().for_each(|c| {processed_hand.entry(c).and_modify(|count| *count += 1).or_insert(1);});
        other.hand.as_str().chars().for_each(|c| {other_processed_hand.entry(c).and_modify(|count| *count += 1).or_insert(1);});
        
        // Q2: If J exists, we subtract 1 from the length since it is "absorbed"
        let adjustment: usize = match processed_hand.get(&'J') {
            Some(5) => 0,
            Some(_) => 1,
            None => 0,
        };
        let other_adjustment: usize= match other_processed_hand.get(&'J') {
            Some(5) => 0,
            Some(_) => 1,
            None => 0,
        };
        let processed_hand_len = processed_hand.values().len() - adjustment;
        let other_processed_hand_len = other_processed_hand.values().len() - other_adjustment;
        if processed_hand_len != other_processed_hand_len {
            // If the number of values of self.hand is less than other.hand, then it is cmp::GREATER
            other_processed_hand_len.cmp(&processed_hand_len)        
        } else {
            // If they number of values is equal, we order by the max count of self.hand and
            // other.hand
            // Q2
            let adjustment = match processed_hand.get(&'J') {
                Some(&x) => x,
                None => 0 as i8,
            };
            let other_adjustment = match other_processed_hand.get(&'J') {
                Some(&x) => x,
                None => 0 as i8,
            };
            // Need to remove 'J' to prevent overcounting
            processed_hand.remove(&'J');
            other_processed_hand.remove(&'J');
            // If J exists in the HashMap we just add it to the maxes
            let max_processed_hand = *processed_hand.values().max().unwrap_or(&0) + adjustment;
            let max_other_processed_hand = *other_processed_hand.values().max().unwrap_or(&0) + other_adjustment;
            if max_processed_hand != max_other_processed_hand {
                max_processed_hand.cmp(&max_other_processed_hand) 
            } else {
                // convert card char to card rank and then compare.
                self.hand.as_str().chars().map(|c| *card_values.get(&c).unwrap()).partial_cmp(other.hand.as_str().chars().map(|c| *card_values.get(&c).unwrap())).unwrap()
            }
        }
    }
}

fn load_file(path: &str) -> String {
    // Load data from .txt format
    let file_path: String = String::from(path);
    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents
}

