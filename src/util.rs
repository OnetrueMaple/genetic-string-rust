#![allow(non_upper_case_globals, dead_code)] // Allows me to ignore specific compiler warnings. There are more options, run the terminal command 'rustc -W help' for more info

use std::fs::File; // Prevents the need to type std::fs::File and std::fs::Read::read_exact when calling module specific functions
use std::io::Read;

pub const all_characters: [char; 93] = [ // 'pub' means access outside file, 'const' means the variable cannot change at runtime, 'all...' var_name, ': [char; 91]' array of 91 'char' datatypes
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', ' ', '?', '.', ',', '<', '>', ':', ';', '/', '{', '}', '[', ']', '|', '-', '_', '+', '=', '`', '~', '\''
];

pub fn random(bound: u8) -> u8 { // 'pub' means public (access outside file), 'fn' means function declaration, 'bound: u8' var_name: datatype, '-> u8' means returns u8 datatype
    let mut f                 = File::open("/dev/urandom").expect("The file could not be opened!"); // Have to explicitly handle the possibility of failing to open the file
    let mut bytes:    [u8; 1] = [0];
    let actual_bound: u8      = u8::MAX - (u8::MAX % bound);

    f.read_exact(&mut bytes).expect("Could not read bytes from urandom!"); // Stores 1 byte to turn into a random number, u8. The number of bytes stored is determined by the size of array.

    loop {                                                                 // Forever loop, exits only when returned or broken out of.
        if bytes[0] < actual_bound {
            return bytes[0] % bound;
        }
        f.read_exact(&mut bytes).expect("Could not read bytes from urandom!");
    }

}

/// ----------------------------------------------------------|
///         RANDOM STARTER BULLSHIT                           |
/// ----------------------------------------------------------|


fn test() { // Function definition
    println!("Hello, world!"); // Print function,'!' means it's a macro for the compiler.
    add();
    let num = random(255);
    println!("Random number is '{}'", num);
}

fn add() { // Adds two numbers, also shows print function usage with variables
    let num1 = 2;
    let num2 = 5;
    println!("Sum of num1: {}, and num2: {}, are {}!", num1, num2, (num1 + num2));
}