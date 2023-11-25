use proc_macro::TokenStream;
use quote::quote;
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

            fn get_context<'a, 'c, 'w: 'c, 's>(
                param: bevy::ecs::system::StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
            ) -> <<Self as MavericRootChildren>::Context as maveric::prelude::NodeContext>::Wrapper<'c> {
                param.into_inner()
            }
        }

    ).into()
}
