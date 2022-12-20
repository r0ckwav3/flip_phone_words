pub fn modtest(){
    println!("test");
}

pub fn isprime(n: u64) -> bool{
    for i in 2..n{
        if n%i == 0 {
            return false;
        }
    }
    true
}

fn chartodigit(c: char) -> u64{
    match c{
        'a'..='c' => 2,
        'd'..='f' => 3,
        'g'..='i' => 4,
        'j'..='l' => 5,
        'm'..='o' => 6,
        'p'..='s' => 7,
        't'..='v' => 8,
        'w'..='z' => 9,
        _other => 0
    }
}

pub fn wordtonum(word: &String) -> u64{
    let mut ans = 0;
    for (i,c) in word.chars().enumerate(){
        ans += chartodigit(c) * 10_u64.pow((word.len()-(i+1)) as u32);
    }
    ans
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
        assert_eq!(wordtonum(&String::from("test")), 8378);
        assert_eq!(wordtonum(&String::from("a")), 2);
        wordtonum(&String::from("longwordtest"));
    }
}