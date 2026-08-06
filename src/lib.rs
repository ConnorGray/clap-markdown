//! This module provides functionality to generate help documentation for
//! `clap`-based command-line applications in Markdown format.
//!
//! It includes a set of features for customization such as adding custom titles,
//! formatting options, table of contents, aliases, and supporting both single-file
//! and multi-file outputs.
//!
//! ## Public API Summary
//!
//! - [`MarkdownOptions`]:
//!     - Struct for configuring various settings when generating Markdown documentation.
//!     - Example: Enabling/disabling the footer, table of contents, and aliases.
//!
//! - [`Markdown`]:
//!     - Struct representing the Markdown output, allowing further modifications and rendering.
//!
//! - Functions:
//!     - [`help_markdown`]: Generate Markdown-formatted help information for a `clap` command with default options.
//!     - [`help_markdown_custom`]: Generate Markdown-formatted help information with custom options.
//!     - [`print_help_markdown`]: Generate and print Markdown-formatted help output to the console.
//!
//! ## Customization
//!
//! The `MarkdownOptions` struct provides the following options:
//!
//! - Set a custom title for the documentation.
//! - Enable or disable the default footer.
//! - Enable or disable the table of contents.
//! - Enable or disable the display of command aliases.
//! - Choose between single-file or multi-file documentation output.
//!
//! ## Example Usage
//!
//! ```rust
//! use clap::Parser;
//!
//! #[derive(Parser)]
//! struct Cli {
//!     #[arg()]
//!     name: String,
//! }
//!
//! let markdown: String = clap_markdown::help_markdown::<Cli>();
//! println!("{}", markdown);
//! ```
//!
//! This describes two conventions for using `clap-markdown`:
//! 1. Add a hidden `--markdown-help` option to your `clap` application:
//! ```rust
//! use clap::Parser;
//!
//! #[derive(Parser)]
//! struct Cli {
//!     #[arg(long, hide = true)]
//!     markdown_help: bool,
//! }
//!
//! fn main() {
//!     let args = Cli::parse();
//!
//!     // Invoked as: `$ my-app --markdown-help`
//!     if args.markdown_help {
//!         clap_markdown::print_help_markdown::<Cli>();
//!     }
//! }
//! ```
//!
//! And then invoke with `--markdown-help` to generate a `CommandLineHelp.md` file:
//!
//! ```shell
//! cargo run -- --markdown-help > docs/CommandLineHelp.md
//! ```
//!
//! 2. You can use the output from any of the [`help_markdown_*_md`] functions.
//!
//! ```rust
//! # use tempfile::tempdir;
//! use std::path::PathBuf;
//! use clap::Parser;
//! use clap_markdown::{Markdown, help_markdown_md};
//!
//! #[derive(Parser)]
//! struct Cli {
//!     #[arg(long, hide = true)]
//!     markdown_help: bool,
//! }
//!
//!
//! fn main() {
//!     let args = Cli::parse();
//!     let md_path = PathBuf::from("CommandLineHelp.md");
//!     # let dir = tempdir().unwrap();
//!     # let md_path = dir.path().join(md_path);
//!     // Invoked as: `$ my-app --markdown-help`
//!     if args.markdown_help {
//!         let md: Markdown = help_markdown_md::<Cli>();
//!         md.write(&md_path).expect("Failed to write Markdown help");
//!     }
//! }
//! ```
//!
//! In either case, save `CommandLineHelp.md` in git, and link to it from the project's README.md or other relevant documentation.
//! Comitting `CommandLineHelp.md` to version control makes it easy to track user-visible changes to the command-line interface.
//! For projects that have multiple associated executables, consider using the
//! command name as a suffix.
//! For example: `CommandLineHelp-your-app.md`, `CommandLineHelp-other-app.md`.
//!
//! ## Notes
//!
//! - This crate assumes `clap` is used as the command-line parsing library.
//! - For multi-file documentation output, any of the [`help_markdown_*_md`] functions
//!   provide a [`Markdown`] struct that can be further handled for multiple files.

// Ensure that doc tests in the README.md file get run.
#[doc(hidden)]
mod test_readme {
    #![doc = include_str!("../README.md")]
}

mod utils;

use clap;
use clap::builder::PossibleValue;
use std::fmt::Write;
use std::{fmt, fs, path};
use utils::pluralize;

//======================================
// Public API types
//======================================

