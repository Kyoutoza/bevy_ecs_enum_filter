use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Data, DeriveInput, Path, PathSegment, Token,
};

/// Derive the `EnumFilter` trait on the given enum.
///
/// This will do a couple things:
/// 1. It will, of course, implement the `EnumFilter` trait
/// 2. It will generate a module with filter components for each enum variant
///
/// The generated module will have the name of the enum (snake-cased), appended by
/// `_filters`. So the enum, `MyEnum`, would generate a module called `my_enum_filters`.
///
/// The module will contain a zero-sized marker component struct for each variant.
/// For example, given the following enum:
///
/// ```
/// enum Foo {
///     Bar,
///     Baz(i32),
/// }
/// ```
///
/// We would end up generating the module `foo_filters` which contains the markers `Bar` and `Baz`.
///
/// See the [`Enum!`] macro for how to properly use this generated module.
#[proc_macro_derive(EnumFilter)]
pub fn derive_enum_filter(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let data = match input.data {
        Data::Enum(data) => data,
        Data::Struct(data) => {
            return syn::Error::new(data.struct_token.span, "Cannot derive `EnumTrait` on struct type")
                .into_compile_error()
                .into();
        }
        Data::Union(data) => {
            return syn::Error::new(data.union_token.span, "Cannot derive `EnumTrait` on union type")
                .into_compile_error()
                .into();
        }
    };

    let vis = &input.vis;
    let ident = &input.ident;
    let mod_ident = get_mod_ident(ident);
    let bevy_ecs_enum_filter = get_crate("bevy_ecs_enum_filter");
    let bevy = {
        #[cfg(not(feature = "bevy"))]
        {
            get_crate("bevy_ecs")
        }
        #[cfg(feature = "bevy")]
        {
            get_crate("bevy")
        }
    };

    let variants = data.variants.iter().map(|variant| &variant.ident).collect::<Vec<_>>();

    let docs = variants.iter().map(|variant| {
        format!("Marker component generated for [`{}::{}`][super::{}::{}]", ident, variant, ident, variant)
    });
    let mod_doc = format!(
        "Auto-generated module containing marker components for each variant of [`{}`][super::{}]",
        ident, ident
    );

    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let inner_insert = variants.iter().fold(vec![], |mut list, variant| {
        list.push(quote! {
            #ident::#variant => entity_mut.insert(#mod_ident::#variant)
        });

        list
    });

    let inner_remove = variants.iter().fold(vec![], |mut list, variant| {
        list.push(quote! {
            #ident::#variant => cmd.remove::<#mod_ident::#variant>()
        });

        list
    });

    #[cfg(not(feature = "bevy"))]
    let impl_component = {
        quote! {
            impl #impl_generics #bevy::component::Component for #ident #ty_generics #where_clause {
                const STORAGE_TYPE: #bevy::component::StorageType = #bevy::component::StorageType::Table;
                type Mutability = #bevy::component::Mutable;

                fn register_component_hooks(hooks: &mut #bevy::component::ComponentHooks) {
                    hooks
                        .on_insert(|mut world, ctx| {
                            let enum_comp = world.get::<#ident>(ctx.entity).unwrap().clone();
                            let mut cmd = world.commands();
                            cmd.queue(move |world: &mut #bevy::prelude::World| {
                                let mut entity_mut = world.entity_mut(ctx.entity);
                                match enum_comp {
                                    #(#inner_insert),*
                                };
                            })
                        })
                        .on_replace(|mut world, ctx| {
                            let enum_comp = world.get::<#ident>(ctx.entity).unwrap().clone();
                            let mut cmd = world.commands();
                            let mut cmd = cmd.entity(ctx.entity);
                            match enum_comp {
                                #(#inner_remove),*
                            };
                        })
                        .on_remove(|mut world, ctx| {
                            let enum_comp = world.get::<#ident>(ctx.entity).unwrap().clone();
                            let mut cmd = world.commands();
                            let mut cmd = cmd.entity(ctx.entity);
                            match enum_comp {
                                #(#inner_remove),*
                            };
                        });
                }
            }
        }
    };
    #[cfg(feature = "bevy")]
    let impl_component = {
        quote! {
            impl #impl_generics #bevy::ecs::component::Component for #ident #ty_generics #where_clause {
                const STORAGE_TYPE: #bevy::ecs::component::StorageType = #bevy::ecs::component::StorageType::Table;
                type Mutability = #bevy::ecs::component::Mutable;

                fn register_component_hooks(hooks: &mut #bevy::ecs::component::ComponentHooks) {
                    hooks
                        .on_insert(|mut world, ctx| {
                            let enum_comp = world.get::<#ident>(ctx.entity).unwrap().clone();
                            let mut cmd = world.commands();
                            cmd.queue(move |world: &mut #bevy::prelude::World| {
                                let mut entity_mut = world.entity_mut(ctx.entity);
                                match enum_comp {
                                    #(#inner_insert),*
                                };
                            })
                        })
                        .on_replace(|mut world, ctx| {
                            let enum_comp = world.get::<#ident>(ctx.entity).unwrap().clone();
                            let mut cmd = world.commands();
                            let mut cmd = cmd.entity(ctx.entity);
                            match enum_comp {
                                #(#inner_remove),*
                            };
                        })
                        .on_remove(|mut world, ctx| {
                            let enum_comp = world.get::<#ident>(ctx.entity).unwrap().clone();
                            let mut cmd = world.commands();
                            let mut cmd = cmd.entity(ctx.entity);
                            match enum_comp {
                                #(#inner_remove),*
                            };
                        });
                }
            }
        }
    };

    TokenStream::from(quote! {
        #impl_component
        impl #impl_generics #bevy_ecs_enum_filter::EnumFilter for #ident #ty_generics #where_clause {}

        #[doc = #mod_doc]
        #[doc(hidden)]
        #vis mod #mod_ident {
            #(
                #[doc = #docs]
                #[doc(hidden)]
                #[derive(#bevy::prelude::Component)]
                pub struct #variants;
            )*
        }
    })
}

