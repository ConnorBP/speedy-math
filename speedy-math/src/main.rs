/* use std::time::{Instant};

macro_rules! benchmark {
    ($($lines:stmt;)*) => {
        let start = Instant::now();
        $($lines;)*
        let duration = start.elapsed();
        println!("Benchmark time: {:?}", duration);
    };
}

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
} */
//#[macro_use]
extern crate benchme;
use benchme::{benchmark, benchmarknamed};
use std::time::Instant;

use std::usize;

fn main() {
    benchmark! {
        println!("omaewamou SHINDEIRUUUUUU! {}", 999_999_999);
        println!("NANI!?!?!?!?");
    }

    benchmarknamed! {
        #[weeeeeeee]
        println!("OwO notices 🚀");
        println!("the yeet was yote.");
    }
}

fn fast_between(value: isize, lower: isize, upper: isize) -> bool {
    // use a < for an inclusive lower bound and exclusive upper bound
    // use <= for an inclusive lower bound and inclusive upper bound
    // alternatively, if the upper bound is inclusive and you can pre-calculate
    //  upper-lower, simply add + 1 to upper-lower and use the < operator.
    //  if ((unsigned)(value-lower) <= (upper-lower))
    //      in_range(value);

    //println!("value({}) - lower({}) = {} value - lower: {}", value, lower, ((value-lower) as usize) <= (upper - lower) as usize, ((value-lower) as usize)); 
    
    //WARNING: rust potentially does not like unsigned overflowing as it might see it as undefined behaviour.
    //We might need some way to tell it this use is ok if they decide to enforce the panic for overflows
    ((value-lower) as usize) <= (upper - lower) as usize //assumes lower is less than upper
}

fn slow_between(value: isize, lower: isize, upper: isize) -> bool {
    value <= upper && value >= lower
}

fn sin_lut() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark() {
        //assert!(stringify!(benchmark!{}) contains "let start = Instant::now();");
        //TODO CHECK FOR VALID OUTPUT
        assert!(true);
    }
    #[test]
    fn test_benchmark_named() {
        //assert!(stringify!(benchmark!{}) contains "let start = Instant::now();");
        //TODO CHECK FOR VALID OUTPUT
        assert!(true);
    }

    #[test]
    fn fast_between_benchmark() {
        //needs to be run with cargo test -- --nocapture
        
        //this one is needed to make sure the results after are fair.
        //The first one seems to always take longer because it has to initialize things
        benchmarknamed! {
            #[warmup_time]
            for x in -1000..1000 {
            slow_between(x, -4, 4);
            };
        }

        benchmarknamed! {
            #[slow_between]
            for x in 0..10000000 {
                slow_between((x % 20) -10, -4, 4);
            };
        }

        benchmarknamed! {
            #[fast_between]
            for x in 0..10000000 {
                fast_between((x % 20) -10, -4, 4);
            };
        }

    }


    #[test]
    fn fast_between_is_correct() {
        for x in 0..1000 {
            //wraps from -10 to 10 a lot of times
            let fast = fast_between((x % 20) -10, -4, 4);
            let slow = slow_between((x % 20) -10, -4, 4);

            assert_eq!(
                fast, slow,
                "fast_between failed at x = `{}` fast: `{}` slow: `{}`",
                x, fast, slow
            );

        }
        //assert propper overflow subtraction https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_sub
        assert_eq!(5i32.overflowing_sub(2), (3, false));
        assert_eq!(usize::MIN.overflowing_sub(1), (usize::MAX, true));
        
        //check usize wrapping_sub
        assert_eq!(100usize.wrapping_sub(100), 0);
        assert_eq!(100usize.wrapping_sub(usize::max_value()), 101);
        //check u32 wrapping_sub https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_sub
        assert_eq!(100u32.wrapping_sub(100), 0);
        assert_eq!(100u32.wrapping_sub(u32::max_value()), 101);
    }
}