/// Options to customize the structure of the output Markdown document.
///
/// Used with [`help_markdown_custom()`].
#[derive(Clone)]
#[non_exhaustive]
pub struct MarkdownOptions {
    title: Option<String>,
    show_footer: bool,
    show_table_of_contents: bool,
    show_aliases: bool,
    multiple_files: MultipleFiles,
}

#[derive(Clone, PartialEq)]
enum MultipleFiles {
    Single,
    Multiple,
}

impl MarkdownOptions {
    /// Construct a default instance of `MarkdownOptions`.
    pub fn new() -> Self {
        Self {
            title: None,
            show_footer: true,
            show_table_of_contents: true,
            show_aliases: true,
            multiple_files: MultipleFiles::Single,
        }
    }

    /// Set a custom title to use in the generated document.
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);

        self
    }

    /// Whether to show the default footer advertising `clap-markdown`.
    pub fn show_footer(mut self, show: bool) -> Self {
        self.show_footer = show;

        self
    }

    /// Whether to show the default table of contents.
    pub fn show_table_of_contents(mut self, show: bool) -> Self {
        self.show_table_of_contents = show;

        self
    }

    /// Whether to show aliases for arguments and commands.
    pub fn show_aliases(mut self, show: bool) -> Self {
        self.show_aliases = show;

        self
    }

    /// Whether to generate multiple files for the documentation.
    pub fn multiple_files(mut self) -> Self {
        self.multiple_files = MultipleFiles::Multiple;

        self
    }
    pub fn single_file(mut self) -> Self {
        self.multiple_files = MultipleFiles::Single;
        self
    }
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Markdown {
    multi: MultipleFiles,
    options: MarkdownOptions,
    text: Vec<String>,
    commands: Vec<String>,
}

impl From<Markdown> for String {
    fn from(md: Markdown) -> Self {
        let text = md.text.join("");
        if md.options.show_footer {
            format!("{}{}", text, footer())
        } else {
            text.clone()
        }
    }
}

impl From<&Markdown> for String {
    fn from(md: &Markdown) -> Self {
        let text = md.text.join("");
        if md.options.show_footer {
            format!("{}{}", text, footer())
        } else {
            text.clone()
        }
    }
}

impl Markdown {
    /// Create a new `Markdown` instance.
    ///
    /// `text` holds the rendered Markdown sections and `commands` holds the
    /// corresponding command paths used when writing multi-file output.
    pub fn new(
        markdown_options: &MarkdownOptions,
        text: Vec<String>,
        commands: Vec<String>,
    ) -> Markdown {
        Markdown {
            multi: markdown_options.multiple_files.clone(),
            options: markdown_options.clone(),
            text,
            commands,
        }
    }

    /// Iterate over rendered Markdown text and its associated command path.
    ///
    /// Each item is `(text, command_path)`.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.into_iter()
    }

    fn write_files(&self, path: &path::PathBuf) -> std::io::Result<()> {
        for (text, command) in self.iter() {
            let help_path = path.join(command).with_extension("md");
            match help_path.parent() {
                Some(parent) => fs::create_dir_all(parent)?,
                None => (),
            }
            let text = if self.options.show_footer {
                format!("{}{}", text, footer())
            } else {
                text.clone()
            };
            fs::write(help_path, text)?
        }
        Ok(())
    }
    fn write_text(&self, path: &path::PathBuf) -> std::io::Result<()> {
        fs::write(path, String::from(self))
    }

    /// Write the Markdown output to disk.
    ///
    /// Uses the configured output mode:
    /// - [`MarkdownOptions::single_file`] writes one file to `path`
    /// - [`MarkdownOptions::multiple_files`] writes one file per command under `path`
    ///
    /// Returns an I/O error if files or parent directories cannot be created.
    pub fn write(&self, path: &path::PathBuf) -> std::io::Result<()> {
        match self.multi {
            MultipleFiles::Single => self.write_text(path),
            MultipleFiles::Multiple => self.write_files(path),
        }
    }
}

impl<'a> IntoIterator for &'a Markdown {
    type Item = (&'a String, &'a String);
    type IntoIter = std::iter::Zip<
        std::slice::Iter<'a, String>,
        std::slice::Iter<'a, String>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.text.iter().zip(self.commands.iter())
    }
}

impl fmt::Display for Markdown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", String::from(self))
    }
}
//======================================
// Public API functions
//======================================

