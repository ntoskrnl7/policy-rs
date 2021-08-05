use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Loader)]
pub fn loader_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl Loader<#name> for #name {
            fn from_path<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<#name> {
                let mut text = String::new();
                use std::io::Read;
                std::fs::File::open(path)?.read_to_string(&mut text)?;
                Self::from_str(text)
            }

            fn from_str<T: AsRef<str>>(text: T) -> std::io::Result<#name> {
                match Self::from_yaml(&text) {
                    Ok(res) => Ok(res),
                    Err(_) => match Self::from_json(&text) {
                        Ok(res) => Ok(res),
                        Err(_) => match Self::from_toml(&text) {
                            Ok(res) => Ok(res),
                            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, err)),
                        },
                    },
                }
            }

            fn from_yaml<T: AsRef<str>>(yaml: T) -> Result<#name, serde_yaml::Error> {
                match serde_yaml::from_str::<#name>(yaml.as_ref()) {
                    Ok(mut res) => Ok(policy::Base::adjust_default_options(&mut res)),
                    Err(err) => Err(err),
                }
            }

            fn from_json<T: AsRef<str>>(json: T) -> Result<#name, serde_json::Error> {
                match serde_json::from_str::<#name>(json.as_ref()) {
                    Ok(mut res) => Ok(policy::Base::adjust_default_options(&mut res)),
                    Err(err) => Err(err),
                }
            }

            fn from_toml<T: AsRef<str>>(toml: T) -> Result<#name, toml::de::Error> {
                match toml::from_str::<#name>(toml.as_ref()) {
                    Ok(mut res) => Ok(policy::Base::adjust_default_options(&mut res)),
                    Err(err) => Err(err),
                }
            }
        }
    };
    TokenStream::from(expanded)
}
