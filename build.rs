use chrono::Utc;
use quote::quote;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

#[derive(Deserialize)]
struct VersionCollection {
    versions: Vec<Version>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Version {
    id: String,
    release_time: chrono::DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut col = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .await?
        .json::<VersionCollection>()
        .await?
        .versions;
    col.sort_by_key(|x| x.release_time);
    let versions = col
        .iter()
        .map(|version| {
            let mut id = version.id.replace(['.', '-', ' '], "_");
            id.insert(0, '_');
            let id: proc_macro2::TokenStream = id.parse().unwrap();
            quote! {#id}
        })
        .collect::<Vec<_>>();
    let version_strings = col
        .iter()
        .map(|version| {
            let mut id = version.id.replace(['.', '-', ' '], "_");
            id.insert(0, '_');
            let id: proc_macro2::TokenStream = id.parse().unwrap();
            let id2 = version.id.clone();
            quote! {Self::#id => #id2}
        })
        .collect::<Vec<_>>();
    let strings_versions = col
        .iter()
        .map(|version| {
            let mut id = version.id.replace(['.', '-', ' '], "_");
            id.insert(0, '_');
            let id: proc_macro2::TokenStream = id.parse().unwrap();
            let id2 = version.id.clone();
            quote! {#id2=>  Ok(Self::#id)}
        })
        .collect::<Vec<_>>();
    let tokens = quote! {
        #[non_exhaustive]
        #[allow(non_camel_case_types)]
        #[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
        pub enum MinecraftVersion {
            #(#versions),*
        }

        impl core::fmt::Display for MinecraftVersion {
            #[allow(clippy::too_many_lines)]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let s = match self {
                    #(#version_strings),*
                };
                f.write_str(s)
            }
        }
        impl core::str::FromStr for MinecraftVersion {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#strings_versions),*,
                    _ => Err(())
                }
            }
        }
    };

    let syntax_tree = syn::parse2(tokens).unwrap();
    let f = File::options()
        .create(true)
        .write(true)
        .append(false)
        .open("./src/gen.rs")?;
    let mut bw = BufWriter::new(f);
    write!(&mut bw, "{}", prettyplease::unparse(&syntax_tree))?;
    Ok(())
}
