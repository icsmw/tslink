use std::ops::Deref;

use crate::{
    context::Context,
    error::E,
    nature::{Composite, Nature, Refered, RustTypeName, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, ReturnType};
use syn::{Block, ImplItemFn, ItemFn};

pub enum FnItem<'a> {
    ItemFn(&'a mut ItemFn),
    ImplItemFn(&'a mut ImplItemFn),
}

impl<'a> FnItem<'a> {
    pub fn get_block(&self) -> &Block {
        match self {
            Self::ImplItemFn(item) => &item.block,
            Self::ItemFn(item) => &item.block,
        }
    }

    pub fn set_block(&mut self, block: Block) {
        match self {
            Self::ImplItemFn(item) => {
                item.block = block;
            }
            Self::ItemFn(item) => {
                item.block = Box::new(block);
            }
        }
    }

    pub fn set_output(&mut self, output: ReturnType) {
        match self {
            Self::ImplItemFn(item) => {
                item.sig.output = output;
            }
            Self::ItemFn(item) => {
                item.sig.output = output;
            }
        }
    }
}

fn split_fn_out(out: &Option<Box<Nature>>) -> (Option<Nature>, Option<Nature>) {
    if let Some(out) = out {
        if let Nature::Composite(Composite::Result(res, err)) = out.deref() {
            (
                res.as_ref().map(|n| n.deref().clone()),
                err.as_ref().map(|n| n.deref().clone()),
            )
        } else {
            (Some(out.deref().clone()), None)
        }
    } else {
        (None, None)
    }
}
fn bind(item: &mut FnItem, name: &str, context: &Context, fn_nature: &Nature) -> Result<(), E> {
    let (args, out) = if let Nature::Composite(Composite::Func(args, out, _, _)) = fn_nature {
        (args, out)
    } else {
        return Err(E::Parsing(format!("Fail to parse fn/method \"{name}\"")));
    };
    let bindings = context.get_bound_args();
    let mut unknown: Vec<String> = vec![];
    bindings.iter().for_each(|(name, _)| {
        if !args.iter().any(|nature| {
            if let Nature::Refered(Refered::FuncArg(n, _, _, _)) = nature {
                name == n
            } else {
                false
            }
        }) && name != "result"
        {
            unknown.push(name.to_owned());
        }
    });
    if !unknown.is_empty() {
        return Err(E::Parsing(format!(
            "Unknown arguments to bind: {}",
            unknown.join(", ")
        )));
    }
    let (fn_res, fn_err) = split_fn_out(out);
    let fn_err_type_ref = if let Some(fn_err) = fn_err.as_ref() {
        Some(format_ident!("{}", fn_err.rust_type_name()?))
    } else {
        None
    };
    let bindings = bindings
        .iter()
        .map(|(name, ref_name)| {
            let varname = format_ident!("{}", name);
            let refname = format_ident!("{}", ref_name);
            if let Some(fn_err_type_ref) = fn_err_type_ref.as_ref() {
                quote! {
                    #[allow(unused_mut)]
                    let mut #varname: #refname = serde_json::from_str(&#varname).map_err(|e| Into::<#fn_err_type_ref>::into(e))?
                }
            } else {
                quote! {
                    #[allow(unused_mut)]
                    let mut #varname: #refname = serde_json::from_str(&#varname).expect("Parsing from JSON string")?
                }
            }
        })
        .collect::<Vec<TokenStream>>();
    if !bindings.is_empty() {
        let stmts = &item.get_block().stmts;
        let block = quote! {
            use serde_json;
            #(#bindings)*;
            #(#stmts)*
        };
        item.set_block(parse_quote! {{#block}});
    }
    if context.result_as_json()? {
        if let (Some(fn_res), Some(fn_err)) = (fn_res, fn_err) {
            let res_rust_type = format_ident!("{}", fn_res.rust_type_name()?);
            let err_rust_type = format_ident!("{}", fn_err.rust_type_name()?);
            let error_as_json = context.error_as_json()?;
            let res_token = fn_res
                .variable_token_stream("res", if error_as_json { None } else { Some(&fn_err) })?;
            let err_token = if error_as_json {
                fn_err.variable_token_stream("err", None)?
            } else {
                let err = format_ident!("{}", "err");
                quote! {#err}
            };
            let stmts = &item.get_block().stmts;
            let block = if error_as_json {
                quote! {
                    let result: Result<#res_rust_type, #err_rust_type> = (move || {
                        #(#stmts)*
                    })();
                    match result {
                        Ok(res) => Ok(#res_token),
                        Err(err) => Err(#err_token)
                    }
                }
            } else {
                quote! {
                    match {#(#stmts)*} {
                        Ok(res) => Ok(#res_token),
                        Err(err) => Err(#err_token)
                    }
                }
            };
            println!(">>>>>>>>>>>>>>>>>>>>> {block}");
            item.set_block(parse_quote! {{#block}});
            let output = if error_as_json {
                quote! { -> Result<String, String>}
            } else {
                quote! { -> Result<String, #err_rust_type>}
            };
            println!(">>>>>>>>>>>>>>>>>>>>> {output}");
            item.set_output(parse_quote! {#output});
        }
    }
    Ok(())
}

pub fn bind_fn(
    item: &mut ItemFn,
    name: &str,
    context: &Context,
    fn_nature: &Nature,
) -> Result<(), E> {
    bind(&mut FnItem::ItemFn(item), name, context, fn_nature)
}

pub fn bind_impl_fn(
    item: &mut ImplItemFn,
    name: &str,
    context: &Context,
    fn_nature: &Nature,
) -> Result<(), E> {
    bind(&mut FnItem::ImplItemFn(item), name, context, fn_nature)
}
