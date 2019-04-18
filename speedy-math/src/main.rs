use std::time::{Instant};

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
}

fn main() {

    benchmark! {
        println!("omaewamou SHINDEIRUUUUUU! {}", 999999999);
        println!("NANI!?!?!?!?");
    }

    namedbenchmark! {
        #[weeeeeeee]
        println!("OwO notices 🚀");
        println!("the yeet was yote.");
    }

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
