use std::{collections::VecDeque, hash::{DefaultHasher, Hash, Hasher}};

fn main() {
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut group_anagrams = Vec::new();
    
    let mut hash_strs = strs.clone().into_iter().map(|str| -> u64 {
        let mut str = str.chars().collect::<Vec<_>>();
        str.sort();
        let str = str.into_iter().collect::<String>();
        let mut hasher = DefaultHasher::new();
        str.hash(&mut hasher);
        hasher.finish()
    }).collect::<VecDeque<_>>();
    
    let mut strs = strs.into_iter().collect::<VecDeque<_>>();

    loop {
        let mut anagrams = Vec::new();
        if let Some(hash) = hash_strs.pop_front() {
            anagrams.push(strs.pop_front().unwrap());
            let mut hash_strs_index = hash_strs.iter().enumerate().filter(|(i, h)| h == &&hash).map(|(index, _)| index).collect::<Vec<_>>();
            hash_strs_index.reverse();
            hash_strs_index.into_iter().for_each(|index| {
                hash_strs.remove(index);
                anagrams.push(strs.remove(index).unwrap());
            });
            group_anagrams.push(anagrams);
        } else {
            break;
        }
    }
    
    group_anagrams 
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_1() {
        let input = vec![String::from("eat"), String::from("tea"), String::from("tan"), String::from("nat"), String::from("ate"), String::from("bat")];
        let output = vec![vec![String::from("bat")], vec![String::from("nat"), String::from("tan")], vec![String::from("ate"), String::from("eat"), String::from("tea")]];
        
        let group_anagrams = group_anagrams(input);
        
        assert_eq!(output, group_anagrams);
    }

    #[test]
    fn test_2() {
        let input = vec![String::from("")];
        let output = vec![vec![String::from("")]];
        
        let group_anagrams = group_anagrams(input);
        
        assert_eq!(output, group_anagrams);
    }

    #[test]
    fn test_3() {
        let input = vec![String::from("a")];
        let output = vec![vec![String::from("a")]];
        
        let group_anagrams = group_anagrams(input);
        
        assert_eq!(output, group_anagrams);
    }
}