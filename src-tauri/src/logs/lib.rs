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
//
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
#![feature(let_chains)]

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse, punctuated::Punctuated, token::Comma, FnArg, ItemFn, LitBool, Pat, PatIdent, PatType,
};

/// This is the state struct to parse arguments lists, i.e., a `TokenStream` of stringified
/// version `soµe_args: &u32, blob: Vec<&[String]>`, to get only the names of the arguments.
struct ParseArgsState {
    /// Equals true iff the cursor is reading a name, false if it reading a type.
    name: bool,
    /// Number of unclosed chevrons at the position of the cursor.
    /// All other groups, `()`, `[]` or `{}`, are treated as group token items so we won't
    /// need to study them here.
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
/// into a `Vec<String>`, with only the names of the arguments.
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
            // skill all groups (`()`, `|]`, `{}`)
            // to see different delimiters, please proc_macro::Group
            // and NOT `proc_macro2::Group` (I know `T` is `proc_macro2`).
            //TODO: understand this incoherence
            T::Group(_) => (),
            // litterals are static numbers, strings...: not allowed
            T::Literal(_) => panic!("Found a litteral in a function declaration"),
        }
    }
    res
}

/// Type of the tokens for the arguments of a function.
type Args = Punctuated<FnArg, Comma>;

#[allow(dead_code)]
fn stringify_args_names(args: &Args) -> String {
    let token_vec = (&args).into_token_stream().into_iter().collect::<Vec<_>>();
    get_args_names(token_vec)
        .into_iter()
        .next()
        .unwrap_or_default()
}

/// Transform a argument to it's identifier, to be evaluated in `quote!`.
fn get_arg_value(arg: FnArg) -> PatIdent {
    match arg {
        syn::FnArg::Typed(PatType { pat, .. }) => match *pat {
            Pat::Ident(ident) => ident,
            _ => panic!("wtf??????"),
        },
        _ => panic!("wtf????"),
    }
}

fn get_args_values(args: &Args) -> Vec<proc_macro2::TokenStream> {
    args.to_owned()
        .into_iter()
        .map(get_arg_value)
        .map(|ident| quote!(format!("{:?}", #ident.clone())))
        .collect()
}

fn log_to_file(func: TokenStream) -> TokenStream {
    let func = parse::<ItemFn>(func).unwrap();
    let pub_tok = func.vis; // `pub` or empty
    let fn_tok = func.sig.fn_token; // `fn` or empty
    let name = func.sig.ident; // name
    let args = func.sig.inputs; // list of arguments
    let res_t = func.sig.output; // return type
    let body = func.block; // body

    let args_ident: Vec<proc_macro2::TokenStream> = get_args_values(&args);
    let name_str = name.to_string();

    quote!(
        #pub_tok #fn_tok #name(#args) #res_t {
            let arg_value: Vec<String> = vec![#(#args_ident),*];
            let res = #body;
            let res_str = if (std::any::TypeId::of::<()>() == std::any::TypeId::of::<()>()) {
                String::from("()")
            } else {
                format!("{res:?}")
            };
            if let Err(er) = fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(LOGS_PATH)
                    .and_then(|mut fd| writeln!(fd, "[{:36}] {:20}({:?}) -> {}", chrono::Local::now().to_string(), #name_str, arg_value.join(", "), res_str))
                {
                    eprintln!("Failed to log errors in {LOGS_PATH} ! ({er:?})");
                };
            res
            }
    ).into()
}

fn notes_state(func: TokenStream) -> TokenStream {
    let func = parse::<ItemFn>(func).unwrap();
    let pub_tok = func.vis; // `pub` or empty
    let fn_tok = func.sig.fn_token; // `fn` or empty
    let name = func.sig.ident; // name
    let args = func.sig.inputs; // list of arguments
    let res_t = func.sig.output; // return type
    let body = func.block; // body

    let args_ident: Vec<proc_macro2::TokenStream> = get_args_values(&args);
    let name_str = name.to_string();

    quote!(
        #pub_tok #fn_tok #name(#args) #res_t {
            let arg_value: Vec<String> = vec![#(#args_ident),*];
            let res = #body;
            let res_str = if (std::any::TypeId::of::<()>() == std::any::TypeId::of::<()>()) {
                String::from("()")
            } else {
                format!("{res:?}")
            };
            if let Err(er) = fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(NOTES_STATE)
                    .and_then(|mut fd| writeln!(fd, "[{:36}] {:20}({:?}) -> {}", chrono::Local::now().to_string(), #name_str, arg_value.join(", "), res_str))
                {
                    eprint!("Failed to log errors in {NOTES_STATE} ! ({{er:?}})");
                };
            res
            }
    ).into()
}

/// Main logger function that is used as procedural macro attribute on the notes API functions.
#[proc_macro_attribute]
pub fn logger(attr: TokenStream, func: TokenStream) -> TokenStream {
    if let Ok(LitBool { value, .. }) = parse::<LitBool>(attr)
        && value
    {
        notes_state(func)
    } else {
        log_to_file(func)
    }
}
