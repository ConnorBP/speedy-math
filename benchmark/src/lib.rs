use std::time::{Duration, Instant};
use proc_macro::{Literal, Span, Ident, TokenStream, TokenTree};
use syn::{parse_macro_input, DeriveInput};
use syn;
use quote::quote;

extern crate proc_macro;

/* #[proc_macro]
pub fn minimal(input: TokenStream) -> TokenStream {
    let Combinations { name, n } = parse_macro_input!(input as Combinations);
    (quote!{
        fn #name() -> i32 {
            #n
        }
    }).into()
} */


#[proc_macro]
pub fn benchmark2(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.ident;
    let abi = &input.abi;

    //let mut benchmark_code = "use std::time::{Duration, Instant}; let start = Instant::now();".to_string().to_owned();
    //benchmark_code.push_str(&item.to_string().to_owned());
    //benchmark_code.push_str("let duration = start.elapsed(); println!('Time elapsed in expensive_function() is: {:?}', duration);");
    
/*     let inner = item.clone().to_string().to_owned();

    let together = format!("{}{}{}", "use std::time::{Duration, Instant}; let start = Instant::now();", &inner, "let duration = start.elapsed(); println!('Time elapsed in expensive_function() is: {:?}', duration);");

    together.parse().unwrap() */

    /* (quote! {
        //timer imports needed for benchmark macro
        use std::time::{Duration, Instant};

        let start = Instant::now();

        #input

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);

    }).into() */
    "fn yeet() -> u32 { 42 }".parse().unwrap()

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
