use clap_markdown::MarkdownOptions;
use pretty_assertions::assert_eq;

/// Tests that the `complex-app` example featured in the README.md is
/// up-to-date.
#[test]
fn test_example_complex_app() {
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
