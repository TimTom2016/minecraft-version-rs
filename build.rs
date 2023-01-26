use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
struct VersionCollection {
    versions: Vec<Version>
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

    let f = File::options().create(true).write(true).append(false).open("./src/gen.rs")?;
    let mut bw = BufWriter::new(f);
    use std::io::Write;

    writeln!(&mut bw, "#[non_exhaustive]")?;
    writeln!(&mut bw, "#[allow(non_camel_case_types)]")?;
    writeln!(&mut bw, "#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]")?;
    writeln!(&mut bw, "pub enum MinecraftVersion {{")?;
    let make = |s: &String| s.replace(['.', '-', ' '], "_");
    let s = col
        .iter()
        .map(|x|
            format!("    _{v},\n", v = make(&x.id))
        )
        .collect::<Vec<_>>()
        .join("");
    write!(&mut bw, "{}", s)?;
    writeln!(&mut bw, "}}")?;

    writeln!(&mut bw)?;

    writeln!(&mut bw, "impl core::fmt::Display for MinecraftVersion {{")?;
    writeln!(&mut bw, "    #[allow(clippy::too_many_lines)]")?;
    writeln!(&mut bw, "    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{")?;
    writeln!(&mut bw, "        let s = match self {{")?;
    let s = col
        .iter()
        .map(|x| format!("            Self::_{case} => \"{value}\", \n", case = make(&x.id), value = &x.id))
        .collect::<Vec<_>>()
        .join("");
    write!(&mut bw, "{}", s)?;
    writeln!(&mut bw, "        }};")?;
    writeln!(&mut bw, "    ")?;
    writeln!(&mut bw, "    f.write_str(s)")?;
    writeln!(&mut bw, "    }}")?;
    writeln!(&mut bw, "}}")?;

    writeln!(&mut bw)?;

    writeln!(&mut bw, "impl core::str::FromStr for MinecraftVersion {{")?;
    writeln!(&mut bw, "    type Err = ();")?;
    writeln!(&mut bw, "    #[allow(clippy::too_many_lines)]")?;
    writeln!(&mut bw, "    fn from_str(s: &str) -> Result<Self, Self::Err> {{")?;
    writeln!(&mut bw, "        match s {{")?;

    let s = col
        .iter()
        .map(|x| format!("            \"{ver}\" => Ok(Self::_{variant}),\n", ver = &x.id, variant = make(&x.id)))
        .collect::<Vec<_>>()
        .join("");

    write!(&mut bw, "{}", s)?;
    write!(&mut bw, "            _ => Err(()),")?;
    writeln!(&mut bw, "        }}")?;
    writeln!(&mut bw, "    }}")?;
    writeln!(&mut bw, "}}")?;

    Ok(())
}
