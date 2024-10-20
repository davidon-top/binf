use proc_macro::TokenStream;
use proc_macro2_diagnostics::Diagnostic;

/// Attribute proc macro that turns a struct into a bitflag
/// The underlying type of the bitflag is chosen based on the number of fields in the struct
/// usage:
/// ```ignore
/// use binf::bitflag;
///
/// #[bitflag]
/// pub struct MyBitflag {
///     a: bool,
///     b: bool,
///     c: bool,
/// }
///
/// fn main() {
///     let mut flag = MyBitflag::new(0);
///     flag.set_a(true);
///     let a = flag.a();
/// }
/// ```
/// You should also be able to add derive macros on the struct because the macro copies all attributes to the new struct
/// so you should be able to derive Serialize for example its just that in serialized data you will see the underlying type (unsigned number)
/// you can also call functions from BitFlag trait beacause the new struct implements deref and deref_mut to the underlying type
#[proc_macro_attribute]
pub fn bitflag(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut diagnostics = Vec::new();

    let struct_def: syn::ItemStruct = syn::parse_macro_input!(input as syn::ItemStruct);
    check_struct(&mut diagnostics, &struct_def);

    let struct_name = struct_def.ident;
    let struct_fields = match &struct_def.fields {
        syn::Fields::Named(f) => f
            .named
            .iter()
            .map(|f| f.ident.clone().unwrap())
            .collect::<Vec<_>>(),
        _ => {
            diagnostics.push(
                syn::Error::new_spanned(
                    &struct_def.fields,
                    "bitflags: struct has incorrect shape, only works on struct with named fields",
                )
                .into(),
            );
            Vec::new()
        }
    };

    let vis = struct_def.vis.clone();
    let attrs = struct_def.attrs.clone();

    // unsigned int type of size larger than the number of fields in the struct
    let u_type = {
        let fields_len = struct_fields.len();
        match fields_len {
            0..=8 => quote::quote! { u8 },
            9..=16 => quote::quote! { u16 },
            17..=32 => quote::quote! { u32 },
            33..=64 => quote::quote! { u64 },
            65..=128 => quote::quote! { u128 },
            _ => {
                diagnostics.push(
                    syn::Error::new_spanned(struct_def.fields, "bitflags: struct has too many fields").into(),
                );
                quote::quote! { u128 }
            }
        }
    };

    let newstruct = quote::quote! {
        #(#attrs)*
        #vis struct #struct_name {
            value: #u_type
        }
    };

    // make functions for each field in the struct
    let mut functions = Vec::new();
    for (i, field) in struct_fields.iter().enumerate() {
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
        impl #struct_name {
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
        impl std::ops::Deref for #struct_name {
            type Target = #u_type;
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }
    };

    let deref_mut_impl = quote::quote! {
        impl std::ops::DerefMut for #struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.value
            }
        }
    };

    let diagnostics = diagnostics.iter().map(|d| d.clone().emit_as_item_tokens());
    quote::quote! {
        #(#diagnostics)*
        #newstruct
        #impls
        #deref_impl
        #deref_mut_impl
    }
    .into()
}

/// returns true if the struct has a correct shape
fn check_struct(diagnostics: &mut Vec<Diagnostic>, input: &syn::ItemStruct) {
    if input.generics.lt_token.is_some() || input.generics.gt_token.is_some() || input.generics.where_clause.is_some() || !input.generics.params.is_empty() { 
        diagnostics.push(
            syn::Error::new_spanned(&input.generics, "generics not allowed in bitflag structs")
                .into(),
        );
    }

    match &input.fields {
        syn::Fields::Unnamed(f) => diagnostics.push(
            syn::Error::new_spanned(f, "bitflags: struct has incorrect shape, found tuple struct").into(),
        ),
        syn::Fields::Unit => diagnostics.push(
            syn::Error::new_spanned(input, "bitflags: struct has incorrect shape, found unit struct").into(),
        ),
        syn::Fields::Named(f) => {
            if f.named.len() > 128 {
                diagnostics.push(syn::Error::new_spanned(f, "bitflags: struct has too many fields").into());
            }
            for field in f.named.iter() {
                match &field.ty {
                    syn::Type::Path(path_type) if path_type.path.is_ident("bool") => {}
                    _ => diagnostics.push(
                        syn::Error::new_spanned(
                            field,
                            "bitflags: struct has incorrect shape, found non bool field",
                        )
                        .into(),
                    ),
                }
            }
        }
    }
}
