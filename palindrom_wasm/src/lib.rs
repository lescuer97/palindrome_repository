
use wasm_bindgen::prelude::*;
use std::io::Write;

#[wasm_bindgen]
pub fn searcher(num: u32) -> Box<[u32]> {
    // it's a mathematical imposibility to need to allocate more than the imput divided by 350
    let capacity = if num < 1000 { num / 10 } else { num / 350 };

    // this is for optimizing and using the right amount of memory spec
    let mut palindrome = Vec::with_capacity(capacity as usize);

    let mut buffer = Vec::with_capacity(16);

    'outer: for i in 11..=num {
        buffer.clear();
        if let Err(_) = buffer.write_fmt(format_args!("{}", i)) {
            std::process::abort();
        }

        let len = buffer.len();
        for i in 0..len / 2 {
            if buffer[i] != buffer[len - 1 - i] {
                continue 'outer;
            }
        }

        palindrome.push(i);
    }
    // returns array of every palindrome number
    palindrome.into_boxed_slice()
}