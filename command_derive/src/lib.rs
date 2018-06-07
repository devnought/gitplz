extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(CommandBoxClone)]
pub fn command_box_clone(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = input.ident;

    let expanded = quote! {
        impl CommandBoxClone for #name {
            fn box_clone(&self) -> Box<Command> {
                Box::new(self.clone())
            }
        }
    };

    expanded.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
