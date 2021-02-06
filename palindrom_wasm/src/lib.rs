use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn searcher(until: usize) -> JsValue {
    let palindromes = palindromes(until);
    return JsValue::from_serde(&palindromes).unwrap()
}

fn palindromes(until: usize) -> Vec<usize> {
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
    palindromes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(palindromes(11), vec![11]);
        assert_eq!(palindromes(22), vec![11, 22]);
    }
}