/// Generate Markdown help for `C` using default [`MarkdownOptions`].
///
/// Convenience wrapper around [`help_markdown_command`].
pub fn help_markdown<C: clap::CommandFactory>() -> String {
    let command = C::command();

    help_markdown_command(&command)
}

/// Generate Markdown help for `C` using custom [`MarkdownOptions`].
///
/// Convenience wrapper around [`help_markdown_command_custom`].
pub fn help_markdown_custom<C: clap::CommandFactory>(
    options: &MarkdownOptions,
) -> String {
    let command = C::command();

    help_markdown_command_custom(&command, options)
}

/// Generate Markdown help for an existing [`clap::Command`] using default options.
///
/// Convenience wrapper around [`help_markdown_command_custom`].
pub fn help_markdown_command(command: &clap::Command) -> String {
    help_markdown_command_custom(command, &Default::default())
}

/// Generate Markdown help for an existing [`clap::Command`] using custom options.
///
/// Returns a single `String` document formatted as Markdown.
///
/// # Panics
///
/// Panics if `options.multiple_files()` is enabled.
/// Use [`help_markdown_command_custom_md`] when generating multi-file output.
pub fn help_markdown_command_custom(
    command: &clap::Command,
    options: &MarkdownOptions,
) -> String {
    if options.multiple_files == MultipleFiles::Multiple {
        panic!("`multiple_files` is not supported for `help_markdown_command_custom`. Use `help_markdown_command_custom_md` instead, which returns a `Markdown` struct that can be written to multiple files.");
    }
    let markdown = help_markdown_command_custom_md(command, options);
    markdown.into()
}

/// Generate Markdown help for `C` using default options, returning structured output.
///
/// Returns [`Markdown`] so callers can render or write multiple files.
pub fn help_markdown_md<C: clap::CommandFactory>() -> Markdown {
    let command = C::command();

    help_markdown_command_md(&command)
}

/// Generate Markdown help for `C` using custom options, returning structured output.
///
/// Returns [`Markdown`] so callers can render or write multiple files.
pub fn help_markdown_custom_md<C: clap::CommandFactory>(
    options: &MarkdownOptions,
) -> Markdown {
    let command = C::command();

    help_markdown_command_custom_md(&command, options)
}

/// Generate Markdown help for an existing [`clap::Command`] using default options.
///
/// Returns [`Markdown`] so callers can render or write multiple files.
pub fn help_markdown_command_md(command: &clap::Command) -> Markdown {
    help_markdown_command_custom_md(command, &Default::default())
}

/// Canonical Markdown generator for an existing [`clap::Command`].
///
/// All public `help_markdown*` helpers delegate to this function.
///
/// Returns [`Markdown`] so callers can render or write multiple files.
pub fn help_markdown_command_custom_md(
    command: &clap::Command,
    options: &MarkdownOptions,
) -> Markdown {
    build_command_markdown_process(command, options)
}

//======================================
// Markdown
//======================================

/// Format the help information for `command` as Markdown and print it.
///
/// Output is printed to the standard output, using [`println!`].
pub fn print_help_markdown<C: clap::CommandFactory>() {
    let command = C::command();
    let markdown = help_markdown_command_md(&command);

    println!("{}", markdown);
}

fn write_preamble_markdown(
    buffer: &mut String,
    command: &clap::Command,
    options: &MarkdownOptions,
) -> fmt::Result {
    write_title_markdown(buffer, command, options)?;

    if options.show_table_of_contents {
        writeln!(buffer, "**Command Overview:**\n")?;

        build_table_of_contents_markdown(
            buffer,
            Vec::new(),
            command,
            0,
            options,
        )?;

        write!(buffer, "\n")?;
    }
    Ok(())
}

fn write_title_markdown(
    buffer: &mut String,
    command: &clap::Command,
    options: &MarkdownOptions,
) -> fmt::Result {
    let title_name = get_canonical_name(command);

    let title = match options.title {
        Some(ref title) => title.to_owned(),
        None => format!("Command-Line Help for `{title_name}`"),
    };
    writeln!(buffer, "# {title}\n",)?;

    writeln!(
        buffer,
        "This document contains the help content for the `{}` command-line program.\n",
        title_name
    )?;
    Ok(())
}

fn footer() -> &'static str {
    r#"<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
