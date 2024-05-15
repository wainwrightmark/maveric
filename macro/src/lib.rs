use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{self};

#[proc_macro_derive(MavericContext)]
pub fn maveric_context_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_maveric_context(&ast)
}

fn impl_maveric_context(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    quote!(
        #[automatically_derived]
        impl MavericContext for #name {}
    )
    .into()
}

#[proc_macro_derive(MavericRoot)]
pub fn maveric_root_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_maveric_root(&ast)
}

fn impl_maveric_root(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    quote!(
        #[automatically_derived]
        impl MavericRoot for #name {
            type ContextParam<'c> = <<Self as maveric::prelude::MavericRootChildren>::Context as maveric::prelude::NodeContext>::Wrapper<'c>;

            fn get_context<'a, 'w, 's>(
                param: bevy::ecs::system::StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
            ) -> <<Self as MavericRootChildren>::Context as maveric::prelude::NodeContext>::Wrapper<'w> {
                param.into_inner()
            }
        }

    ).into()
}

#[proc_macro_derive(NodeContext)]
pub fn node_context_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_node_context(&ast)
}

fn impl_node_context(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let data_struct: &syn::DataStruct = match &ast.data {
        syn::Data::Struct(s) => s,
        syn::Data::Enum(_) => panic!("Node context can only be derived for structs"),
        syn::Data::Union(_) => panic!("Node context can only be derived for unions"),
    };

    let fields_named = match &data_struct.fields {
        syn::Fields::Named(fd) => fd,
        syn::Fields::Unnamed(_) => {
            panic!("Node context can only be derived for structs with named fields")
        }
        syn::Fields::Unit => {
            panic!("Node context can only be derived for structs with named fields")
        }
    };

    let wrapper_name = format_ident!("{name}Wrapper");
    let visibility = &ast.vis;

    let wrapper_fields = fields_named.named.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;
        quote!(pub #field_name: <#field_type as maveric::node_context::NodeContext>::Wrapper<'w> )
    });

    let has_changed = fields_named.named.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        quote!(maveric::has_changed::HasChanged::has_changed(&self.#field_name) )
    });

    quote!(

            #[derive(bevy::ecs::system::SystemParam)]
            #visibility struct #wrapper_name<'w>{
                #(#wrapper_fields),*
            }


            #[automatically_derived]
            impl maveric::node_context::NodeContext for #name {
                type Wrapper<'c> = #wrapper_name<'c>;

                // fn has_changed(wrapper: &Self::Wrapper<'_>) -> bool {
                //     #(#has_changed)||*
                // }
            }

            #[automatically_derived]
            impl<'c> maveric::has_changed::HasChanged for #wrapper_name<'c>
            {
                fn has_changed(&self) -> bool {
                    #(#has_changed)||*
                }
            }

        )
    .into()
}
