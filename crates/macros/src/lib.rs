use proc_macro::TokenStream;

#[proc_macro]
pub fn rx(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn rsx(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
