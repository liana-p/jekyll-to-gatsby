# Jekyll to Gatsby

CLI tool for batch updating jekyll markdown content files to markdown files useable by Gatsby (changing the date and slug)

## What it does

Given a directory of Jekyll markdown post files, with a structure like this:

```
- some-folder
  - 2012-02-23-a-post.md
  - 2020-06-12-another-post.md
```

Will batch process and output the posts in an `output` folder (configurable), extracting the date from the file name.

It also adds the `date` field in the frontmatter header of each post so Gatsby can find the date.

New structure (based on the structure used in the [Gatsby Starter Blog](https://www.gatsbyjs.com/starters/gatsbyjs/gatsby-starter-blog):

```
- some-folder
  - a-post
    - index.md
  - another-post
    - index.md
```

## Usage

Written in Rust, can be run with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Install the command

```bash
cargo install jekyll-to-gatsby
```

Run the command

```
cd my-folder-with-posts
cargo run
```

The new files should appear in the `output` folder
