# Jekyll to Gatsby

CLI tool for batch updating jekyll markdown content files to markdown files useable by Gatsby (changing the date and slug)

## Usage

Written in Rust, can be run with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Install the command

```bash
cargo install jekyll-to-gatsby
```

Run the command

```
cd my-folder-with-posts
jekyll-to-gatsby
```

The new files should appear in the `output` folder

## What it does

Given a directory of Jekyll markdown post files, with a structure like this:

```
- some-folder
  - 2012-02-23-a-post.md
  - 2020-06-12-another-post.md
```

- Will batch process and output the posts in an `output` folder (configurable), extracting the date from the file name.
- Adds the `date` field in the frontmatter header of each post so Gatsby can find the date.
- Replaces the `{{ site.url }}{{ site.baseurl }}` syntax in URLs from Jekyll to remove it. You can move your previous Jekyll `assets` folder to the `static` folder in Gatsby and paths should work.

New structure (based on the structure used in the [Gatsby Starter Blog](https://www.gatsbyjs.com/starters/gatsbyjs/gatsby-starter-blog):

```
- some-folder
  - a-post
    - index.md
  - another-post
    - index.md
```

## Options

Use `jekyll-to-gatsby --help`

```
Read some lines of a file Get first n lines of a file

USAGE:
    jekyll-to-gatsby [FLAGS] [OPTIONS] [pattern]

FLAGS:
    -d, --clean-dir
            Clean output directory before starting

    -h, --help
            Prints help information

    -k, --keep-dates
            Keep dates in file names

    -f, --no-folders
            Don't create individual folders for articles.

    -s, --no-slug
            Don't add a slug to the frontmatter header

    -u, --no-url-replace
            Don't replace the jekyll {{site.baseurl}} syntax in URLs

    -V, --version
            Prints version information

    -v, --verbosity
            Pass many times for more log output

            By default, it'll only report errors. Passing `-v` one time also prints warnings, `-vv` enables info
            logging, `-vvv` debug, and `-vvvv` trace.

OPTIONS:
    -o, --output <results_dir>
            Custom output directory [default: output]


ARGS:
    <pattern>
            Custom glob pattern for finding markdown files. [default: **/*.md]
```