"#
}

fn build_table_of_contents_markdown(
    buffer: &mut String,
    // Parent commands of `command`.
    parent_command_path: Vec<String>,
    command: &clap::Command,
    depth: usize,
    markdown_options: &MarkdownOptions,
) -> fmt::Result {
    // Don't document commands marked with `clap(hide = true)` (which includes
    // `print-all-help`).
    if command.is_hide_set() {
        return Ok(());
    }

    let title_name = get_canonical_name(command);

    // Append the name of `command` to `command_path`.
    let command_path = {
        let mut command_path = parent_command_path;
        command_path.push(title_name);
        command_path
    };
    let contents = match markdown_options.multiple_files {
        MultipleFiles::Multiple => format!(
            "* [`{}`]({}.md)",
            command_path.join(" "),
            command_path.join("/")
        ),
        MultipleFiles::Single => format!(
            "* [`{}`↴](#{})",
            command_path.join(" "),
            command_path.join("-")
        ),
    };
    writeln!(buffer, "{}", contents)?;

    //----------------------------------
    // Recurse to write subcommands
    //----------------------------------

    for subcommand in command.get_subcommands() {
        build_table_of_contents_markdown(
            buffer,
            command_path.clone(),
            subcommand,
            depth + 1,
            markdown_options,
        )?;
    }

    Ok(())
}

fn build_command_markdown_parts(
    // Parent commands of `command`.
    parent_command_path: &Vec<String>,
    command: &clap::Command,
    options: &MarkdownOptions,
) -> Result<String, fmt::Error> {
    let mut buffer = String::with_capacity(100);
    if let Some(long_about) = command.get_long_about() {
        writeln!(buffer, "{}\n", long_about)?;
    } else if let Some(about) = command.get_about() {
        writeln!(buffer, "{}\n", about)?;
    }

    if let Some(help) = command.get_before_long_help() {
        writeln!(buffer, "{}\n", help)?;
    } else if let Some(help) = command.get_before_help() {
        writeln!(buffer, "{}\n", help)?;
    }

    write_usage(parent_command_path, command, &mut buffer)?;

    if options.show_aliases {
        let aliases = command.get_visible_aliases().collect::<Vec<&str>>();
        if let Some(aliases_str) = get_alias_string(&aliases) {
            writeln!(
                buffer,
                "**{}:** {aliases_str}\n",
                pluralize(aliases.len(), "Command Alias", "Command Aliases")
            )?;
        }
    }

    if let Some(help) = command.get_after_long_help() {
        writeln!(buffer, "{}\n", help)?;
    } else if let Some(help) = command.get_after_help() {
        writeln!(buffer, "{}\n", help)?;
    }

    if command.get_subcommands().next().is_some() {
        let subcom = build_command_markdown_subcommands(
            command,
            parent_command_path,
            options,
        )?;
        write!(buffer, "{}\n", subcom)?;
    }

    if command.get_positionals().next().is_some() {
        writeln!(buffer, "###### **Arguments:**\n")?;

        for pos_arg in command.get_positionals() {
            write_arg_markdown(&mut buffer, pos_arg)?;
        }

        write!(buffer, "\n")?;
    }

    let non_pos: Vec<_> = command
        .get_arguments()
        .filter(|arg| !arg.is_positional() && !arg.is_hide_set())
        .collect();

    if !non_pos.is_empty() {
        writeln!(buffer, "###### **Options:**\n")?;

        for arg in non_pos {
            write_arg_markdown(&mut buffer, arg)?;
        }

        write!(buffer, "\n")?;
    }
    Ok(buffer)
}

fn write_usage(
    parent_command_path: &Vec<String>,
    command: &clap::Command,
    buffer: &mut String,
) -> Result<(), fmt::Error> {
    let command_path = if parent_command_path.is_empty() {
        String::new()
    } else {
        let mut s = parent_command_path.join(" ");
        s.push(' ');
        s
    };
    let command = command
        .clone()
        .render_usage()
        .to_string()
        .replace("Usage: ", "");

    writeln!(buffer, "**Usage:** `{}{}`\n", command_path, command)?;
    Ok(())
}

