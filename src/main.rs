use std::fs;
use std::io;

use flip_phone_words::*;

fn main() {
    // prime stuff
    let word = String::from("prime");
    let wordnum = wordtonum(&word);
    println!("\"{}\" on a keypad is: {}", word, wordnum);
    if isprime(wordnum) {
        println!("prime!");
    } else {
        println!("not prime ");
    }

    for prefix in 0..100 {
        let tempnum = prefix * 100000 + wordnum;
        if isprime(tempnum) {
            println!("... but {} is!", tempnum);
        }
    }

    // num to word stuff
    let filename = "top1000words.txt";
    let filecontents = fs::read_to_string(filename)
            .expect(&format!("could not find file \"{}\"", filename));
    let mut words = filecontents.split_whitespace();

    println!("Please input a number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num = input.trim().parse().expect("Input is not a number");

    let result = numtophrase(num, &mut words);
    for phrase in result {
        println!("{}", phrase)
    }
}
