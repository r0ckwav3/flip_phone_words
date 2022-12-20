use flip_phone_words::*;

fn main() {
    let word = String::from("prime");
    let wordnum = wordtonum(&word);
    println!("\"{}\" on a keypad is: {}", word, wordnum);
    if isprime(wordnum){
        println!("prime!");
    }else{
        println!("not prime ");
    }
    
    for prefix in 0..100{
        let tempnum = prefix*100000 + wordnum;
        if isprime(tempnum){
            println!("... but {} is!", tempnum);
        }
    }
}