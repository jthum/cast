use std::{
    env, fs,
    path::{Path, PathBuf},
};

use reqwest::blocking::Client;
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const GOOGLE_FONTS_CONTENTS_API: &str = "https://api.github.com/repos/google/fonts/contents";
const LICENSE_DIRS: [&str; 3] = ["ofl", "apache", "ufl"];

fn main() -> Result<()> {
    let command = Command::parse(env::args().skip(1).collect())?;
    match command {
        Command::GoogleFont(options) => install_google_font(options),
    }
}

enum Command {
    GoogleFont(GoogleFontOptions),
}

impl Command {
    fn parse(args: Vec<String>) -> Result<Self> {
        match args.first().map(String::as_str) {
            Some("google-font") => Ok(Self::GoogleFont(GoogleFontOptions::parse(&args[1..])?)),
            _ => Err(usage().into()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct GoogleFontOptions {
    family: String,
    out: PathBuf,
    weights: Vec<u16>,
    include_italics: bool,
}

impl GoogleFontOptions {
    fn parse(args: &[String]) -> Result<Self> {
        let mut family = None;
        let mut out = None;
        let mut weights = Vec::new();
        let mut include_italics = false;
        let mut index = 0;

        while index < args.len() {
            match args[index].as_str() {
                "--family" => {
                    index += 1;
                    family = args.get(index).cloned();
                }
                "--out" => {
                    index += 1;
                    out = args.get(index).map(PathBuf::from);
                }
                "--weights" => {
                    index += 1;
                    weights = parse_weights(args.get(index).ok_or_else(usage)?)?;
                }
                "--include-italics" => {
                    include_italics = true;
                }
                value if !value.starts_with("--") && family.is_none() => {
                    family = Some(value.to_owned());
                }
                _ => return Err(usage().into()),
            }
            index += 1;
        }

        Ok(Self {
            family: family.ok_or_else(usage)?,
            out: out.ok_or_else(usage)?,
            weights,
            include_italics,
        })
    }
}

fn usage() -> String {
    "Usage: cargo run -p cast-font-tool -- google-font <family> --out <dir> [--weights 400,500,600] [--include-italics]".to_owned()
}

fn install_google_font(options: GoogleFontOptions) -> Result<()> {
    let client = Client::builder().user_agent("cast-font-tool").build()?;
    let slug = google_fonts_slug(&options.family);
    let family = find_google_font_family(&client, &slug)?.ok_or_else(|| {
        format!(
            "Font family {:?} was not found in google/fonts",
            options.family
        )
    })?;
    let fonts = select_fonts(&family.files, &options.weights, options.include_italics);

    if fonts.is_empty() {
        return Err(format!(
            "Font family {:?} did not expose matching TTF files",
            options.family
        )
        .into());
    }

    fs::create_dir_all(&options.out)?;
    for font in fonts {
        download_file(&client, font, &options.out)?;
    }

    if let Some(license) = family.license_file.as_ref() {
        download_file_as(&client, license, &options.out.join("LICENSE.txt"))?;
    }

    println!(
        "Installed {} from google/fonts/{}/{} into {}",
        options.family,
        family.license_dir,
        slug,
        options.out.display()
    );
    Ok(())
}

fn find_google_font_family(client: &Client, slug: &str) -> Result<Option<GoogleFontFamily>> {
    for license_dir in LICENSE_DIRS {
        let url = format!("{GOOGLE_FONTS_CONTENTS_API}/{license_dir}/{slug}?ref=main");
        if let Some(entries) = github_contents(client, &url)? {
            let mut files = Vec::new();
            let mut license_file = None;
            collect_files(client, entries, &mut files, &mut license_file)?;
            return Ok(Some(GoogleFontFamily {
                license_dir: license_dir.to_owned(),
                files,
                license_file,
            }));
        }
    }

    Ok(None)
}

fn collect_files(
    client: &Client,
    entries: Vec<GitHubContent>,
    files: &mut Vec<GitHubFile>,
    license_file: &mut Option<GitHubFile>,
) -> Result<()> {
    for entry in entries {
        match entry.kind.as_str() {
            "dir" => {
                if let Some(entries) = github_contents(client, &entry.url)? {
                    collect_files(client, entries, files, license_file)?;
                }
            }
            "file" if entry.name.ends_with(".ttf") => {
                if let Some(download_url) = entry.download_url {
                    files.push(GitHubFile {
                        name: entry.name,
                        download_url,
                    });
                }
            }
            "file" if is_license_file(&entry.name) => {
                if let Some(download_url) = entry.download_url {
                    *license_file = Some(GitHubFile {
                        name: entry.name,
                        download_url,
                    });
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn github_contents(client: &Client, url: &str) -> Result<Option<Vec<GitHubContent>>> {
    let response = client.get(url).send()?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    Ok(Some(response.error_for_status()?.json()?))
}

fn select_fonts<'a>(
    files: &'a [GitHubFile],
    weights: &[u16],
    include_italics: bool,
) -> Vec<&'a GitHubFile> {
    let mut selected = files
        .iter()
        .filter(|file| include_italics || !is_italic(&file.name))
        .filter(|file| {
            weights.is_empty()
                || font_weight(&file.name).is_some_and(|weight| weights.contains(&weight))
        })
        .collect::<Vec<_>>();

    if selected.is_empty() && !weights.is_empty() {
        selected = files
            .iter()
            .filter(|file| include_italics || !is_italic(&file.name))
            .filter(|file| is_weight_variable(&file.name))
            .collect();
    }

    selected.sort_by_key(|file| {
        (
            font_weight(&file.name).unwrap_or(u16::MAX),
            file.name.clone(),
        )
    });
    selected
}

fn download_file(client: &Client, file: &GitHubFile, out: &Path) -> Result<()> {
    download_file_as(client, file, &out.join(&file.name))
}

fn download_file_as(client: &Client, file: &GitHubFile, path: &Path) -> Result<()> {
    let bytes = client
        .get(&file.download_url)
        .send()?
        .error_for_status()?
        .bytes()?;
    fs::write(path, bytes.as_ref())?;
    println!("wrote {}", path.display());
    Ok(())
}

fn google_fonts_slug(family: &str) -> String {
    family
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

fn parse_weights(weights: &str) -> Result<Vec<u16>> {
    weights
        .split(',')
        .map(|weight| {
            weight
                .trim()
                .parse::<u16>()
                .map_err(|error| format!("Invalid font weight {weight:?}: {error}").into())
        })
        .collect()
}

fn is_license_file(name: &str) -> bool {
    matches!(name, "LICENSE.txt" | "OFL.txt" | "UFL.txt")
}

fn is_italic(name: &str) -> bool {
    name.to_ascii_lowercase().contains("italic")
}

fn is_weight_variable(name: &str) -> bool {
    name.to_ascii_lowercase().contains("wght")
}

fn font_weight(name: &str) -> Option<u16> {
    let name = name.to_ascii_lowercase();
    [
        ("extrabold", 800),
        ("semibold", 600),
        ("extralight", 200),
        ("thin", 100),
        ("light", 300),
        ("regular", 400),
        ("medium", 500),
        ("bold", 700),
        ("black", 900),
    ]
    .into_iter()
    .find_map(|(label, weight)| name.contains(label).then_some(weight))
}

#[derive(Debug)]
struct GoogleFontFamily {
    license_dir: String,
    files: Vec<GitHubFile>,
    license_file: Option<GitHubFile>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GitHubFile {
    name: String,
    download_url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubContent {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    url: String,
    download_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn google_fonts_slug_removes_separators() {
        assert_eq!(google_fonts_slug("JetBrains Mono"), "jetbrainsmono");
        assert_eq!(google_fonts_slug("Source Sans 3"), "sourcesans3");
    }

    #[test]
    fn parse_weights_splits_comma_list() {
        assert_eq!(parse_weights("400, 500,600").unwrap(), vec![400, 500, 600]);
    }

    #[test]
    fn font_weight_detects_weight_names_in_order() {
        assert_eq!(font_weight("Inter-SemiBold.ttf"), Some(600));
        assert_eq!(font_weight("Inter-ExtraBold.ttf"), Some(800));
        assert_eq!(font_weight("Inter-Bold.ttf"), Some(700));
    }

    #[test]
    fn select_fonts_filters_weights_and_italics() {
        let files = vec![
            github_file("Family-Regular.ttf"),
            github_file("Family-Medium.ttf"),
            github_file("Family-SemiBold.ttf"),
            github_file("Family-Italic.ttf"),
        ];

        let selected = select_fonts(&files, &[400, 600], false)
            .into_iter()
            .map(|file| file.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(selected, vec!["Family-Regular.ttf", "Family-SemiBold.ttf"]);
    }

    #[test]
    fn select_fonts_falls_back_to_variable_weight_file() {
        let files = vec![github_file("Family[wght].ttf")];

        let selected = select_fonts(&files, &[400, 700], false)
            .into_iter()
            .map(|file| file.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(selected, vec!["Family[wght].ttf"]);
    }

    fn github_file(name: &str) -> GitHubFile {
        GitHubFile {
            name: name.to_owned(),
            download_url: format!("https://example.test/{name}"),
        }
    }
}
