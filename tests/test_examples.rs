use clap_markdown::MarkdownOptions;
use pretty_assertions::assert_eq;

/// Tests that the `complex-app` example featured in the README.md is
/// up-to-date.
#[test]
fn test_example_complex_app_single_file() {
    mod complex_app {
        include!("../docs/examples/complex_app.rs");
    }

    assert_eq!(
        clap_markdown::help_markdown::<complex_app::Cli>(),
        include_str!("../docs/examples/complex-app.md")
    );

    assert_eq!(
        clap_markdown::help_markdown_custom::<complex_app::Cli>(
            &MarkdownOptions::new()
                .title("Some Custom Title for Complex App".to_string())
                .show_footer(false)
                .show_table_of_contents(false)
                .show_aliases(false)
        ),
        include_str!("../docs/examples/complex-app-custom.md"),
        "Mismatch testing CUSTOM Markdown output"
    );
}

#[test]
fn test_example_complex_app_muliple_files() {
    mod complex_app {
        include!("../docs/examples/complex_app.rs");
    }

    let out = tempfile::tempdir().unwrap();
    let md = clap_markdown::help_markdown_custom_md::<complex_app::Cli>(
        &MarkdownOptions::new().multiple_files(),
    );

    md.write(&out.path().to_path_buf()).unwrap();

    assert!(out.path().join("index.md").exists());
    assert!(out.path().join("complex-app/test.md").exists());
    assert!(out
        .path()
        .join("complex-app/only-hidden-options.md")
        .exists());
}
