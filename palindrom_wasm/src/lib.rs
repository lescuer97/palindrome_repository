
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn searcher(num: usize ) -> JsValue  {
    // it's a mathematical imposibility to need to allocate more than the imput divided by 350
    let capacity = num / 350;
    
    // this is for optimizing and using the right amount of memory spec
    let mut palindrome: Vec<usize> = Vec::with_capacity(capacity);
    
        for i in 10..=num {
            
            let string_number = i.to_string();

            let reversed_number: String = string_number.chars().rev().collect::<String>();
     
            if string_number[..0]  == reversed_number[..0] {

                if string_number == reversed_number {
                
                     palindrome.push(i);    
                    }
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
