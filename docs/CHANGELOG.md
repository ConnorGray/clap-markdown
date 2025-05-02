# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

## [0.1.5] — 2025-05-01

### Added

* `clap-markdown` will now list subcommand and argument aliases in the generated
  Markdown. ([#36], authored by [@willemneal](https://github.com/willemneal),
  and [#41]).

  This can be controlled with the new `MarkdownOptions::show_aliases()` option
  setter.

## [0.1.4] — 2024-06-15

### Added

* Added `help_markdown_custom()` and `help_markdown_command_custom()`, for
  customizing the built Markdown using a new `MarkdownOptions` configuration
  structure.
  ([#25], co-authored-by [@morgante](https://github.com/morgante)
  and [@keturiosakys](https://github.com/keturiosakys))

  Supported customization options include:

  - Contents of the command title header.
  - Whether to show the footer explaining the documentation was generated with
    `clap-markdown`.
  - Whether to show the command overview table of contents.

* Added new CI build and test action to the repository, so that new PRs will be
  required automatically to pass all tests.
  ([#24])

### Changed

* `clap-markdown` will now respect the `Command.display_name` property, if set,
  and use it in the rendered Markdown when showing the name and usage of the
  command.
  ([#27],
  co-authored-by [@keturiosakys](https://github.com/keturiosakys)
  and [@hatchan](https://github.com/hatchan))

* The possible values of a flag that can only either be present or absent
  (true or false) will no longer be shown.
  ([#22], contributed by [@ilyagr](https://github.com/ilyagr))

* Hidden arguments (`Arg.hide == true`) will no longer be included in the
  rendered Markdown.
  ([#22], contributed by [@ilyagr](https://github.com/ilyagr))

* Long help text for commands will now be included in the rendered output.
  ([#26], contributed by [@morgante](https://github.com/morgante))

* Long help text for options will now be included in the rendered output.
  ([#23], contributed by [@ilyagr](https://github.com/ilyagr))



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

<!-- v0.1.4 -->
[#22]: https://github.com/ConnorGray/clap-markdown/pull/22
[#23]: https://github.com/ConnorGray/clap-markdown/pull/23
[#24]: https://github.com/ConnorGray/clap-markdown/pull/24
[#25]: https://github.com/ConnorGray/clap-markdown/pull/25
[#26]: https://github.com/ConnorGray/clap-markdown/pull/26
[#27]: https://github.com/ConnorGray/clap-markdown/pull/27

<!-- v0.1.5 -->
[#36]: https://github.com/ConnorGray/clap-markdown/pull/36
[#41]: https://github.com/ConnorGray/clap-markdown/pull/41

<!-- Unreleased -->

[Unreleased]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.5...HEAD

[0.1.5]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/ConnorGray/clap-markdown/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/ConnorGray/clap-markdown/releases/tag/v0.0.1
