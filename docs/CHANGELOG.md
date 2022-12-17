# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]



## [0.1.0] - 2022-12-17

### Added

* Added new [`./docs/examples/`](../docs/examples/) directory to the repository,
  which Rust source code for the `complex_app.rs` example program, and the
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

[unreleased]: https://github.com/ConnorGray/clap-markdown/compare/v0.1.0...HEAD

[0.1.0]: https://github.com/ConnorGray/clap-markdown/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/ConnorGray/clap-markdown/releases/tag/v0.0.1