use crate::{
    context::Context,
    error::E,
    nature::{Composite, Nature, Refered, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::ops::Deref;
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
            if let Nature::Refered(Refered::FuncArg(n, _, _, _)) = nature.deref() {
                name == n
            } else {
                false
            }
        }) {
            if name != "result" {
                unknown.push(name.to_owned());
            }
        }
    });
    if !unknown.is_empty() {
        return Err(E::Parsing(format!(
            "Unknown arguments to bind: {}",
            unknown.join(", ")
        )));
    }
    let bindings = bindings
    .iter()
    .map(|(name, ref_name)| {
        let varname = format_ident!("{}", name);
        let refname = format_ident!("{}", ref_name);
        quote!{
            #[allow(unused_mut)]
            let mut #varname: #refname = serde_json::from_str(&#varname).map_err(|e| e.to_string())?
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
        if let Some(Nature::Composite(Composite::Result(Some(fn_res), Some(fn_err)))) =
            out.as_deref()
        {
            let res_token = fn_res.token_stream("res")?;
            let err_token = fn_err.token_stream("err")?;
            let stmts = &item.get_block().stmts;
            let block = quote! {
                match {#(#stmts)*} {
                    Ok(res) => Ok(#res_token),
                    Err(err) => Err(#err_token)
                }
            };
            item.set_block(parse_quote! {{#block}});
            let output = quote! { -> Result<String, String>};
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