/// This macro can be used to retrieve the marker component generated by the [`EnumFilter`] derive for
/// the given enum value.
///
/// Because this macro relies on the module generated by the [`EnumFilter`] derive macro, you must
/// make sure it is in scope. Otherwise, you'll likely run into a compile error.
///
/// # Example
///
/// The basic usage of this macro looks like this:
///
/// ```ignore
/// type Marker = Enum!(Enum::Variant);
/// // or, Enum!(path::to::Enum::Variant)
/// ```
///
/// > Note: It doesn't matter whether `Enum::Variant` is a unit, tuple, or struct variant—
/// > you do __not__ need to specify any fields. Treat all variants like a unit variant.
///
/// ```ignore
/// // Make sure everything is in scope
/// use path::to::{Foo, foo_filters};
/// type Marker = Enum!(Foo::Baz);
/// ```
///
/// [`EnumFilter`]: derive@EnumFilter
#[allow(non_snake_case)]
#[proc_macro]
pub fn Enum(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Path);

    let path_len = input.segments.len();

    if path_len < 2 {
        return syn::Error::new(input.span(), "expected a valid enum expression (i.e. `Foo::Bar`)")
            .into_compile_error()
            .into();
    }

    let ident = input.segments[path_len - 2].ident.clone();
    let variant = input.segments[path_len - 1].ident.clone();
    let path_prefix =
        Punctuated::<PathSegment, Token![::]>::from_iter(input.segments.iter().take(path_len - 2).cloned());

    let mod_ident = get_mod_ident(&ident);

    let mod_path = if path_prefix.is_empty() { quote!(#mod_ident) } else { quote!(#path_prefix::#mod_ident) };

    TokenStream::from(quote! {
        #mod_path::#variant
    })
}

fn get_mod_ident(enum_ident: &Ident) -> Ident {
    format_ident!("{}_filters", enum_ident.to_string().to_case(Case::Snake))
}

fn get_crate(name: &str) -> proc_macro2::TokenStream {
    let found_crate = crate_name(name).unwrap_or_else(|_e| panic!("`{}` is present in `Cargo.toml`", name));

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, proc_macro2::Span::call_site());
            // let ident = format_ident!("{}", &name);
            quote!( #ident )
        }
    }
}

/*
       impl #impl_generics #bevy_ecs_enum_filter::EnumFilter for #ident #ty_generics #where_clause {
           fn set_marker(cmd: &mut #bevy::prelude::EntityCommands, value: &Self) {
               #(if matches!(value, #ident::#variants{..}) {
                   let entity = cmd.id();
                   let mut commands = cmd.commands();

                   commands.queue(move |world: &mut #bevy::prelude::World| {
                       let mut entity_mut = world.entity_mut(entity);
                       if !entity_mut.contains::<#mod_ident::#variants>() {
                           // Only insert the marker if it doesn't already exist
                           entity_mut.insert(#mod_ident::#variants);
                       }
                   });
               } else {
                   cmd.remove::<#mod_ident::#variants>();
               })*
           }

           fn remove_marker(cmd: &mut #bevy::prelude::EntityCommands) {
               #(cmd.remove::<#mod_ident::#variants>();)*
           }
       }
*/
