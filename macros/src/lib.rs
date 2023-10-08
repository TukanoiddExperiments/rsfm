use attribute_derive::Attribute;
use convert_case::Casing;
use manyhow::manyhow;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{spanned::Spanned, Data, DataEnum, DeriveInput, Error, Ident, LitStr, Path, Variant};

/**
Example:
```
#[$crate::register_icons]
enum PhosphorIcon {
    Folder,
    File,
    Link,
    SealWarning
}
```
 */
#[manyhow(proc_macro_attribute)]
pub fn register_icons(attr: TokenStream2, item: TokenStream2) -> syn::Result<TokenStream2> {
    let RI {
        egui_root,
        crate_mod,
        svgs,
    } = syn::parse2(attr)?;
    let svgs_str = svgs.value();

    let item_span = item.span();

    let DeriveInput { ident, data, .. } = syn::parse2(item)?;

    let ident_span = ident.span();
    let ident_str = ident.to_string();
    let mod_name = ident_str
        .strip_suffix("Icon")
        .ok_or(Error::new(
            ident_span,
            "Name enum like this: {name}Icon. Example; PhosphorIcon",
        ))?
        .to_lowercase();
    let mod_name_ident = Ident::new(&mod_name, ident_span);

    let DataEnum { variants, .. } = match data {
        Data::Enum(data_enum) => data_enum,
        _ => return Err(Error::new(item_span, "Only supports enums".to_string())),
    };
    let variant_idents = variants.iter().map(|Variant { ident, .. }| ident);
    let variant_ri_attrs = variants
        .iter()
        .map(|Variant { ident, attrs, .. }| {
            RIInner::from_attributes(attrs).map(|rii| rii.into_non_opt(ident))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let retained_image: Path = syn::parse_quote!(egui_extras::image::RetainedImage);
    let registry_field_idents =
        variant_ri_attrs
            .iter()
            .map(|RIInnerNonOpt { svg_name, .. }| {
                Ident::new(
                    &svg_name.value().to_case(convert_case::Case::Snake),
                    svg_name.span(),
                )
            });

    let icon = {
        let cast_to_symbol = {
            let matches = variant_ri_attrs
            .iter()
            .zip(variant_idents.clone())
            .map(|(RIInnerNonOpt { const_name, .. }, var_ident)| quote!(Self::#var_ident => #crate_mod::#const_name,));

            quote!(
                pub fn symbol(&self) -> &'static str {
                    match self {
                        #(#matches)*
                    }
                }
            )
        };
        let cast_to_image = {
            let matches = variant_idents
                .clone()
                .zip(registry_field_idents.clone())
                .map(|(var_ident, reg_field_ident)| quote!(Self::#var_ident => &REGISTRY.#reg_field_ident,));

            quote!(
                pub fn image(&self) -> &'static #retained_image {
                    match self {
                        #(#matches)*
                    }
                }

                pub fn image_widget(&self, ctx: &#egui_root::Context, size: #egui_root::Vec2) -> #egui_root::Image {
                    #egui_root::Image::new(self.image().texture_id(ctx), size)
                }
            )
        };

        quote!(
            pub enum Icon {
                #(#variant_idents),*
            }

            impl Icon {
                #cast_to_symbol
                #cast_to_image
            }
        )
    };
    let icon_registry = {
        let fields = registry_field_idents
            .clone()
            .map(|ident| quote!(#ident: #retained_image,));
        let load_fields = variant_ri_attrs
            .iter()
            .zip(registry_field_idents.clone())
            .map(|(RIInnerNonOpt { svg_name, .. }, ident)| {
                let svg_name_str = svg_name.value();
                let svg_name_span = svg_name.span();
                let svg_path =
                    LitStr::new(&format!("{svgs_str}/{svg_name_str}.svg"), svg_name_span);
                let expect_msg = LitStr::new(
                    &format!("Failed to load {svg_name_str} icon"),
                    svg_name_span,
                );
                let debug_name = LitStr::new(&format!("{mod_name}-{svg_name_str}"), svg_name_span);

                quote!(
                    #ident: #retained_image::from_color_image(
                        #debug_name,
                        egui_extras::image::load_svg_bytes(include_bytes!(#svg_path))
                            .expect(&format!(#expect_msg))
                    )
                )
            });

        quote! {
            lazy_static::lazy_static! {
                static ref REGISTRY: IconRegistry = IconRegistry::load();
            }

            struct IconRegistry {
                #(#fields)*
            }

            impl IconRegistry {
                fn load() -> Self {
                    Self {
                        #(#load_fields),*
                    }
                }
            }
        }
    };

    Ok(quote!(
        pub type #ident = #mod_name_ident::Icon;

        pub mod #mod_name_ident {
            #icon
            #icon_registry
        }
    ))
}

#[derive(Attribute)]
#[attribute(ident = register_icons)]
struct RI {
    egui_root: Path,
    crate_mod: Path,
    svgs: LitStr,
}

#[derive(Attribute)]
#[attribute(ident = ri)]
struct RIInner {
    #[attribute(optional)]
    const_name: Option<Ident>,
    #[attribute(optional)]
    svg_name: Option<LitStr>,
}

impl RIInner {
    fn into_non_opt(self, ident: &Ident) -> RIInnerNonOpt {
        let ident_span = ident.span();
        let ident = ident.to_string();
        let const_span = match &self.const_name {
            Some(cs) => cs.span(),
            None => ident_span,
        };
        let svg_span = match &self.const_name {
            Some(s) => s.span(),
            None => ident_span,
        };

        RIInnerNonOpt {
            const_name: Ident::new(
                &self
                    .const_name
                    .map(|const_name| const_name.to_string())
                    .unwrap_or(ident.to_case(convert_case::Case::UpperSnake)),
                const_span,
            ),
            svg_name: LitStr::new(
                &self
                    .svg_name
                    .map(|svg_name| svg_name.value())
                    .unwrap_or(ident.to_case(convert_case::Case::Kebab)),
                svg_span,
            ),
        }
    }
}

struct RIInnerNonOpt {
    const_name: Ident,
    svg_name: LitStr,
}
