extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated, ItemMacro, LitInt, Macro, Token};
use thiserror::Error;

#[proc_macro_attribute]
pub fn expand(attrs: TokenStream, input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input);
	let attrs = parse_macro_input!(attrs);
	expand_expand(attrs, input).unwrap_or_else(syn::Error::into_compile_error).into()
}

fn expand_expand(attrs: LitInt, input: ItemMacro) -> syn::Result<proc_macro2::TokenStream> {
	let n: usize = attrs.base10_parse()?;
	let macro_ident = input.ident.as_ref().ok_or(Error::MacroRulesExpected)?;
	let idents = (0..n).map(|n| format_ident!("T{n}"));
	let mut punctuated: Punctuated<Ident, Token![,]> = Punctuated::new();
	let mut invocations: Vec<Macro> = Vec::with_capacity(n);
	
	for ident in idents {
		punctuated.push(ident);
		invocations.push(parse_quote!(#macro_ident!(#punctuated)));
	}

	Ok(quote! {
		#input
		#(#invocations;)*
	})
}

#[derive(Debug, Error)]
enum Error {
	#[error("`macro_rules!` expected")]
	MacroRulesExpected,
}

impl From<Error> for syn::Error {
	fn from(value: Error) -> Self {
		syn::Error::new(Span::call_site(), value.to_string())
	}
}
