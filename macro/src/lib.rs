use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Index};

/// Implement `SystemParam` to use a struct as a parameter in a system
#[proc_macro_derive(HasChanged)]
pub fn derive_system_param(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let syn::Data::Struct(syn::DataStruct {
        fields: field_definitions,
        ..
    }) = ast.data
    else {
        return syn::Error::new(
            ast.span(),
            "Invalid `SystemParam` type: expected a `struct`",
        )
        .into_compile_error()
        .into();
    };

    let mut fields = Vec::new();
    let mut field_types = Vec::new();
    for (i, field) in field_definitions.iter().enumerate() {
        let i = Index::from(i);
        fields.push(
            field
                .ident
                .as_ref()
                .map(|f| quote! { #f })
                .unwrap_or_else(|| quote! { #i }),
        );
        field_types.push(&field.ty);
    }

    let generics = ast.generics;

    // Emit an error if there's any unrecognized lifetime names.
    for lt in generics.lifetimes() {
        let ident = &lt.lifetime.ident;
        let w = format_ident!("w");
        let s = format_ident!("s");
        if ident != &w && ident != &s {
            return syn::Error::new_spanned(
                lt,
                r#"invalid lifetime name: expected `'w` or `'s`
 'w -- refers to data stored in the World.
 's -- refers to data stored in the SystemParam's state.'"#,
            )
            .into_compile_error()
            .into();
        }
    }

    let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    let mut has_changed_generics = generics.clone();
    let has_changed_where_clause = has_changed_generics.make_where_clause();
    for field_type in &field_types {
        has_changed_where_clause
            .predicates
            .push(syn::parse_quote!(#field_type: maveric::has_changed::HasChanged));
    }

    let mut has_item_changed_generics = generics.clone();
    let has_item_changed_where_clause = has_item_changed_generics.make_where_clause();
    for field_type in &field_types {
        has_item_changed_where_clause
            .predicates
            .push(syn::parse_quote!(#field_type: maveric::has_item_changed::HasItemChanged));
    }

    let struct_name = &ast.ident;

    let has_changed_impl = fields
        .iter()
        .map(|field| quote!(maveric::has_changed::HasChanged::has_changed(&self.#field) ));

    TokenStream::from(quote! {


        impl #impl_generics maveric::has_changed::HasChanged for #struct_name #ty_generics #has_changed_where_clause
        {
            fn has_changed(&self) -> bool {
                #(#has_changed_impl)||*
            }
        }

        impl #impl_generics maveric::has_item_changed::HasItemChanged for #struct_name #ty_generics #has_item_changed_where_clause
        {
                fn has_item_changed<'w1, 's1>(item: &Self::Item<'w1, 's1>) -> bool {
                    item.has_changed()
                }
        }
    })
}
