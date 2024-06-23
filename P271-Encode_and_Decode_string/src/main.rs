use base64::prelude::*;

fn main() {
    
}

struct Solution;

impl Solution {
    const SECRET: &'static str = "Secret_Key";

    fn encode(strs: Vec<String>) -> Option<String> {
        if strs.is_empty() {
            None
        } else {
            let strs_joined = strs.join(Self::SECRET);
            Some(BASE64_STANDARD.encode(strs_joined))
        }
    }

    fn decode(s: Option<String>) -> Vec<String> {
        match s {
            None => Vec::new(),
            Some(s) => {
                let chars = BASE64_STANDARD.decode(s).unwrap().into_iter().map(|c| (c as char).to_string()).collect::<Vec<_>>();
                let chars_joined = chars.join("");
                chars_joined.split(Self::SECRET).map(|w| w.to_string()).collect::<Vec<_>>()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn test_1() {
        let input = vec!["neet","code","love","you"].iter().map(|x| x.to_string()).collect::<Vec<_>>();

        let encoded = Solution::encode(input.clone());
        let decoded = Solution::decode(encoded);

        assert_eq!(input, decoded);
    }

    #[test]
    fn test_2() {
        let input = vec![""].iter().map(|x| x.to_string()).collect::<Vec<_>>();

        let encoded = Solution::encode(input.clone());
        let decoded = Solution::decode(encoded);

        assert_eq!(input, decoded);
    }

    #[test]
    fn test_3() {
        let input = vec!["neet","","love","you"].iter().map(|x| x.to_string()).collect::<Vec<_>>();

        let encoded = Solution::encode(input.clone());
        let decoded = Solution::decode(encoded);

        assert_eq!(input, decoded);
    }

    #[test]
    fn test_4() {
        let input = vec![];

        let encoded = Solution::encode(input.clone());
        let decoded = Solution::decode(encoded);

        assert_eq!(input, decoded);
    }
}
