/*
#![feature(proc_macro_hygiene)]

//put this in Cargo.toml for the procedural macro
[dependencies]
benchmark_macros = { version = "*", path = "../benchmark" }
*/

use std::time::{Instant};
/* 
use proc_macro::{Literal, Span, Ident, TokenStream, TokenTree};
use proc_macro::quote; */

/* macro_rules! benchmark1 {
    ($t:expr) => {{
        let start = Instant::now();
        $t
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }}
} */

//use benchmark_macros::benchmark2;

macro_rules! benchmark {
    ($($lines:stmt;)*) => {
        let start = Instant::now();
        $($lines;)*
        let duration = start.elapsed();
        println!("Benchmark time: {:?}", duration);
    };
}
//#[$($outer:meta) ?]
macro_rules! namedbenchmark {
    (
        #[$($name:ident)?]
        $(
            $(#[$inner:ident $($args:tt)*])*
            $lines:stmt;
        )*
    ) => {
            let start = Instant::now();
            $($lines;)*
            let duration = start.elapsed();
            println!("{} time: {:?} | {}", stringify!($($name)?), duration, stringify!(args));
        };
}

fn main() {
    
/*  benchmark2! {
        println!("Very challenging workload, I am.");
        println!("Is it in between? {}.", if fast_between(0, -1, 1) {"Yes"} else {"No"});
    } */

    benchmark! {
        println!("omaewamou SHINDEIRUUUUUU! {}", 999999999);
        println!("NANI!?!?!?!?");
    }

    namedbenchmark! {
        #[weeeeeeee]
        println!("OwO notices 🚀");
        println!("the yeet was yote.");
    }

/*     benchmark2! {
        //run codein here
        println!("I am hard to run! :D");
        println!("yeet");
    } */
}




fn fast_between(value : u16, lower : u16, upper : u16) -> bool {
// use a < for an inclusive lower bound and exclusive upper bound
// use <= for an inclusive lower bound and inclusive upper bound
// alternatively, if the upper bound is inclusive and you can pre-calculate
//  upper-lower, simply add + 1 to upper-lower and use the < operator.
  //  if ((unsigned)(value-lower) <= (upper-lower))
  //      in_range(value);

    //let lu = (lower-upper) as u16;
    //let ul = upper-lower;
    //let vnorm = value-upper;
    //if vnorm < ul && < lu
    (value-lower) as u16 <= (upper - lower)
}