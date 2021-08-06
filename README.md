# update-jekyll-content-to-gatsby

CLI for updating jekyll markdown content files to markdown files useable by Gatsby (changing the date and slug)

## Usage

Written in Rust, can be run with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Just download/clone this repo, copy the jekyll markdown posts you want to convert to the root, and run:

```bash
cargo run
```

The new files should appear in the `output` folder
