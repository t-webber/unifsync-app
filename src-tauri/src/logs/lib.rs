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
//
#![feature(if_let_guard)]

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse, punctuated::Punctuated, token::Comma, FnArg, Ident, ItemFn, Pat, PatIdent, PatType,
};

#[allow(dead_code)]
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
    chevrons: u32,
}

impl ParseArgsState {
    /// Functions that tells you if you are inside or inside of a type.
    /// - If it returns `true`, the comma means you pass to next argument.
    /// - If it returns `false`, you are still inside of a `<>`.
    ///
    /// # Note
    /// We can't be inside of `()`, `[]` or `{}` because there are treated as
    /// groups, so are ignored when parsing the `TokenStream`..
    fn is_unfolded(&self) -> bool {
        self.chevrons == 0
    }

    fn new() -> Self {
        Self {
            name: true,
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
                // colon found outside of type: switching to type name (end of argument name)
                ':' if state.is_unfolded() => state.name = false,
                // comma found outside of type: switch back to argument name (end of type name)
                ',' if state.is_unfolded() => state.name = true,
                // comma inside of type name: ignore
                ',' => (),
                // chevrons are not considered groups, so we need to do it manually
                '<' => state.chevrons += 1,
                '>' => state.chevrons += 1,
                '&' => (),
                x => panic!("Unsupported character: {x}"),
            },
            T::Ident(ident) if state.name => res.push(ident.to_string()),
            T::Ident(_) => (),
            // skill all groups ((), |], {})
            // to see different delimiters, please proc_macro::Group
            // and NOT proc_macro2::Group (I know T is proc_macro2).
            //TODO: understand this incoherence
            T::Group(_) => (),
            // litterals are static numbers, strings...: not allowed
            T::Literal(_) => panic!("Found a litteral in a function declaration"),
        }
    }
    res
}

fn stringify_args_names(args: &Punctuated<FnArg, Comma>) -> String {
    let token_vec = (&args).into_token_stream().into_iter().collect::<Vec<_>>();
    get_args_names(token_vec)
        .into_iter()
        .next()
        .unwrap_or_default()
}

fn get_arg_value(arg: FnArg) -> PatIdent {
    match arg {
        syn::FnArg::Typed(PatType { pat, .. }) => match *pat {
            Pat::Ident(ident) => ident,
            _ => panic!("wtf??????"),
        },
        _ => panic!("wtf????"),
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
    let arg_str = stringify_args_names(&args);

    let first_arg_tok = args.first().unwrap().to_owned();
    let first_arg_ident = get_arg_value(first_arg_tok);

    quote!(
        #pub_tok #fn_tok #name(#args) #res_t {
            let res = #body;
            let res_str = if (std::any::TypeId::of::<()>() == std::any::TypeId::of::<()>()) {
                String::from("()")
            } else {
                format!("{res:?}")
            };
            let arg_value = #first_arg_ident;
            if let Err(er) = fs::OpenOptions::new()
                    .append(true)
                    .open(LOGS_PATH)
                    .and_then(|mut fd| writeln!(fd, "[{}] {}({} = {:?}) -> {}", chrono::Local::now(), #name_str, #arg_str, arg_value, res_str))
                {
                    eprint!("Failed to log errors ! ({{er:?}})");
                };
            res
            }
    ).into()
}
