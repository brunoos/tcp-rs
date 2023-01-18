extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, Ident};
use quote::{quote};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[proc_macro_attribute]
pub fn lua_cfunction(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::ItemFn);
    let caller = &input.sig.ident.clone();

    let mut name = caller.to_string();
    
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let hash = hasher.finish();

    name.push_str("_");
    name.push_str(hash.to_string().as_str());
    let callee = Ident::new(name.as_str(), caller.span());

    input.sig.ident = callee.clone(); 

    let tokens = quote!{
        #[inline]
        #input

        #[no_mangle]
        pub unsafe extern "C" fn #caller(l: *mut lua::lua_State) -> c_int {
            #callee(l)
        }
    };
    return TokenStream::from(tokens);
}