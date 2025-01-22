# Some Custom Title for Complex App

This document contains the help content for the `complex-app` command-line program.

## `complex-app`

An example command-line tool

**Usage:** `complex-app [OPTIONS] [NAME] [COMMAND]`

###### **Subcommands:**

* `test` [alias: `tester`] — does testing things
* `only-hidden-options` — Demo that `Options` is not printed if all options are hidden

###### **Arguments:**

* `<NAME>` — Optional name to operate on

   Longer description

###### **Options:**

* `-c`, `--config <FILE>` [alias: `configuration`] — Sets a custom config file
* `--target <TARGET>`

  Default value: `local`

  Possible values:
  - `local`:
    Do the operation locally
  - `remote`

* `--very-very-verbose` [aliases: `vv`, `vvv`]
* `-d`, `--debug` — Turn debugging information on

   Repeat this option to see more and more debug information.



## `complex-app test` [alias: `tester`]

does testing things

**Usage:** `complex-app test [OPTIONS]`

###### **Options:**

* `-l`, `--list` — lists test values



## `complex-app only-hidden-options`

Demo that `Options` is not printed if all options are hidden

**Usage:** `complex-app only-hidden-options`



