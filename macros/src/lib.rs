use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitflag(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let structdef: syn::ItemStruct = syn::parse_macro_input!(input as syn::ItemStruct);
    // check if the struct has correct shape
    if !check_struct(&structdef) {
        panic!("struct has incorrect shape");
    }
    let structname = structdef.ident;
    let structfields = match &structdef.fields {
        syn::Fields::Named(f) => f.named.iter().map(|f| f.ident.clone().unwrap()).collect::<Vec<_>>(),
        _ => panic!("struct has incorrect shape"),
    };

    let vis = structdef.vis.clone();
    let attrs = structdef.attrs.clone();

    // a unsigned int type of a size larger then the number of fields in the struct
    let u_type = {
        let fields_len = structfields.len();
        match fields_len {
            0..=8 => quote::quote! { u8 },
            9..=16 => quote::quote! { u16 },
            17..=32 => quote::quote! { u32 },
            33..=64 => quote::quote! { u64 },
            65..=128 => quote::quote! { u128 },
            _ => panic!("struct has too many fields"),
        }
    };


    let newstruct: syn::ItemStruct = syn::parse_quote! {
        #(#attrs)*
        #vis struct #structname {
            value: #u_type
        }
    };

    // make functions for each field in the struct
    let mut functions = Vec::new();
    for (i, field) in structfields.iter().enumerate() {
        let field = field.clone();
        let i = i as u8;
    let set_ident = syn::Ident::new(&format!("set_{}", field), field.span());
        functions.push(quote::quote! {
            pub fn #field(&self) -> bool {
                self.value.get_flag(#i)
            }
            pub fn #set_ident(&mut self, value: bool) {
                self.value.set_flag(#i, value);
            }
        });
    }

    let impls = quote::quote! {
        impl #structname {
            #(#functions)*

            pub fn new(val: #u_type) -> Self {
                Self {
                    value: val
                }
            }

            pub fn value(&self) -> #u_type {
                self.value
            }
        }
    };

    let deref_impl = quote::quote! {
        impl std::ops::Deref for #structname {
            type Target = #u_type;
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }
    };

    let deref_mut_impl = quote::quote! {
        impl std::ops::DerefMut for #structname {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.value
            }
        }
    };

    quote::quote! {
        #newstruct
        #impls
        #deref_impl
        #deref_mut_impl
    }.into()
}

/// returns true if the struct has a correct shape
fn check_struct(input: &syn::ItemStruct) -> bool {
    if input.generics.lt_token.is_some() {
        return false;
    }
    if input.generics.gt_token.is_some() {
        return false;
    }
    if input.generics.where_clause.is_some() {
        return false;
    }
    if !input.generics.params.is_empty() {
        return false;
    }

    match &input.fields {
        syn::Fields::Unnamed(_) => false,
        syn::Fields::Unit => false,
        syn::Fields::Named(f) => {
            if f.named.len() > 128 {
                return false;
            }
            true
        },
    }
}
