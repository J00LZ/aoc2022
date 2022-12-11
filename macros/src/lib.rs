use std::ops::Deref;

use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, LitInt, Token};

struct Days(Vec<u32>);

impl Parse for Days {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut vec = Vec::new();
        while !input.is_empty() {
            let lit: LitInt = input.parse()?;
            vec.push(lit.base10_parse()?);
            input.parse::<Option<Token![,]>>()?;
        }
        Ok(Days(vec))
    }
}

impl IntoIterator for Days {
    type Item = u32;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for Days {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[proc_macro]
pub fn days(attrs: TokenStream) -> TokenStream {
    let days = syn::parse_macro_input!(attrs as Days);
    let modules = days.iter().copied().map(|day| {
        let module_name = format_ident!("day{:02}", day);
        let static_name = format_ident!("DAY{:02}", day);
        let type_name = format_ident!("Day{:02}", day);
        let test_module_name = format_ident!("test_day{:02}", day);
        let module = quote! {
            pub mod #module_name;
            #[cfg(test)]
            pub static #static_name: Lazy<#module_name::#type_name> = Lazy::new(|| {
                #module_name::#type_name::new(std::fs::read_to_string(&format!("../input/day{:02}.txt", #day)).unwrap())
            });
            #[cfg(not(test))]
            pub static #static_name: Lazy<#module_name::#type_name> = Lazy::new(|| {
                #module_name::#type_name::new(std::fs::read_to_string(&format!("./input/day{:02}.txt", #day)).unwrap())
            });

            #[cfg(test)]
            mod #test_module_name {
                use super::#static_name;
                use std::fs::read_to_string;
                use super::Day;
                use std::path::Path;
                use super::get_files;

                #[test]
                fn real_part1() {
                    assert_eq!(
                        #static_name.part1(),
                        read_to_string(format!("../output/day{:02}-p1.txt", #day)).unwrap()
                    );
                }

                #[test]
                fn real_part2() {
                    assert_eq!(
                        #static_name.part2(),
                        read_to_string(&format!("../output/day{:02}-p2.txt", #day)).unwrap()
                    );
                }

                #[test]
                fn sample_part1() {
                    for (i, o) in get_files(#day, 1) {
                        let input = std::fs::read_to_string(i.path()).unwrap();
                        let output = std::fs::read_to_string(o.path()).unwrap();
                        let day = super::#module_name::#type_name::new(input);
                        assert_eq!(day.part1(), output);
                    }
                }
            }
        };
        module
    }).collect_vec();
    let z = days
        .iter()
        .copied()
        .map(|day| {
            let static_name = format_ident!("DAY{:02}", day);
            quote! {
                #static_name.deref()
            }
        })
        .collect_vec();
    quote! {
        use std::ops::Deref;
        use once_cell::sync::Lazy;
        use std::fs::{DirEntry, read_dir};

        #(#modules)*
        pub static DAYS: Lazy<Vec<&dyn Day>> = Lazy::new(|| {
            vec![
                #(
                    #z,
                )*
            ]
        });
        fn get_files(day: u32, part: u32) -> Vec<(DirEntry, DirEntry)> {
            let tests_inputs = read_dir("../testinput")
                .unwrap()
                .into_iter()
                .filter(|it| {
                    it.as_ref()
                        .unwrap()
                        .file_name()
                        .to_string_lossy()
                        .starts_with(&format!("day{:02}", day))
                })
                .flatten()
                .collect::<Vec<_>>();
            let outputs = read_dir("../testoutput")
                .unwrap()
                .into_iter()
                .filter(|it| {
                    it.as_ref()
                        .unwrap()
                        .file_name()
                        .to_string_lossy()
                        .starts_with(&format!("day{:02}-p{}", day, part))
                })
                .flatten()
                .collect::<Vec<_>>();
            tests_inputs.into_iter().zip(outputs).collect()
        }
    }
    .into()
}
