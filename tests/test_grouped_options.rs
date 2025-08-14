use clap::{Arg, Command};
use clap_markdown::{help_markdown_command_custom, MarkdownOptions};
use pretty_assertions::assert_eq;

#[test]
fn test_grouped_options_by_help_heading() {
    let app = Command::new("grouped-app")
        .about("Test app with grouped options")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .help_heading("General Options")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug output")
                .help_heading("General Options")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input file")
                .help_heading("File Options")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file")
                .help_heading("File Options")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("Output format")
                .value_name("FORMAT"), // No help_heading - should go to default "Options"
        );

    let expected_output = "\
# Command-Line Help for `grouped-app`

This document contains the help content for the `grouped-app` command-line program.

**Command Overview:**

* [`grouped-app`↴](#grouped-app)

## `grouped-app`

Test app with grouped options

**Usage:** `grouped-app [OPTIONS]`

###### **File Options:**

* `-i`, `--input <FILE>` — Input file
* `-o`, `--output <FILE>` — Output file

###### **General Options:**

* `-v`, `--verbose` — Enable verbose output
* `-d`, `--debug` — Enable debug output

###### **Options:**

* `-f`, `--format <FORMAT>` — Output format



";

    assert_eq!(
        help_markdown_command_custom(
            &app,
            &MarkdownOptions::new().show_footer(false)
        ),
        expected_output
    );
}

#[test]
fn test_no_grouped_options_backward_compatibility() {
    let app = Command::new("simple-app")
        .about("Test app without grouped options")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file")
                .value_name("FILE"),
        );

    let expected_output = "\
# Command-Line Help for `simple-app`

This document contains the help content for the `simple-app` command-line program.

**Command Overview:**

* [`simple-app`↴](#simple-app)

## `simple-app`

Test app without grouped options

**Usage:** `simple-app [OPTIONS]`

###### **Options:**

* `-v`, `--verbose` — Enable verbose output
* `-o`, `--output <FILE>` — Output file



";

    assert_eq!(
        help_markdown_command_custom(
            &app,
            &MarkdownOptions::new().show_footer(false)
        ),
        expected_output
    );
}

#[test]
fn test_empty_help_heading() {
    // Test edge case where help_heading is an empty string
    let app = Command::new("empty-heading-app")
        .about("Test app with empty help heading")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .help_heading("") // Empty string
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug output")
                .help_heading("Debug Options")
                .action(clap::ArgAction::SetTrue),
        );

    let output = help_markdown_command_custom(
        &app,
        &MarkdownOptions::new().show_footer(false),
    );

    // Should have both empty heading section and Debug Options section
    assert!(output.contains("###### **:**")); // Empty heading
    assert!(output.contains("###### **Debug Options:**"));
    assert!(output.contains("Enable verbose output"));
    assert!(output.contains("Enable debug output"));
}
