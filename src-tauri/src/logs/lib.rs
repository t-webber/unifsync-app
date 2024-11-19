#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo
)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::missing_docs_in_private_items, clippy::arithmetic_side_effects)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::print_stderr)]
#![allow(clippy::allow_attributes)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::ref_patterns)]

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse, Ident, ItemFn};
trait ParseIntoString {
    fn parse_into_string(&self) -> String;
}

impl<T: ToTokens + Clone> ParseIntoString for T {
    fn parse_into_string(&self) -> String {
        let tokens: TokenStream = self.into_token_stream().into();
        parse::<Ident>(tokens).map_or_else(|_| String::from(" ? "), |parsed| parsed.to_string())
    }
}

#[proc_macro_attribute]
pub fn logger(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse::<ItemFn>(func).unwrap();
    let pub_tok = func.vis; // pub or empty
    let fn_tok = func.sig.fn_token;
    let name = func.sig.ident; // name
    let args = func.sig.inputs; // args lsit
    let res_t = func.sig.output; // return
    let body = func.block;

    let name_str = name.to_string();
    let args_str = args.parse_into_string();
    let res_str = res_t.parse_into_string();

    quote!(
        #pub_tok #fn_tok #name(#args) #res_t {
            if let Err(er) = fs::OpenOptions::new()
                    .append(true)
                    .open(LOGS_PATH)
                    .and_then(|mut fd| writeln!(fd, "[{}] {}({}) -> {}", chrono::Local::now(), #name_str, #args_str, #res_str))
                {
                    eprint!("Failed to log errors ! ({{er:?}})");
                }
            #body
            }

    )
    .into()
}
