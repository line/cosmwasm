use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use syn::Item;

use crate::callable_point;

pub fn make_callable_points(body: Vec<Item>) -> (Vec<TokenTree>, Vec<TokenStream>) {
    let mut list_callable_points = Vec::new();
    let mut maked = Vec::new();

    for i in &body {
        if let syn::Item::Fn(function) = i.clone() {
            let (function_remove_macro, is_strip) = strip_callable_point(function.clone());

            if is_strip {
                let (maked_callable_point, callee_func) =
                    callable_point::make_callable_point(function_remove_macro);
                maked.extend(maked_callable_point);
                list_callable_points.push(callee_func);
            } else {
                maked.extend(quote! {#i});
            }
        } else {
            maked.extend(quote! {#i});
        }
    }

    (maked, list_callable_points)
}

// strip #[callable_point]
pub fn strip_callable_point(function: syn::ItemFn) -> (syn::ItemFn, bool) {
    let mut res = vec![];
    let mut is_strip = false;
    for attr in function.attrs {
        if attr.path.is_ident("callable_point") {
            is_strip = true;
        } else {
            res.push(attr)
        }
    }

    (
        syn::ItemFn {
            attrs: res,
            vis: function.vis,
            sig: function.sig,
            block: function.block,
        },
        is_strip,
    )
}
