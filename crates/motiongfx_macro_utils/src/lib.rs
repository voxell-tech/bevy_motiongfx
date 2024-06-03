extern crate proc_macro;

use std::{env, path::PathBuf};

use proc_macro::TokenStream;
use syn::DeriveInput;
use toml_edit::{DocumentMut, Item};

/// The path to the `Cargo.toml` file for the MotionGfx project.
pub struct MotionGfxManifest {
    manifest: DocumentMut,
}

impl Default for MotionGfxManifest {
    fn default() -> Self {
        Self {
            manifest: env::var_os("CARGO_MANIFEST_DIR")
                .map(PathBuf::from)
                .map(|mut path| {
                    path.push("Cargo.toml");
                    if !path.exists() {
                        panic!(
                            "No Cargo manifest found for crate. Expected: {}",
                            path.display()
                        );
                    }
                    let manifest = std::fs::read_to_string(path.clone()).unwrap_or_else(|_| {
                        panic!("Unable to read cargo manifest: {}", path.display())
                    });
                    manifest.parse::<DocumentMut>().unwrap_or_else(|_| {
                        panic!("Failed to parse cargo manifest: {}", path.display())
                    })
                })
                .expect("CARGO_MANIFEST_DIR is not defined."),
        }
    }
}
const BEVY_MOTIONGFX: &str = "bevy_motiongfx";

impl MotionGfxManifest {
    /// Attempt to retrieve the [path](syn::Path) of a particular package in
    /// the [manifest](MotionGfxManifest) by [name](str).
    pub fn maybe_get_path(&self, name: &str) -> Option<syn::Path> {
        fn dep_package(dep: &Item) -> Option<&str> {
            if dep.as_str().is_some() {
                None
            } else {
                dep.get("package").map(|name| {
                    println!("{:?}", name.as_str());
                    name.as_str().unwrap()
                })
            }
        }

        let find_in_deps = |deps: &Item| -> Option<syn::Path> {
            let package = if let Some(dep) = deps.get(name) {
                return Some(Self::parse_str(dep_package(dep).unwrap_or(name)));
            } else if let Some(dep) = deps.get(BEVY_MOTIONGFX) {
                dep_package(dep).unwrap_or(BEVY_MOTIONGFX)
            } else {
                return None;
            };

            let mut path = Self::parse_str::<syn::Path>(package);
            if let Some(module) = name.strip_prefix("motiongfx_") {
                path.segments.push(Self::parse_str(module));
            }
            Some(path)
        };

        let package_name = self
            .manifest
            .get("package")
            .and_then(|p| p.get("name"))
            .unwrap();
        let deps = self.manifest.get("dependencies");
        let deps_dev = self.manifest.get("dev-dependencies");

        // First, check if we are referencing the package itself
        if let Some(package_name) = package_name.as_str() {
            if package_name == name {
                return Some(Self::parse_str("crate"));
            }
        }

        deps.and_then(find_in_deps)
            .or_else(|| deps_dev.and_then(find_in_deps))
    }

    /// Returns the path for the crate with the given name.
    ///
    /// This is a convenience method for constructing a [manifest] and
    /// calling the [`get_path`] method.
    ///
    /// This method should only be used where you just need the path and can't
    /// cache the [manifest]. If caching is possible, it's recommended to create
    /// the [manifest] yourself and use the [`get_path`] method.
    ///
    /// [`get_path`]: Self::get_path
    /// [manifest]: Self
    pub fn get_path_direct(name: &str) -> syn::Path {
        Self::default().get_path(name)
    }

    /// Returns the path for the crate with the given name.
    pub fn get_path(&self, name: &str) -> syn::Path {
        self.maybe_get_path(name)
            .unwrap_or_else(|| Self::parse_str(name))
    }

    /// Attempt to parse the provided [path](str) as a [syntax tree node](syn::parse::Parse)
    pub fn try_parse_str<T: syn::parse::Parse>(path: &str) -> Option<T> {
        syn::parse(path.parse::<TokenStream>().ok()?).ok()
    }

    /// Attempt to parse provided [path](str) as a [syntax tree node](syn::parse::Parse).
    ///
    /// # Panics
    ///
    /// Will panic if the path is not able to be parsed. For a non-panicing option, see [`try_parse_str`]
    ///
    /// [`try_parse_str`]: Self::try_parse_str
    pub fn parse_str<T: syn::parse::Parse>(path: &str) -> T {
        Self::try_parse_str(path).unwrap()
    }

    /// Attempt to get a subcrate [path](syn::Path) under Bevy MotionGfx by [name](str)
    pub fn get_subcrate(&self, subcrate: &str) -> Option<syn::Path> {
        self.maybe_get_path(BEVY_MOTIONGFX)
            .map(|bevy_path| {
                let mut segments = bevy_path.segments;
                segments.push(MotionGfxManifest::parse_str(subcrate));
                syn::Path {
                    leading_colon: None,
                    segments,
                }
            })
            .or_else(|| self.maybe_get_path(&format!("motiongfx_{subcrate}")))
    }
}

/// Attempt to get the one and only field tagged by a given [attribute name](str).
///
/// # Panics
///
/// Will panic if no field is found or more than 1 field is found.
pub fn get_one_field_of_attribute(ast: &DeriveInput, attr_name: &str) -> syn::Ident {
    let syn::Data::Struct(struct_data) = &ast.data else {
        panic!("Can only be implemented on a Struct.");
    };

    let field_filter: Vec<&syn::Field> = struct_data
        .fields
        .iter()
        .filter(|field| {
            field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident(attr_name))
                .count()
                == 1
        })
        .collect();

    if field_filter.len() != 1 {
        panic!(
            "Expected exactly 1 field with #[{}] attribute. Given {}.",
            attr_name,
            field_filter.len()
        );
    }

    let shape_ident = field_filter[0].ident.as_ref().unwrap();
    shape_ident.clone()
}
