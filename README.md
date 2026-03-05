# CodeWars--From-A-to-Z-7-kyu---Passed
Given a string indicating a range of letters, return a string which includes all the letters in that range, including the last letter. Note that if the range is given in capital letters, return the string in capitals also!
Given a string indicating a range of letters, return a string which includes all the letters in that range, including the last letter.
Note that if the range is given in capital letters, return the string in capitals also!

Examples
"a-z" ➞ "abcdefghijklmnopqrstuvwxyz"
"h-o" ➞ "hijklmno"
"Q-Z" ➞ "QRSTUVWXYZ"
"J-J" ➞ "J"
Notes
A hyphen will separate the two letters in the string.
You don't need to worry about error handling in this kata (i.e. both letters will be the same case and the second letter will not be before the first alphabetically).

TEST CASES:
#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use super::gimme_the_letters;
        
    fn dotest(sp: &str, expected: &str) {
        let actual = gimme_the_letters(sp);
        assert!(actual == expected, 
            "With sp = \"{sp}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("a-z", "abcdefghijklmnopqrstuvwxyz");
        dotest("h-o", "hijklmno");
        dotest("Q-Z", "QRSTUVWXYZ");
        dotest("J-J", "J");
        dotest("a-b", "ab");
        dotest("A-A", "A");
        dotest("g-i", "ghi");
        dotest("H-I", "HI");
        dotest("y-z", "yz");
        dotest("e-k", "efghijk");
        dotest("a-q", "abcdefghijklmnopq");
        dotest("F-O", "FGHIJKLMNO");
    }
    
    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let length = rng.gen_range(1u8..=26);
            let a = (if rand::random() { 97 } else { 65 } + rng.gen_range(0..=26-length)) as char;
            let expected = &(a..(a as u8 + length) as char).collect::<String>();
            let s = &format!("{}-{}", a, (a as u8 + length - 1) as char);
            dotest(s, expected);
        }
    }
}
