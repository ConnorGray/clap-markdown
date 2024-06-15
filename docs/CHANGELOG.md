# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

### Added

* Added `help_markdown_custom()` and `help_markdown_command_custom()`, for
  customizing the built Markdown.

  Supported customization options include:

  - Contents of the command title header.
  - Whether to show the footer explaining the documentation was generated with
    `clap-markdown`.
  - Whether to show the command overview table of contents.

### Changed

* `clap-markdown` will now respect the `Command.display_name` property, if set,
  and use it in the rendered Markdown when showing the name and usage of the
  command.
  ([#27],
  co-authored-by [@keturiosakys](https://github.com/keturiosakys)
  and [@hatchan](https://github.com/hatchan))



## [0.1.3] - 2022-12-28

### Added

* For arguments that take a value, the argument value name
  ([`Arg::get_value_names()`](https://docs.rs/clap/4.0.32/clap/struct.Arg.html#method.get_value_names))
  is now included in the generated Markdown. ([#11])

* For arguments that have default values
  ([`Arg::get_default_values()`](https://docs.rs/clap/4.0.32/clap/struct.Arg.html#method.get_default_values))
  , those default values are mentioned in the generated Markdown. ([#11])

### Changed

* The rendered Markdown for
  [`PossibleValue`](https://docs.rs/clap/4.0.32/clap/builder/struct.PossibleValue.html)
  literal values now uses `code` styling instead of **emphasized** styling, for
  consistency with other literally-what-the-user-types help content (e.g.
  option or subcommand names). ([#11])



## [0.1.2] - 2022-12-28

### Added

* Generated markdown for
  [`PossibleValue`](https://docs.rs/clap/4.0.32/clap/builder/struct.PossibleValue.html)
  values will now include the
  [`PossibleValue::get_help()`](https://docs.rs/clap/4.0.32/clap/builder/struct.PossibleValue.html#method.get_help)
  content, if it exists. ([#8])

  This means that documentation comments present on enum variants that use
  `#[derive(clap::ValueEnum)]` will be summarized in the generated Markdown
  documentation.



## [0.1.1] - 2022-12-26

### Added

* Added new basic example to README.md. ([#4])

* Added new 'Usage Convention: CommandLineHelp.md' section to README.md ([#4])

* Add list of projects using `clap-markdown` and `CommandLineHelp.md` convention
  to README.md. ([#5])



## [0.1.0] - 2022-12-17

### Added

* Added new [`./docs/examples/`](../docs/examples/) directory to the repository,
  with Rust source code for the `complex_app.rs` example program, and the
  generated `complex-app.md` Markdown content for that program. ([#1])

### Changed

* Modify generated Markdown. ([#1])

  The following changes are present in the generated Markdown:

  - Added 'Command-Line Help for {name}' header
  - Include
    [`Arg::get_possible_values()`](https://docs.rs/clap/latest/clap/builder/struct.Arg.html#method.get_possible_values)
    information in arguments and options documentation.
  - Renamed 'Commands' label to 'Subcommands'
  - Added small footer at the bottom stating that the document was generated
    automatically by `clap-markdown`.



## [0.0.1] - 2022-12-13

Initial release of `clap-markdown`.


<!-- v0.0.1 -->

<!-- v0.1.0 -->
[#1]: https://github.com/ConnorGray/clap-markdown/pull/1

<!-- v0.1.1 -->
[#4]: https://github.com/ConnorGray/clap-markdown/pull/4
[#5]: https://github.com/ConnorGray/clap-markdown/pull/5

<!-- v0.1.2 -->
[#8]: https://github.com/ConnorGray/clap-markdown/pull/8

<!-- v0.1.3 -->
[#11]: https://github.com/ConnorGray/clap-markdown/pull/11

<!-- Unreleased -->
[#27]: https://github.com/ConnorGray/clap-markdown/pull/27

[unreleased]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.3...HEAD

[0.1.3]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/ConnorGray/clap-markdown/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/ConnorGray/clap-markdown/releases/tag/v0.0.1
