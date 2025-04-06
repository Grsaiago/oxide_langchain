mod tool;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn tool(_args: TokenStream, item: TokenStream) -> TokenStream {
    tool::main(item)
}