fn build_command_markdown_subcommands(
    command: &clap::Command,
    parent_command_path: &Vec<String>,
    options: &MarkdownOptions,
) -> Result<String, fmt::Error> {
    let mut buffer = String::with_capacity(100);
    writeln!(buffer, "###### **Subcommands:**\n")?;
    for subcommand in command.get_subcommands() {
        if subcommand.is_hide_set() {
            continue;
        }

        let title_name = get_canonical_name(subcommand);

        let about = match subcommand.get_about() {
            Some(about) => about.to_string(),
            None => String::new(),
        };
        let link_path = match options.multiple_files {
            MultipleFiles::Single => format!("`{title_name}`"),
            MultipleFiles::Multiple => {
                let command_path_str: String = parent_command_path
                    .last()
                    .unwrap_or(&String::new())
                    .clone();
                format!("[`{title_name}`]({command_path_str}/{title_name}.md)")
            },
        };

        writeln!(buffer, "* {link_path} — {about}",)?;
    }
    Ok(buffer.to_string())
}

fn build_command_markdown_process(
    command: &clap::Command,
    options: &MarkdownOptions,
) -> Markdown {
    let mut vec_buffer = Vec::new();
    let mut cmd_buffer = Vec::new();

    build_command_markdown(
        &mut vec_buffer,
        &mut cmd_buffer,
        Vec::new(),
        command,
        options,
    )
    .unwrap();
    Markdown::new(options, vec_buffer, cmd_buffer)
}
fn build_command_markdown(
    vec_buffer: &mut Vec<String>,
    cmd_buffer: &mut Vec<String>,
    // Parent commands of `command`.
    parent_command_path: Vec<String>,
    command: &clap::Command,
    options: &MarkdownOptions,
) -> fmt::Result {
    if command.is_hide_set() {
        return Ok(());
    }
    let mut buffer = String::with_capacity(100);
    let title_name = get_canonical_name(command);
    let command_path = {
        let mut command_path = parent_command_path.clone();
        command_path.push(title_name);
        command_path
    };
    if parent_command_path.is_empty() {
        write_preamble_markdown(&mut buffer, command, options)?;
        cmd_buffer.push(String::from("index"));
    } else {
        cmd_buffer.push(command_path.join("/"));
    }
    writeln!(buffer, "## `{}`\n", command_path.join(" "))?;
    let cmd_md =
        build_command_markdown_parts(&parent_command_path, command, options)?;
    write!(buffer, "{}\n\n", cmd_md)?;
    vec_buffer.push(buffer);

    for subcommand in command.get_subcommands() {
        build_command_markdown(
            vec_buffer,
            cmd_buffer,
            command_path.clone(),
            subcommand,
            options,
        )?;
    }
    Ok(())
}

