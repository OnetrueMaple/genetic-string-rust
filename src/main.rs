mod util;           // Include a file in the same directory

use util::{random, all_characters}; // Avoid requiring util::random() every time you wish to call random

fn score(string: &str, target: &str) -> isize {
    let char_list = string.as_bytes();    // Must convert between string object and bytes for comparison: strings are not iterable directly.
    let ideal = target.as_bytes();        //    Same as above ^
    let mut points: isize = 0;
    if string.len() > target.len() {      // string.len - target.len certainly positive, points must start off negative...
        points -= (string.len() as isize) - (target.len() as isize);
        for i in 0..target.len() {
            if char_list[i as usize] == ideal[i as usize] {
                points += 1;              // Add points if the characters at the same indice of each string match.
            }
        }
    }
    else if string.len() < target.len() { // target.len - string.len certainly positive, points must start off negative...
        points -= (target.len() as isize) - (string.len() as isize);
        for i in 0..string.len() {
            if char_list[i as usize] == ideal[i as usize] {
                points += 1;              // Add points if the characters at the same indice of each string match.
            }
        }
    }
    else {                                // string.len and target.len must be equal, points can start at 0.
        for i in 0..target.len() {
            if char_list[i as usize] == ideal[i as usize] {
                points += 1;              // Add points if the characters at the same indice of each string match.
            }
        }
    }
    return points;
}

fn score_all(list: &Vec<String>, target: &str, debug: bool) -> usize {
    let       count:      usize      = list.len();
    let mut   scores:     Vec<isize> = Vec::new();
    let mut   highest_id: usize      = 0;

    if debug == true { println!("target: '{}'", target); }

    for i in 0..count {
        let curr_score: isize = score(&list[i], target);
        scores.push(curr_score);
        if curr_score >= scores[highest_id] {
            highest_id = i;
        }
    }

    if debug == true { println!("Scores: {:?}, Highest_ID: {}", scores, highest_id); }

    return highest_id;
}

fn mutate(string: &str, count: usize, debug: bool) -> Vec<String> {
    let mut strings:      Vec<String> = Vec::with_capacity(count);
    let     str_copy:     String      = string.to_string();

    if debug == true { println!("str_copy before: '{}'", str_copy); }

    strings.push(str_copy);
    for _i in 0..(count - 1) {
        // Gather all necessary data per new string
        let mut str_copy: String      = string.to_string();
        let     function: usize       = random(3u8) as usize;
        // If I was better, I should actually have this function return a Result<> since this vv line could fail if 'string' is more than 256 characters.
        let     index:    usize       = random(string.len() as u8) as usize;
        let     new_char: char        = all_characters[random(93u8) as usize];

        // Add random character in random location
        if function == 0 {
            str_copy.insert(index, new_char);
        }
        // Remove random character (random location)
        else if function == 1 {
            str_copy.remove(index);
        }
        // Change random character (location) to a random character
        else {
            str_copy.remove(index);
            str_copy.insert(index, new_char);
        }

        //println!("str_copy after:  '{}', function: '{}', index: '{}', new_char: '{}'", str_copy, function, index, new_char);
        strings.push(str_copy);
    }

    if debug == true { println!("strings: {:?}", strings); }

    return strings;
}

fn main() {
    // Peep my convention for variable declaration vvv. "const/let 'mut' 'var_name' 'data-type' = 'data'" The amount of spaces varies by the largest char string per category, leave one space between.
    // I'll leave it this way since this will show me where I'm unaware of details. Ideally, I fill in every gap with something, if I am unable to fill a gap I will not leave information in the spot.
    const   HARDCODE:      bool   = false;
    const   PRINT_DEBUG:   bool   = false;
    let     terminal_input        = std::io::stdin(); // Let the user decide a string to find...          || The absence of 'mut' and 'const' implies 'const'
    let mut target:        String = String::new();    // Stores the string the user wishes to have found...
    let     gen_size:      usize  = 100;                // Let the user decide how many strings should be generated per new generation.

    if HARDCODE != true {
        println!("What string should the genetic algorithm find?: ");
        terminal_input.read_line(&mut target).expect("Could not read input from a terminal! Exiting...");
        target.remove(target.len() - 1usize);
    } else {
        target.push_str("what the fuck man"); // I'm gonna hardcode one for testing purposes.
    }

    let mut generation:    Vec<String> = Vec::new();
    // Initializing the 'generation' with random garbage...
    for _i in 0..gen_size {
        let mut temp_str:  String   = String::new();
        // '15 * 17' (255) is the biggest number within the range of 2^8 (255), since I declared the function takes an 8-bit number as input. I want it to throw away as few bits as possible, so I made the number
        //      large without skewing the odds... that's also why it's necessary to '% 15'. If the number generated goes over 15, I need it to get back into that range and returning remainder of division does that.
        let     num_chars: usize    = random(15) as usize;
        for _j in 0..num_chars {
            temp_str.push(all_characters[random(93) as usize]);
        }
        generation.push(temp_str);
    }
    let mut highest_score: usize    = score_all(&generation, &target, PRINT_DEBUG); // Stores the index of the highest scoring string found in 'generation'

    let mut attempts:      usize    = 1; // Counter for knowing how long it took the program to find the string.
    while score(&generation[highest_score], &target) != (target.len() as isize) {
        attempts += 1;
        generation    = mutate(&generation[highest_score], gen_size, PRINT_DEBUG);
        highest_score = score_all(&generation, &target, PRINT_DEBUG);
        println!("Closest string:   '{}'\n-----------------------", generation[highest_score]);
    }

    println!("Found the string: '{}' after {} generations!", generation[highest_score], attempts);

}
