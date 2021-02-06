
//  @deno-types="./palindrom_wasm/out/palindrom_wasm.d.ts";

// This is a web assembly function built in Rust
// import { searcher } from "./palindrom_wasm/deno_wasm/palindrom_wasm.js";

//typescript palindrome searcher
import {searcher} from './searcher.ts';

// input of the limit number

const initialDate =  performance.now()
searcher(999999);
// searcher(1112); 
const finalDate = performance.now()
console.log("Miliseconds: ",finalDate - initialDate);

// SMALL TEST OF SPEED with input of 99999999: 
    // WASM: 28978, 30456, 29845
    // TS: 19377, 19624, 18488