use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path: String = String::from("./input.txt");
    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let contents = contents.as_str().split("\n");
    let mut numbers: Vec<u32> = vec![];
    
// Part I: Just parsing digits
// for text in contents {
// 
// let calibration_value: u32 = get_calibration_values(text, text);
//        numbers.push(calibration_value);
//    }
//    println!("The total is {}", numbers.iter().sum::<u32>());
   
    // Part II: Parsing string representation of digits
    for text in contents {
        let calibration_value: u32 = get_calibration_values(&convert_text_digit_to_numeric_digit(text, false), &convert_text_digit_to_numeric_digit(text, true));
    numbers.push(calibration_value);
    }

    println!("The total is {}", numbers.iter().sum::<u32>());
   
}

fn convert_text_digit_to_numeric_digit(text: &str, reverse: bool) -> String {
    // Define the digit dictionary
    let digits: HashMap<&str, char> = if !reverse {
    HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'), 
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ])} else {
    HashMap::from([
        ("eno", '1'),
        ("owt", '2'),
        ("eerht", '3'),
        ("ruof", '4'),
        ("evif", '5'),
        ("xis", '6'), 
        ("neves", '7'),
        ("thgie", '8'),
        ("enin", '9')
    ])
    };
    let text_to_string: String  = {
        if reverse {
            text.chars().rev().collect::<String>()
        } else {
            String::from(text)
        }
    };
    let mut builder_string: String = String::new();
    let mut text_as_chars = text_to_string.as_str().chars();
    let mut num_remaining = text.len(); 
    while num_remaining != 0 { 
        let found_digit: Option<&str> = digits.clone().into_keys().find(|x| text_as_chars.as_str().starts_with(*x));
        match found_digit {
            Some(entry) => {
                // Add the translated digit to the builder string
                builder_string.push(*digits.get::<str>(entry).expect("The digit did not have a corresponding character representation"));
                // Advance the iterator by the length of the matched digit
                let n: usize = entry.len() as usize;
                text_as_chars.nth(n - 1);
                num_remaining -= n;
            },
            None => {
                builder_string.push(text_as_chars.next().expect("Failed to iterate through iter"));
                num_remaining -= 1;
            },
        }
    }

    if reverse {
        builder_string.as_str().chars().rev().collect::<String>()
    } else {
        builder_string
    }
    
}

fn find_first_digit(text: &str, reverse: bool) -> char { 
    let text_to_string: String  = {
        if reverse {
            text.chars().rev().collect::<String>()
        } else {
            String::from(text)
        }
    };
    let text_as_chars = text_to_string.as_str().chars();

    for character in text_as_chars {
        if character.is_numeric() {
            return character;
        }
    }
    '_'    
}

fn get_calibration_values(text_forward_parse: &str, text_backward_parse: &str) -> u32 {
    let tens_digit: u32 = find_first_digit(text_forward_parse, false).to_digit(10).expect("Failed to parse digit") * 10;
    let ones_digit: u32 = find_first_digit(text_backward_parse, true).to_digit(10).expect("Failed to parse digit");

    tens_digit + ones_digit
}

#[cfg(test)]
mod tests {
use crate::find_first_digit;
use crate::convert_text_digit_to_numeric_digit;

    #[test]
    fn no_digit_case() {
        let no_digit_str: &str = "hello world";
        assert_eq!(find_first_digit(no_digit_str, true), '_');

        assert_eq!(find_first_digit(no_digit_str, false), '_');
    }

    #[test]
    fn one_digit_case() {
        let one_digit_str: &str = "hello1world";
        assert_eq!(find_first_digit(one_digit_str, true), '1');

        assert_eq!(find_first_digit(one_digit_str, false), '1');
    }

    #[test]
    fn two_digit_case() {
        let two_digit_str: &str = "2hello1world";
        assert_eq!(find_first_digit(two_digit_str, true), '1');

        assert_eq!(find_first_digit(two_digit_str, false), '2');
    }

    #[test]
    fn several_digit_case() {
        let several_digit_str: &str = "h3llo1wor1d";
        assert_eq!(find_first_digit(several_digit_str, true), '1');

        assert_eq!(find_first_digit(several_digit_str, false), '3');
    }

    #[test]
    fn convert_digits_to_numbers() {
       assert_eq!(convert_text_digit_to_numeric_digit("two1nine", true), "219");

       assert_eq!(convert_text_digit_to_numeric_digit("eighttwothree", true), "823");

       assert_eq!(convert_text_digit_to_numeric_digit("abcone2threexyz", true), "abc123xyz");

       assert_eq!(convert_text_digit_to_numeric_digit("xtwone3four", true), "xtw134");

       assert_eq!(convert_text_digit_to_numeric_digit("4nineeightseven22", true), "498722");

       assert_eq!(convert_text_digit_to_numeric_digit("zoneight234", true), "zon8234");
           
       assert_eq!(convert_text_digit_to_numeric_digit("zoneight234", false), "z1ight234");
       
       assert_eq!(convert_text_digit_to_numeric_digit("7pqrstsixteen", false), "7pqrst6teen");
        

    }        
}


