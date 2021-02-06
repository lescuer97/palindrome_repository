
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn searcher(num: usize ) -> JsValue  {
    
    let capacity = num / 350;
    
    // this is for optimizing and using the right amount of memory spec
    let mut palindrome: Vec<usize> = Vec::with_capacity(capacity);
    
        for i in 10..=num {
      
            let reversed_number: usize = i.to_string().chars().rev().collect::<String>().parse::<usize>().unwrap();
            if i  == reversed_number {
                     palindrome.push(reversed_number);    
            }
        }

       return JsValue::from_serde(&palindrome).unwrap(); 
    }
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
