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
#![allow(clippy::arithmetic_side_effects)]
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

struct ParseArgsState {
    name: bool,
    parens: u32,
    brackets: u32,
    chevrons: u32,
}

impl ParseArgsState {
    /// Functions that tells you if you are inside or inside of a type.
    /// If it returns true, the comma means you pass to next argument.
    /// If it false, you are still inside of a <> or a () or a [].
    fn is_unfolded(&self) -> bool {
        self.brackets == 0 && self.parens == 0 && self.chevrons == 0
    }

    fn new() -> Self {
        Self {
            name: true,
            parens: 0,
            brackets: 0,
            chevrons: 0,
        }
    }
}

/// Transforms a TokenStream that is a vec of arguments of a function
/// into a vec of strings, with only the names of the arguments.
fn get_args_names(args: Vec<proc_macro2::TokenTree>) -> Vec<String> {
    use proc_macro2::TokenTree as T;
    let mut res = vec![];
    let mut state = ParseArgsState::new();
    for arg in args {
        match arg {
            T::Punct(punct) => match punct.as_char() {
                ':' if state.is_unfolded() => state.name = false,
                ',' if state.is_unfolded() => state.name = true,
                ',' => (),
                '<' => state.chevrons += 1,
                '>' => state.chevrons += 1,
                '(' => state.parens += 1,
                ')' => state.parens -= 1,
                '[' => state.brackets += 1,
                ']' => state.brackets -= 1,
                '&' => (),
                x if x.is_alphanumeric() => (),
                x => panic!("Found the strange character: {x}"),
            },
            T::Ident(ident) if state.name => res.push(ident.to_string()),
            T::Ident(_) => (),
            T::Group(_) => todo!(),
            T::Literal(_) => todo!(),
        }
    }
    res
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

    let args_vec_tok = (&args).into_token_stream().into_iter().collect::<Vec<_>>();
    dbg!(&args_vec_tok);
    let args_vec_str = get_args_names(args_vec_tok);
    dbg!(&args_vec_str);

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
