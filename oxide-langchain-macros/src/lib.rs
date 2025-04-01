mod tool;

use proc_macro::TokenStream;
use tool::main;

#[proc_macro_attribute]
pub fn tool(_args: TokenStream, item: TokenStream) -> TokenStream {
    tool::main(item)
}
