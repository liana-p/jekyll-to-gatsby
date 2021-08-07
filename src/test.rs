use super::*;

#[test]
fn test_file_destination_no_args() -> Result<(), Error> {
    let context = Context{
      no_folders: false,
      pattern: String::from("**/*.md"),
      results_dir: String::from("output"),
      clean_dir: false,
      keep_dates: false,
      no_url_replace: false,
      no_slug: false,
    };

    let result = setup_file_destination(
      String::from("2012-03-22-some-post.md"),
            &context)?;
    assert_eq!(result.new_date, "2012-03-22T22:40:32.169Z");
    assert_eq!(result.slug, "some-post");
    assert_eq!(result.new_name, "some-post");
    assert_eq!(result.output_path, PathBuf::from("output/some-post/index.md"));
    Ok(())
}

#[test]
fn test_file_destination_keep_date() -> Result<(), Error> {
    let context = Context{
      no_folders: false,
      pattern: String::from("**/*.md"),
      results_dir: String::from("output"),
      clean_dir: false,
      keep_dates: true,
      no_url_replace: false,
      no_slug: false,
    };

    let result = setup_file_destination(
      String::from("2008-12-03-a-post.md"),
            &context)?;
    assert_eq!(result.new_date, "2008-12-03T22:40:32.169Z");
    assert_eq!(result.slug, "a-post");
    assert_eq!(result.new_name, "2008-12-03-a-post");
    assert_eq!(result.output_path, PathBuf::from("output/2008-12-03-a-post/index.md"));
    Ok(())
}

#[test]
fn test_file_destination_no_folders_keep_date() -> Result<(), Error> {
    let context = Context{
      no_folders: true,
      pattern: String::from("**/*.md"),
      results_dir: String::from("output"),
      clean_dir: false,
      keep_dates: true,
      no_url_replace: false,
      no_slug: false,
    };

    let result = setup_file_destination(
      String::from("2008-12-03-a-post.md"),
            &context)?;
    assert_eq!(result.new_date, "2008-12-03T22:40:32.169Z");
    assert_eq!(result.slug, "a-post");
    assert_eq!(result.new_name, "2008-12-03-a-post");
    assert_eq!(result.output_path, PathBuf::from("output/2008-12-03-a-post.md"));
    Ok(())
}
