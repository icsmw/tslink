use std::ops::Deref;

use crate::{
    context::Context,
    error::E,
    nature::{Composite, Extract, Nature, Refered, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Block, Fields, ImplItem, ImplItemFn, ItemFn, ReturnType, Stmt};

pub fn is_method(item_fn: &ItemFn) -> bool {
    item_fn
        .sig
        .inputs
        .iter()
        .find(|input| matches!(input, syn::FnArg::Receiver(_)))
        .is_some()
}

// pub fn is_method(item_fn: &ItemFn, context: Context) -> Result<bool, E> {
//     if item_fn
//         .sig
//         .inputs
//         .iter()
//         .find(|input| matches!(input, syn::FnArg::Receiver(_)))
//         .is_some()
//     {
//         Ok(true)
//     } else {
//         Ok(Nature::extract(item_fn, context)?.is_self_returned())
//     }
// }

pub fn read_fields(fields: &Fields, parent: &mut Nature, parent_context: Context) -> Result<(), E> {
    match fields {
        Fields::Named(ref fields) => {
            for field in fields.named.iter() {
                let mut context = Context::try_from_or_default(&field.attrs)?;
                context.set_parent(parent_context.clone());
                if context.ignore_self() {
                    continue;
                }
                let name = field.ident.clone().unwrap();
                parent.bind(Nature::Refered(Refered::Field(
                    name.to_string(),
                    context.clone(),
                    Box::new(Nature::extract(&field.ty, context.clone())?),
                )))?;
            }
            parent.check_ignored_fields()?;
        }
        _ => {}
    }
    Ok(())
}

pub fn read_impl(
    items: &mut Vec<ImplItem>,
    parent: &mut Nature,
    struct_context: Context,
    _parent_context: Context,
) -> Result<(), E> {
    for item in items.iter_mut() {
        match item {
            ImplItem::Fn(fn_item) => {
                let mut context = Context::try_from_or_default(&fn_item.attrs)?;
                context.set_parent(struct_context.clone());
                if context.ignore_self() {
                    continue;
                }
                let name = fn_item.sig.ident.to_string();
                if context.is_ignored(&name) {
                    continue;
                }
                let fn_nature = Nature::extract(&*fn_item, context.clone())?;
                parent.bind(Nature::Refered(Refered::Field(
                    name.to_string(),
                    context.clone(),
                    Box::new(fn_nature.clone()),
                )))?;
                let (args, out) =
                    if let Nature::Composite(Composite::Func(args, out, _, _)) = fn_nature {
                        (args, out)
                    } else {
                        return Err(E::Parsing(format!("Fail to parse method \"{name}\"")));
                    };
                let bindings = context.get_bound_args();
                let mut unknown: Vec<String> = vec![];
                bindings.iter().for_each(|(name, _)| {
                    if !args.iter().any(|nature| {
                        if let Nature::Refered(Refered::FuncArg(n, _, _)) = nature.deref() {
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
                    let stmts = &fn_item.block.stmts;
                    let block = quote! {
                        use serde_json;
                        #(#bindings)*;
                        #(#stmts)*
                    };
                    fn_item.block = parse_quote! {{#block}};
                }
                if context.result_as_json()? {
                    if let Some(Nature::Composite(Composite::Result(Some(fn_res), Some(fn_err)))) =
                        out.as_deref()
                    {
                        let res_token = fn_res.token_stream("res")?;
                        let err_token = fn_err.token_stream("err")?;
                        let stmts = &fn_item.block.stmts;
                        let block = quote! {
                            match {#(#stmts)*} {
                                Ok(res) => Ok(#res_token),
                                Err(err) => Err(#err_token)
                            }
                        };
                        fn_item.block = parse_quote! {{#block}};
                        let output = quote! { -> Result<String, String>};
                        fn_item.sig.output = parse_quote! {#output};
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
