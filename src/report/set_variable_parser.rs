use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Expr;
use syn::Ident;
use syn::Lit;
use syn::parse::Parse;
use syn::parse::ParseStream;
use super::custom_keys;

pub struct SetVariableParser {
    variable_name: syn::Ident,
    assign_token: syn::Token![=],
    value: syn::Expr,
}


impl Parse for SetVariableParser{
    fn parse(input: ParseStream) -> syn::Result<Self> {

        let variable_name: Ident = input.parse()?;
        let assign_token: syn::Token![=] = input.parse()?;
        let value: Expr = input.parse()?;

        Ok(Self {
            variable_name,
            assign_token,
            value,

        })
    }
}

impl ToTokens for SetVariableParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {

        let variable_name = &self.variable_name;
        let assign_token = &self.assign_token;
        let value = &self.value;

        tokens.extend(quote! {
            #variable_name
            #assign_token
            #value
        });
    }
}

impl SetVariableParser {
    pub fn translate(input: &str) -> Result<TokenStream, String> {
        let tokens = syn::parse_str::<SetVariableParser>(input);
        if let Err(e) = tokens {
            return Err(e.to_string().clone());
        }

        let tokens = tokens.unwrap();
        Ok(tokens.into_token_stream())
    }
}