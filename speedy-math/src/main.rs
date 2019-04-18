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

fn fast_between(value: i16, lower: i16, upper: i16) -> bool {
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
    ((value - lower) as u16) as i16 <= (upper - lower)
}

fn slow_between(value: i16, lower: i16, upper: i16) -> bool {
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
    fn fast_between_is_correct() {
        for x in -10..10 {
            let fast = fast_between(x, -4, 4);
            let slow = slow_between(x, -4, 4);

            assert_eq!(
                fast, slow,
                "fast_between failed at x = `{}` fast: `{}` slow: `{}`",
                x, fast, slow
            );
        }
    }
}
