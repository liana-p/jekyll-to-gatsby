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
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;

    let files = glob(&args.pattern)?;
    let results_dir = std::path::Path::new(&args.results_dir);
    create_results_dir(results_dir, &args.clean_dir)?;
    info!("Saving {} generated files into {:?}...", files.len(), args.results_dir);
    let results = files
        .par_iter()
        .map(|path| {
            transform_markdown(path, &args.results_dir)
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
    results_dir: &str,
) -> Result<(), Error> {
    println!("Processing {}", original.display());
    let file_name = original
        .file_name()
        .ok_or_else(|| format_err!("couldn't read file name of {:?}", original))?;
    let file_name_str = file_name.to_str().unwrap();
    let date_matches = DATE_REGEX.captures(file_name_str);
    if let Some(date_match) = &date_matches {
        let year = date_match.name("y").unwrap().as_str();
        let month = date_match.name("m").unwrap().as_str();
        let day = date_match.name("d").unwrap().as_str();
        // Using placeholder time since old posts don't have a time
        let new_date = format!("{}-{}-{}T22:40:32.169Z", year, month, day);
        let new_name = DATE_REGEX.replace(file_name_str, "").to_string();
        println!("New file name {}", new_name);
        let new_dir_path = PathBuf::from(results_dir)
            .join(&new_name);
        create_results_dir(&new_dir_path, &false)?;
        let output_path = PathBuf::from(results_dir)
            .join(new_name)
            .join("index")
            .with_extension("md");
        let content = read_file(file_name)?;
        let result = FRONTMATTER.replace(content.as_str(), format!("---
date: \"{}\"
", new_date));
        write_to_file(output_path, &result)?;
        info!("Processed {} successfully!", original.display());
        Ok(())
    } else {
        Err(format_err!("Couldn't find a date in file name {}", original.display()))
    }
}

/// Read some lines of a file
// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
    /// How many lines to get
    #[structopt(long = "no-folders", short = "nf", help = "Don't create individual folders for articles.")]
    no_folders: bool,
    // Add a positional argument that the user has to supply:
    /// The file to read
    #[structopt(default_value = "**/*.md")]
    pattern: String,
    /// Where do you want to save the thumbnails?
    #[structopt(long = "output", short = "o", default_value = "output")]
    results_dir: String,

    /// Should we clean the output directory?
    #[structopt(long="clean-dir")]
    clean_dir: bool,

    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}