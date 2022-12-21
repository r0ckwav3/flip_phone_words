use std::iter;

pub fn modtest() {
    println!("test");
}

pub fn isprime(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn chartodigit(c: char) -> u64 {
    match c {
        'a'..='c' => 2,
        'd'..='f' => 3,
        'g'..='i' => 4,
        'j'..='l' => 5,
        'm'..='o' => 6,
        'p'..='s' => 7,
        't'..='v' => 8,
        'w'..='z' => 9,
        _other => 0,
    }
}

pub fn wordtonum(word: &str) -> u64 {
    let mut ans = 0;
    for (i, c) in word.chars().enumerate() {
        ans += chartodigit(c) * 10_u64.pow((word.len() - (i + 1)) as u32);
    }
    ans
}

fn countdigits(num: u64) -> u32 {
    ((num as f64).log(10.0).floor() as u32) - 1
}

// think of it as a slice [start, end)
fn truncatenum(num: u64, start: u32, end: u32) -> u64 {
    let numlen = countdigits(num);
    (num % 10_u64.pow(numlen - start)) / 10_u64.pow(numlen - end)
}

pub fn numtophrase<'a, Itertype>(num: u64, words: &mut Itertype) -> Vec<&'a str>
where
    Itertype: iter::Iterator<Item = &'a str>,
{
    let numlen = countdigits(num);
    // if a word can be inserted at index i and match all the following numbers
    let mut indexmatches: Vec<Vec<&str>> = Vec::with_capacity(numlen as usize);
    for word in words {
        let wordnum = wordtonum(word);
        for index in 0..(numlen - ((word.len() as u32) - 1)) {
            if wordnum == truncatenum(num, index, index + word.len() as u32) {
                indexmatches[index as usize].push(word);
            }
        }
    }

    vec!["test phrase"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chartodigit_test() {
        assert_eq!(chartodigit('a'), 2);
        assert_eq!(chartodigit('c'), 2);
        assert_eq!(chartodigit('u'), 8);
        assert_eq!(chartodigit(' '), 0);
    }

    #[test]
    fn wordtonum_test() {
        assert_eq!(wordtonum("test"), 8378);
        assert_eq!(wordtonum("a"), 2);
        wordtonum(&String::from("longwordtest"));
    }

    #[test]
    fn isprime_test() {
        assert!(!isprime(91));
        assert!(isprime(17));
        assert!(!isprime(132401897324091));
    }
}
