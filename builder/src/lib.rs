use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;
use proc_macro2::{Ident, Span};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("#liangjianfeng {:#?}", input);

    let struct_name = input.ident;
    let builder_name = Ident::new(&format!("{}Builder", struct_name), Span::call_site());
    println!("struct_name = {:?}", struct_name);
    println!("builder_name = {:?}", builder_name);
    let value = if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(name) = &data_struct.fields {
            name.named.iter()
                .filter_map(|f| {
                    f.ident.as_ref().map(|ident| (ident, &f.ty))
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    } else {
        vec![]
    };
    let builder_init = value.iter().map(|(ident, _ty)| {
        quote! {
            #ident: None
        }
    }).collect::<Vec<_>>();
    let builder_field = value.iter().map(|(ident, ty)| {
        quote! {
            #ident: std::option::Option<#ty>
        }
    }).collect::<Vec<_>>();

    let impl_struct = quote! {
        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_init),*
                }
            }
        }
    };
    println!("impl_struct: {}", impl_struct);

    let struct_builder = quote! {
        struct #builder_name {
            #(#builder_field),*
        }
    };
    println!("struct_builder: {}", struct_builder);

    let builder_field = value.iter().map(|(ident, ty)| {
        quote! {
            fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    }).collect::<Vec<_>>();
    let impl_builder = quote! {
        impl #builder_name {
            #(#builder_field)*
        }
    };

    let output = quote! {
        #struct_builder
        #impl_struct
        #impl_builder
    };
    println!("output: {}", output);
    TokenStream::from(output)
}
