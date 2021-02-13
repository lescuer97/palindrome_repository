use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn searcher(until: usize) -> Box<[usize]> {
    let palindromes = palindromes(until);
    return palindromes
}

fn palindromes(until: usize) -> Box<[usize]> {
    let mut palindromes: Vec<usize> = Vec::with_capacity(until / 350);
    let mut string_number = String::new();
    for i in 10..=until {
        string_number.clear();
        use core::fmt::Write as _;
        write!(string_number, "{}", i).unwrap();
        if string_number
            .chars()
            .zip(string_number.chars().rev())
            .all(|(forward, backward)| forward == backward)
        {
            palindromes.push(i);
        }
        
    }
    return palindromes.into_boxed_slice();
}

#[cfg(test)]
mod tests {
    use super::*;   
    
    #[test]
    fn it_works() {
        let sample_1: Box<[usize]> = Box::new([11]);
        let sample_2: Box<[usize]> = Box::new([11, 22]);
        let sample_3: Box<[usize]> = Box::new([11, 22,33,44,55,66,77,88,99,101,111]);

        assert_eq!( palindromes(11), sample_1);
        assert_eq!(palindromes(22), sample_2);
        assert_eq!(palindromes(111), sample_3);
    }
}
