use regex::Regex;

fn main() {
    assert!(Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")));
    assert!(!Solution::is_palindrome(String::from("race a car")));
    assert!(Solution::is_palindrome(String::from(" ")));
}

struct Solution;

impl Solution {
        
    pub fn is_palindrome(s: String) -> bool {
        let re = Regex::new(r"(\s+)|([\p{P}\p{S}])").unwrap();
        let input = re.split(&s).filter(|w| !w.is_empty()).map(|w| w.to_lowercase()).collect::<String>();
        
        let mut copy = input.clone().chars().collect::<Vec<_>>();
        copy.reverse();
        let reverse = copy.into_iter().collect::<String>();

        input == reverse
    }
}