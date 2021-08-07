use quicli::prelude::*;
use structopt::StructOpt;
use std::path::Path;
use std::env;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref FRONTMATTER: regex::Regex = Regex::new(r"---[\n\r]").unwrap();
    static ref DATE_REGEX: regex::Regex = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})-").unwrap();
    static ref URL_REGEX: regex::Regex = Regex::new(r"\{\{\s*site.url\s*\}\}\s*\{\{\s*site.baseurl\s*\}\}").unwrap();
    static ref REMOVE_EXTENSION: regex::Regex = Regex::new(r"\..*").unwrap();
}


fn main() -> CliResult {
    let args: Cli = Cli::from_args();
    args.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;
    let ctx = Context::from_cli(args);

    let files = glob(&ctx.pattern)?;
    let results_dir = std::path::Path::new(&ctx.results_dir);
    create_results_dir(results_dir, &ctx.clean_dir)?;
    info!("Saving {} generated files into {:?}...", files.len(), ctx.results_dir);
    let results = files
        .par_iter()
        .map(|path| {
            transform_markdown(path, &ctx)
            .map_err(|e| error!("Failed to process {} ({})", path.display(), e))
        });
    let results_count: i32 = results
        .map(|x| if x.is_ok() { 1 } else { 0 })
        .sum();
    println!(
        "{} of {} files successfully converted!",
        results_count,
        files.len()
    );
    Ok(())
}

fn create_results_dir(results_dir: &Path, clean_dir: &bool) -> Result<(), Error> {
    if *clean_dir && results_dir.exists() {
        remove_dir_all(&results_dir)?;
    }
    create_dir(&results_dir)?;
    Ok(())
}

fn transform_markdown(
    original: &Path,
    ctx: &Context,
) -> Result<(), Error> {
    println!("Processing {}", original.display());
    let file_name_os = original
        .file_name()
        .ok_or_else(|| format_err!("couldn't read file name of {:?}", original))?;
    let file_name = file_name_os.to_str().unwrap().to_string();
    let file_destination = setup_file_destination(file_name, ctx)?;
    let content = read_file(file_name_os)?;
    let mut new_frontmatter = format!("---
date: \"{}\"
", file_destination.new_date);
    if !ctx.no_slug {
        new_frontmatter = format!("{}slug: \"{}\"
", new_frontmatter, file_destination.slug);
    }
    let mut result = FRONTMATTER.replace(content.as_str(), new_frontmatter).to_string();
    if !ctx.no_url_replace {
        result = URL_REGEX.replace_all(&result, "").to_string();
    }
    write_to_file(file_destination.output_path, &result)?;
    info!("Processed {} successfully!", original.display());
    Ok(())
}

struct FileDestinationResult {
    output_path: PathBuf,
    new_name: String,
    new_date: String,
    slug: String,
}
fn setup_file_destination(
    file_name: String,
    ctx: &Context,
) -> Result<FileDestinationResult, Error> {
    let date_matches = DATE_REGEX.captures(&file_name);
    if let Some(date_match) = &date_matches {
        let year = date_match.name("y").unwrap().as_str();
        let month = date_match.name("m").unwrap().as_str();
        let day = date_match.name("d").unwrap().as_str();
        // Using placeholder time since old posts don't have a time
        let new_date = format!("{}-{}-{}T22:40:32.169Z", year, month, day);
        let name_without_extension = REMOVE_EXTENSION.replace(&file_name, "").to_string();
        let name_without_date = DATE_REGEX.replace(&name_without_extension, "").to_string();
        let new_name: String;
        if ctx.keep_dates {
            new_name = name_without_extension;
        } else {
            new_name = String::from(&name_without_date);
        }
        println!("New file name {}", new_name);
        if !ctx.no_folders {
            let new_dir_path = PathBuf::from(&ctx.results_dir)
                .join(&new_name);
            create_results_dir(&new_dir_path, &false)?;
        }
        let mut output_path = PathBuf::from(&ctx.results_dir)
            .join(&new_name);
        if !ctx.no_folders {
            output_path = output_path.join("index");
        }
        output_path = output_path.with_extension("md");
        Ok(FileDestinationResult {
            output_path,
            new_name,
            new_date,
            slug: name_without_date,
        })
    } else {
        Err(format_err!("Couldn't find a date in file name {}", file_name))
    }
}

/// Read some lines of a file
// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "no-folders", short = "f", help = "Don't create individual folders for articles.")]
    no_folders: bool,

    #[structopt(default_value = "**/*.md", help = "Custom glob pattern for finding markdown files.")]
    pattern: String,

    #[structopt(long = "output", short = "o", default_value = "output", help = "Custom output directory")]
    results_dir: String,

    #[structopt(long="clean-dir", short = "d", help = "Clean output directory before starting")]
    clean_dir: bool,

    #[structopt(long="keep-dates", short = "k", help = "Keep dates in file names")]
    keep_dates: bool,

    #[structopt(long="no-url-replace", short = "u", help = "Don't replace the jekyll {{site.baseurl}} syntax in URLs")]
    no_url_replace: bool,

    #[structopt(long="no-slug", short = "s", help = "Don't add a slug to the frontmatter header")]
    no_slug: bool,

    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}


// Annoying code duplication because we can't instantiate Cli for unit tests due to Verbosity being private...
struct Context {
    no_folders: bool,
    pattern: String,
    results_dir: String,
    clean_dir: bool,
    keep_dates: bool,
    no_url_replace: bool,
    no_slug: bool,
}

impl Context {
    pub fn from_cli(cli: Cli) -> Context {
        Context {
            no_folders: cli.no_folders,
            pattern: cli.pattern,
            results_dir: cli.results_dir,
            clean_dir: cli.clean_dir,
            keep_dates: cli.keep_dates,
            no_url_replace: cli.no_url_replace,
            no_slug: cli.no_slug,
        }
    }
}
#[cfg(test)]
mod test;
