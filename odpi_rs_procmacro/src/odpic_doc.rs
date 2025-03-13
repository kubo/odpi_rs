// odpi_rs - a thin wrapper over Oracle Database Programming Interface for C
//
// URL: https://github.com/kubo/odpi_rs
//
//-----------------------------------------------------------------------------
// Copyright (c) 2025 Kubo Takehiro <kubo@jiubao.org>. All rights reserved.
// This program is free software: you can modify it and/or redistribute it
// under the terms of:
//
// (i)  the Universal Permissive License v 1.0 or at your option, any
//      later version (http://oss.oracle.com/licenses/upl); and/or
//
// (ii) the Apache License v 2.0. (http://www.apache.org/licenses/LICENSE-2.0)
//-----------------------------------------------------------------------------
use crate::ODPIC_DOC;
use convert_case::{Case, Casing};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::mem;
use std::sync::LazyLock;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    Attribute, Expr, Ident, ImplItem, Item, LitStr, Meta, Token, Type, Visibility,
};

static ENUM_FIELDS_MAPPING: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("oracle_type", "oracleTypeNum"),
        ("default_native_type", "defaultNativeTypeNum"),
        ("is_plsql", "isPLSQL"),
        ("is_ddl", "isDDL"),
        ("is_dml", "isDML"),
    ])
});

struct KeyValue {
    pub key: Ident,
    pub value: LitStr,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;
        let value = input.parse()?;
        Ok(KeyValue { key, value })
    }
}

struct KeyValues(Vec<KeyValue>);

impl KeyValues {
    fn find<K>(&self, key: K) -> Option<String>
    where
        K: AsRef<str>,
    {
        self.0.iter().find_map(|kv| {
            if kv.key == key {
                Some(kv.value.value())
            } else {
                None
            }
        })
    }
}

impl Parse for KeyValues {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(KeyValues(
            Punctuated::<KeyValue, Token![,]>::parse_terminated(input)?
                .into_iter()
                .collect(),
        ))
    }
}

// The argument of Self::from_bits_retain(...)
struct FromBitsRetainArg(String);

impl Parse for FromBitsRetainArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(from_bits_retain);
        let args;
        let _: Token![Self] = input.parse()?;
        let _: Token![:] = input.parse()?;
        let _: Token![:] = input.parse()?;
        let _: from_bits_retain = input.parse()?;
        syn::parenthesized!(args in input);
        let arg: Ident = args.parse()?;
        Ok(FromBitsRetainArg(arg.to_string()))
    }
}

fn attrs_includes_odpic_doc(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if let Meta::List(path, ..) = &attr.meta {
            if let Some(ident) = path.path.get_ident() {
                if ident == "odpic_doc" {
                    return true;
                }
            }
        }
    }
    false
}

pub fn odpic_doc(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    fn prepend_desc_to_attrs(name: &str, attrs: &mut Vec<Attribute>) {
        if let Some(desc) = ODPIC_DOC.find_desc(name) {
            let mut doc_attrs: Vec<Attribute> = parse_quote! {
                #[doc = #desc]
                /// # Note
                /// Some comments are verbatim copies from ODPI-C doc.
                /// They are written for C language and may be inappropriate for Rust.
            };
            mem::swap(attrs, &mut doc_attrs);
            attrs.extend(doc_attrs);
        } else {
            println!("desc not found: {}", name);
        }
    }

    let args = parse_macro_input!(args as KeyValues);
    let name_opt = args.find("name");

    let mut item = parse_macro_input!(input as Item);
    match &mut item {
        Item::Struct(item_struct) => {
            let ident_name = if let Some(name) = name_opt {
                name
            } else {
                format!("dpi{}", item_struct.ident)
            };
            prepend_desc_to_attrs(&ident_name, &mut item_struct.attrs);

            // Add doc comments of ODPI-C doc to struct fields.
            for field in &mut item_struct.fields {
                if let (Some(field_name), Visibility::Public(_)) = (&field.ident, &field.vis) {
                    let field_name = field_name.to_string();
                    let field_name = ENUM_FIELDS_MAPPING
                        .get(field_name.as_str())
                        .map(|n| n.to_string())
                        .unwrap_or_else(|| field_name.to_case(Case::Camel));
                    let field_name = format!("{}::{}", ident_name, field_name);
                    if let Some(desc) = ODPIC_DOC.find_desc(&field_name) {
                        field.attrs.insert(0, parse_quote! { #[doc = #desc] });
                    } else {
                        println!("desc not found: {}", field_name);
                    }
                }
            }
        }
        Item::Enum(item_enum) => {
            let enum_name = if let Some(name) = name_opt {
                name
            } else {
                format!("dpi{}", item_enum.ident)
            };
            prepend_desc_to_attrs(&enum_name, &mut item_enum.attrs);
            for variant in &mut item_enum.variants {
                if let Some((_, Expr::Path(path))) = &variant.discriminant {
                    if let Some(ident) = path.path.get_ident() {
                        let name = ident.to_string();
                        if let Some(desc) = ODPIC_DOC.find_desc(&name) {
                            variant.attrs.insert(0, parse_quote! { #[doc = #desc] });
                        } else {
                            println!("desc not found: {}", name);
                        }
                    }
                }
            }
        }
        Item::Fn(item_fn) => prepend_desc_to_attrs(name_opt.as_ref().unwrap(), &mut item_fn.attrs),
        Item::Impl(item_impl) => {
            if let Type::Path(ref path) = *item_impl.self_ty {
                let ident_name = if let Some(name) = name_opt {
                    name
                } else {
                    format!("dpi{}", path.path.get_ident().expect("item_impl ident"))
                };
                for item in &mut item_impl.items {
                    if let ImplItem::Fn(impl_item_fn) = item {
                        if let Visibility::Public(_) = impl_item_fn.vis {
                        } else {
                            continue;
                        }
                        if attrs_includes_odpic_doc(&impl_item_fn.attrs) {
                            continue;
                        }
                        let func_name = format!(
                            "{}_{}",
                            ident_name,
                            impl_item_fn.sig.ident.to_string().to_case(Case::Camel)
                        );
                        if let Some(desc) = ODPIC_DOC.find_desc(&func_name) {
                            impl_item_fn
                                .attrs
                                .insert(0, parse_quote! { #[doc = #desc] });
                            continue;
                        }
                        let get_func_name = format!(
                            "{}_get{}",
                            ident_name,
                            impl_item_fn.sig.ident.to_string().to_case(Case::Pascal)
                        );
                        if let Some(desc) = ODPIC_DOC.find_desc(&get_func_name) {
                            impl_item_fn
                                .attrs
                                .insert(0, parse_quote! { #[doc = #desc] });
                            continue;
                        }
                        println!("desc not found: {}", func_name);
                    }
                }
            }
        }
        Item::Const(item_const) => {
            if let Ok(FromBitsRetainArg(name)) =
                syn::parse::<FromBitsRetainArg>(item_const.expr.to_token_stream().into())
            {
                // Get the constant passed to Self::from_bits_retain(), defined by
                // the bitflags macro.
                if let Some(desc) = ODPIC_DOC.find_desc(&name) {
                    item_const.attrs.insert(0, parse_quote! { #[doc = #desc] });
                } else {
                    println!("desc not found: {}", name);
                }
            }
        }
        _ => (),
    }
    quote!(#item).into()
}
