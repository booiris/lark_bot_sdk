// 特别鸣谢 copilot and chatgpt💕
use std::collections::{HashMap, HashSet};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[derive(PartialEq, Eq, Hash)]
enum ParamKind {
    Query,
    Body,
    Path,
    Header,
    Stream,
    Unknown,
}

impl From<String> for ParamKind {
    fn from(value: String) -> Self {
        match value.as_str() {
            "query" => ParamKind::Query,
            "body" => ParamKind::Body,
            "path" => ParamKind::Path,
            "header" => ParamKind::Header,
            "stream" => ParamKind::Stream,
            _ => panic!("unknown param kind, v: {}", value),
        }
    }
}

#[derive(PartialEq, Eq)]
enum StructValueType {
    List,
    Var,
}

impl From<String> for StructValueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "list" => StructValueType::List,
            "var" => StructValueType::Var,
            _ => panic!("unknown struct value type, v: {}", value),
        }
    }
}

/// 将请求结构体变成 lark_bot_sdk 接受的 api 请求结构体。
///
/// 目前接受 4 种类型:
///
/// | 类型   | 用途                      | 示例                                                               |
/// | ------ | ------------------------- | ------------------------------------------------------------------ |
/// | query  | 请求中的 query            | #[api(kind = "query",name = "xx",v_type = "var",option = "false")] |
/// | path   | 替换请求 url 中的可变参数 | #[api(kind = "path", name = "xx")]                         |
/// | body   | 请求中的 body             | #[api(kind = "body", name = "xx")]                               |
/// | stream | 流式请求                  | #[api(kind = "stream", name = "#data#", option = "false")]         |
///
/// # Examples
///
/// ```
/// use crate::core::model::{ReqParam, StreamReqParam};
/// #[derive(Debug, serde::Serialize, Clone,lark_bot_sdk::macros::ApiReqParams)]
/// struct Req {
///     #[api(kind = "query", name = "id", v_type = "var", option = "false")]
///     pub id: i64,
///     #[api(kind = "query", name = "opt_id", v_type = "var", option = "true")]
///     pub opt_id: Option<i64>,
///     #[api(kind = "query", name = "list_ids", v_type = "list", option = "false")]
///     pub list_ids: Vec<i64>,
///     #[api(
///         kind = "query",
///         name = "opt_list_ids",
///         v_type = "list",
///         option = "true"
///     )]
///     pub opt_list_ids: Option<Vec<i64>>,
///
///     #[api(kind = "path", name = "name")]
///     pub name: String,
///
///     #[api(kind = "body", name = "name")]
///     pub body_name: String,
/// }
/// ```
#[proc_macro_derive(ApiReqParams, attributes(api))]
pub fn api_req_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match input.data {
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Named(fields) => fields,
            _ => panic!("only support struct with named fields"),
        },
        _ => panic!("only support struct"),
    };
    let mut params_kinds = HashMap::new(); // query、body、path、header ..etc
    for field in &fields.named {
        for attr in &field.attrs {
            match &attr.meta {
                syn::Meta::List(meta) if meta.path.is_ident("api") => {
                    let args= meta.parse_args_with(syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated).expect("parse api args error");
                    let mut arg_fields = HashMap::new();
                    let mut field_kind = ParamKind::Unknown;
                    for arg in args {
                        match arg {
                            syn::Expr::Assign(expr) => {
                                if let syn::Expr::Path(path) = expr.left.as_ref() {
                                    let var_name = path
                                        .path
                                        .segments
                                        .first()
                                        .expect("left value must be var")
                                        .ident
                                        .to_string();
                                    if var_name == "kind" {
                                        if field_kind != ParamKind::Unknown {
                                            panic!("api attr only support one kind")
                                        } else if let syn::Expr::Lit(lit) = *expr.right.clone() {
                                            if let syn::Lit::Str(s) = &lit.lit {
                                                field_kind = s.value().into();
                                            } else {
                                                panic!(
                                                    "api kind attr right val only support string"
                                                )
                                            }
                                        } else {
                                            panic!("api kind attr right val only support literal")
                                        }
                                    };
                                    if arg_fields.contains_key(&var_name) {
                                        panic!(
                                            "api attr only support one name, field: {}",
                                            var_name
                                        );
                                    }
                                    arg_fields.insert(var_name, expr.right);
                                } else {
                                    panic!("api attr left value only support var")
                                }
                            }
                            _ => panic!("api attr only support assign expr"),
                        }
                    }
                    if field_kind == ParamKind::Unknown {
                        panic!("api attr must have kind")
                    }
                    params_kinds
                        .entry(field_kind)
                        .or_insert(vec![])
                        .push((field, arg_fields));
                }
                _ => continue,
            }
        }
    }

    let ident = input.ident;

    let mut path = vec![];
    let mut stream = vec![];
    // let mut header = vec![quote! {let mut header = vec![];}];
    let mut query = vec![];
    let mut body_struct = None;
    let mut is_need_body = false;
    let mut body_construct = None;
    let body_empty_name = format_ident!("{}BodyEmpty", ident);
    let body_name = format_ident!("{}Body", ident);
    let mut is_stream_req = false;

    for (kind, fields) in params_kinds {
        match kind {
            ParamKind::Query => {
                let mut names = HashSet::new();
                query.push(quote! {let mut query = vec![];});
                for (field, args) in fields {
                    let name = must_string(args.get("name").expect("name must be set"));
                    let option_tag = must_string(args.get("option").expect("option must be set"));
                    let value_type =
                        must_string(args.get("v_type").expect("v_type must be set")).into();
                    if names.contains(&name) {
                        panic!("query name must be unique, name: {}", name)
                    }
                    names.insert(name.clone());
                    let field_name = field.ident.as_ref();

                    match value_type {
                        StructValueType::List => {
                            let content = if option_tag.as_str() == "true" {
                                quote! {
                                    if let Some(v) = &self.#field_name {
                                        query.push((#name.to_string(), v.iter().map(|item| item.to_string()).collect::<Vec<String>>().join(",")));
                                    }
                                }
                            } else {
                                quote! {
                                    query.push((#name.to_string(), self.#field_name.iter().map(|item| item.to_string()).collect::<Vec<String>>().join(",")));
                                }
                            };

                            query.push(content);
                        }
                        StructValueType::Var => {
                            let content = if option_tag.as_str() == "true" {
                                quote! {
                                    if let Some(v) = &self.#field_name {
                                        query.push((#name.to_string(), v.to_string()));
                                    }
                                }
                            } else {
                                quote! {
                                    query.push((#name.to_string(), self.#field_name.to_string()));

                                }
                            };
                            query.push(content);
                        }
                    }
                }
            }
            ParamKind::Body => {
                is_need_body = true;
                let field_names = fields.iter().map(|(field, _)| field.ident.as_ref());
                let fields_type = fields.iter().map(|(field, _)| field.ty.clone());
                let names = fields
                    .iter()
                    .map(|(_, args)| must_string(args.get("name").expect("name must be set")));
                body_struct = Some(quote! {
                    #[derive(serde::Serialize,serde::Deserialize,Debug,Clone,Default)]
                    #[serde(default)]
                    pub(crate) struct #body_name{
                        #(#[serde(rename = #names)] #field_names: #fields_type,)*
                    }
                });
                let field_names = fields.iter().map(|(field, _)| field.ident.as_ref());
                body_construct = Some(quote! {
                    #body_name{
                        #(#field_names: self.#field_names,)*
                    }
                });
            }
            ParamKind::Path => {
                let mut names = HashSet::new();
                path.push(quote! {let mut path = std::collections::HashMap::new();});
                for (field, args) in fields {
                    let name = must_string(args.get("name").expect("name must be set"));
                    if names.contains(&name) {
                        panic!("path name must be unique, name: {}", name)
                    }
                    names.insert(name.clone());
                    let field_name = field.ident.as_ref();
                    path.push(quote! {
                        path.insert(#name.to_string(), self.#field_name.to_string());
                    });
                }
            }
            ParamKind::Header => unimplemented!(),
            ParamKind::Stream => {
                is_stream_req = true;
                let mut names = HashSet::new();
                for (field, args) in fields {
                    let name = must_string(args.get("name").expect("name must be set"));
                    if names.contains(&name) {
                        panic!("stream name must be unique, name: {}", name)
                    }
                    names.insert(name.clone());
                    if name == "#data#" {
                        let field_name = field.ident.as_ref();
                        stream.push(quote! {
                            let data = Box::new(self.#field_name);
                        });
                        continue;
                    }

                    let option_tag = must_string(args.get("option").expect("option must be set"));
                    if option_tag.as_str() == "false" {
                        let field_name = field.ident.as_ref();
                        stream.push(quote! {
                            field.insert(#name.to_string(), self.#field_name.to_string());
                        });
                    } else {
                        let field_name = field.ident.as_ref();
                        stream.push(quote! {
                            if let Some(v) = &self.#field_name {
                                field.insert(#name.to_string(), v.to_string());
                            }
                        });
                    }
                }
            }
            ParamKind::Unknown => unreachable!(),
        }
    }

    let path_v = if !path.is_empty() {
        quote! {Some(path)}
    } else {
        quote! {None}
    };

    let query_v = if !query.is_empty() {
        quote! {Some(query)}
    } else {
        quote! {None}
    };

    let empty_body = quote! { #[derive(serde::Serialize,serde::Deserialize,Debug,Clone,Default)] pub(crate) struct #body_empty_name{}};
    let body_type = if is_need_body {
        quote! {#body_name}
    } else {
        quote! {#body_empty_name}
    };
    let body_v = if is_need_body {
        quote! {Some(#body_construct)}
    } else {
        quote! {None}
    };

    let req_type = if is_stream_req {
        quote! { impl<Data: StreamReqData> #ident<Data>}
    } else {
        quote! { impl #ident }
    };

    let expanded = quote! {
        #empty_body

        #body_struct

        #req_type {
            pub fn gen_param(self) -> ReqParam<#body_type> {
                #(#path)*
                #(#query)*
                ReqParam {
                    query: #query_v,
                    path: #path_v,
                    body: #body_v,
                }
            }

            pub fn gen_stream_param(self) -> Option<StreamReqParam> {
                let mut field = std::collections::HashMap::new();
                let mut data = Box::new(tokio::io::empty());
                #(#stream)*
                Some(
                    StreamReqParam {
                        field,
                        data,
                    }
                )
            }
        }
    };
    TokenStream::from(expanded)
}

fn must_string(expr: &syn::Expr) -> String {
    if let syn::Expr::Lit(lit) = expr {
        if let syn::Lit::Str(s) = &lit.lit {
            return s.value();
        }
    }
    panic!("must be string");
}

/// 用于给返回结构体添加 `HasBaseResp` 这个 trait，需要返回结构体中有类型为 `BaseResp` 的 base 字段。注意 base 需要添加 #\[serde(flatten)\] 这个属性。
/// # Examples
///
/// ```
/// use lark_bot_sdk::api::{BaseResp, HasBaseResp};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk::macros::ApiBaseResp)]
/// struct Resp {
///     #[serde(flatten)]
///     pub base: BaseResp,
/// }
/// ```
#[proc_macro_derive(ApiBaseResp)]
pub fn api_base_resp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let expanded = quote! {
        impl HasBaseResp for #ident {
            fn base_resp(&self) -> &BaseResp{
                &self.base
            }

            fn take_base_resp(self) -> BaseResp{
                self.base
            }
        }
    };
    TokenStream::from(expanded)
}
