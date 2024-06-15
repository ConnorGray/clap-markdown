use clap::{Arg, Command};
use clap_markdown::{help_markdown_command_custom, MarkdownOptions};

use pretty_assertions::assert_eq;

/// Test behavior for an app that defines a:
///
/// * `name`
/// * `display_name`
///
/// but no custom `bin_name`.
#[test]
fn test_title_behavior_for_name_and_display_name_app() {
    let mut app = Command::new("my-program-name")
        // Note: Intentionally not setting custom bin name.
        // .bin_name("my-program-bin-name")
        .display_name("my-program-display-name")
        .version("1.2.3")
        .about("This program does things.")
        .arg(Arg::new("foo").short('f'));
    let () = app.build();

    //-------------------------------------------------------------------
    // Test the native behavior of `clap`, in terms of whether it prefers
    // the `name`, `bin_name`, and `display_name` properties are used.
    //-------------------------------------------------------------------

    assert_eq!(
        app.render_long_help().to_string(),
        "\
This program does things.

Usage: my-program-name [OPTIONS]

Options:
  -f <foo>
          

  -h, --help
          Print help

  -V, --version
          Print version
"
    );

    //-------------------------------------------------------
    // Test how clap-markdown handles the various name fields
    //-------------------------------------------------------

    assert_eq!(
        help_markdown_command_custom(
            &app,
            &MarkdownOptions::new().show_footer(false)
        ),
        "\
# Command-Line Help for my-program-display-name

This document contains the help content for the `my-program-display-name` command-line program.

**Command Overview:**

* [`my-program-display-name`↴](#my-program-display-name)

## `my-program-display-name`

This program does things.

**Usage:** `my-program-name [OPTIONS]`

###### **Options:**

* `-f <FOO>`
* `-h`, `--help` — Print help
* `-V`, `--version` — Print version



"
    );
}

/// Test behavior for an app that defines a:
///
/// * `name`
/// * `bin_name`
/// * `display_name`
#[test]
fn test_title_behavior_for_name_bin_name_and_display_name_app() {
    let mut app = Command::new("my-program-name")
        .bin_name("my-program-bin-name")
        .display_name("my-program-display-name")
        .version("1.2.3")
        .about("This program does things.")
        .arg(Arg::new("foo").short('f'));
    let () = app.build();

    //-------------------------------------------------------------------
    // Test the native behavior of `clap`, in terms of whether it prefers
    // the `name`, `bin_name`, and `display_name` properties are used.
    //-------------------------------------------------------------------

    assert_eq!(
        app.render_long_help().to_string(),
        "\
This program does things.

Usage: my-program-bin-name [OPTIONS]

Options:
  -f <foo>
          

  -h, --help
          Print help

  -V, --version
          Print version
"
    );

    //-------------------------------------------------------
    // Test how clap-markdown handles the various name fields
    //-------------------------------------------------------

    assert_eq!(
        help_markdown_command_custom(
            &app,
            &MarkdownOptions::new().show_footer(false)
        ),
        "\
# Command-Line Help for my-program-display-name

This document contains the help content for the `my-program-display-name` command-line program.

**Command Overview:**

* [`my-program-display-name`↴](#my-program-display-name)

## `my-program-display-name`

This program does things.

**Usage:** `my-program-bin-name [OPTIONS]`

###### **Options:**

* `-f <FOO>`
* `-h`, `--help` — Print help
* `-V`, `--version` — Print version



"
    );
}