fn write_arg_markdown(buffer: &mut String, arg: &clap::Arg) -> fmt::Result {
    // Markdown list item
    write!(buffer, "* ")?;

    let value_name: String = match arg.get_value_names() {
        // TODO: What if multiple names are provided?
        Some([name, ..]) => name.as_str().to_owned(),
        Some([]) => unreachable!(
            "clap Arg::get_value_names() returned Some(..) of empty list"
        ),
        None => arg.get_id().to_string().to_ascii_uppercase(),
    };

    match (arg.get_short(), arg.get_long()) {
        (Some(short), Some(long)) => {
            if arg.get_action().takes_values() {
                write!(buffer, "`-{short}`, `--{long} <{value_name}>`")?
            } else {
                write!(buffer, "`-{short}`, `--{long}`")?
            }
        },
        (Some(short), None) => {
            if arg.get_action().takes_values() {
                write!(buffer, "`-{short} <{value_name}>`")?
            } else {
                write!(buffer, "`-{short}`")?
            }
        },
        (None, Some(long)) => {
            if arg.get_action().takes_values() {
                write!(buffer, "`--{} <{value_name}>`", long)?
            } else {
                write!(buffer, "`--{}`", long)?
            }
        },
        (None, None) => {
            debug_assert!(arg.is_positional(), "unexpected non-positional Arg with neither short nor long name: {arg:?}");

            write!(buffer, "`<{value_name}>`",)?;
        },
    }

    if let Some(aliases) = arg.get_visible_aliases().as_deref() {
        if let Some(aliases_str) = get_alias_string(aliases) {
            write!(
                buffer,
                " [{}: {aliases_str}]",
                pluralize(aliases.len(), "alias", "aliases")
            )?;
        }
    }

    if let Some(help) = arg.get_long_help() {
        // TODO: Parse formatting in the string
        buffer.push_str(&indent(&help.to_string(), " — ", "   "))
    } else if let Some(short_help) = arg.get_help() {
        writeln!(buffer, " — {short_help}")?;
    } else {
        writeln!(buffer)?;
    }

    //--------------------
    // Arg default values
    //--------------------

    if !arg.get_default_values().is_empty() {
        let default_values: String = arg
            .get_default_values()
            .iter()
            .map(|value| format!("`{}`", value.to_string_lossy()))
            .collect::<Vec<String>>()
            .join(", ");

        if arg.get_default_values().len() > 1 {
            // Plural
            writeln!(buffer, "\n  Default values: {default_values}")?;
        } else {
            // Singular
            writeln!(buffer, "\n  Default value: {default_values}")?;
        }
    }

    //--------------------
    // Arg possible values
    //--------------------

    let possible_values: Vec<PossibleValue> = arg
        .get_possible_values()
        .into_iter()
        .filter(|pv| !pv.is_hide_set())
        .collect();

    // Print possible values for options that take a value, but not for flags
    // that can only be either present or absent and do not take a value.
    if !possible_values.is_empty()
        && !matches!(arg.get_action(), clap::ArgAction::SetTrue)
    {
        let any_have_help: bool =
            possible_values.iter().any(|pv| pv.get_help().is_some());

        if any_have_help {
            // If any of the possible values have help text, print them
            // as a separate item in a bulleted list, and include the
            // help text for those that have it. E.g.:
            //
            //     Possible values:
            //     - `value1`:
            //       The help text
            //     - `value2`
            //     - `value3`:
            //       The help text

            let text: String = possible_values
                .iter()
                .map(|pv| match pv.get_help() {
                    Some(help) => {
                        format!("  - `{}`:\n    {}\n", pv.get_name(), help)
                    },
                    None => format!("  - `{}`\n", pv.get_name()),
                })
                .collect::<Vec<String>>()
                .join("");

            writeln!(buffer, "\n  Possible values:\n{text}")?;
        } else {
            // If none of the possible values have any documentation, print
            // them all inline on a single line.
            let text: String = possible_values
                .iter()
                // TODO: Show PossibleValue::get_help(), and PossibleValue::get_name_and_aliases().
                .map(|pv| format!("`{}`", pv.get_name()))
                .collect::<Vec<String>>()
                .join(", ");

            writeln!(buffer, "\n  Possible values: {text}\n")?;
        }
    }

    Ok(())
}

/// Utility function to get the canonical name of a command.
///
/// It's logic is to get the display name if it exists, otherwise get the bin
/// name if it exists, otherwise get the package name.
///
/// Note that the default `Command.name` field of a clap command is typically
/// meant for programmatic usage as well as for display (if no `display_name`
/// override is set).
fn get_canonical_name(command: &clap::Command) -> String {
    command
        .get_display_name()
        .or_else(|| command.get_bin_name())
        .map(|name| name.to_owned())
        .unwrap_or_else(|| command.get_name().to_owned())
}

/// Indents non-empty lines. The output always ends with a newline.
fn indent(s: &str, first: &str, rest: &str) -> String {
    if s.is_empty() {
        // For consistency. It's easiest to always add a newline at the end, and
        // there's little reason not to.
        return "\n".to_string();
    }
    let mut result = String::new();
    let mut first_line = true;

    for line in s.lines() {
        if !line.is_empty() {
            result.push_str(if first_line { first } else { rest });
            result.push_str(line);
            first_line = false;
        }
        result.push('\n');
    }
    result
}

fn get_alias_string(aliases: &[&str]) -> Option<String> {
    if aliases.is_empty() {
        return None;
    }

    Some(format!(
        "{}",
        aliases
            .iter()
            .map(|alias| format!("`{alias}`"))
            .collect::<Vec<_>>()
            .join(", ")
    ))
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_indent() {
        use super::indent;
        assert_eq!(
            &indent("Header\n\nMore info", "___", "~~~~"),
            "___Header\n\n~~~~More info\n"
        );
        assert_eq!(
            &indent("Header\n\nMore info\n", "___", "~~~~"),
            &indent("Header\n\nMore info", "___", "~~~~"),
        );
        assert_eq!(&indent("", "___", "~~~~"), "\n");
        assert_eq!(&indent("\n", "___", "~~~~"), "\n");
    }
}
