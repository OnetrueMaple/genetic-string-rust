## Genetic String Algorithm
A program that will find a string passed by the user through random brute force after a minimal selection bias.
I made this to help me learn Rust, it is a recent project of mine.

## Features
    * Allows the user to set the string
    * Contains a simple PRNG with real entropy (time variance)
    * Only uses the standard library

## Technologies
    * Rust for learning
    * Oversimplified ML algorithm to score and modify strings of text.

## Usage
If you have Rust installed on your computer, clone the repo using 'git clone https://github.com/OnetrueMaple/genetic-string-rust' then run 'cargo run'.
Once you enter a string, the program will build it from a random string.

## Preview
```
user@User:~$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/genetic-string`
What string should the genetic algorithm find?:
any
Closest string:   'lfA'
-----------------------
Closest string:   'lfA'
-----------------------
Closest string:   'jfA'
-----------------------
Closest string:   'jf/'
-----------------------
Closest string:   'jn/'
-----------------------
Closest string:   'rn/'
-----------------------
Closest string:   'vn/'
-----------------------
Closest string:   'vn('
-----------------------
Closest string:   'vna'
-----------------------
Closest string:   'vn)'
-----------------------
Closest string:   'vna'
-----------------------
Closest string:   'ana'
-----------------------
Closest string:   'an#'
-----------------------
Closest string:   'any'
-----------------------
Found the string: 'any' after 15 generations!

```