extern crate logs_defs;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse, DeriveInput, Ident, ItemFn};

#[proc_macro_derive(Trait)]
pub fn trait_derive(input: TokenStream) -> TokenStream {
    let ast = parse::<DeriveInput>(input).unwrap();
    let name = ast.ident;

    let output = quote! {
        impl Trait for #name {
            fn helo() {
                println!("blob");
            }
        }
    };
    output.into()
}

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
            println!("[{}] {}({}) -> {}", chrono::Local::now(), #name_str, #args_str, #res_str);
            #body
            }

    )
    .into()
    // if let Err(er) = fs::OpenOptions::new()
    //         .append(true)
    //         .open(LOGS_PATH)
    //         .and_then(|mut fd| writeln!(fd, "{{msg}}: {{err:?}}"))
    //     {{
    //         eprint!("Failed to log errors ! ({{er:?}})");
    // code.parse().unwrap()
}
