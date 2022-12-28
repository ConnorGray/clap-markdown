# Command-Line Help for `complex-app`

This document contains the help content for the `complex-app` command-line program.

**Command Overview:**

* [`complex-app`↴](#complex-app)
* [`complex-app test`↴](#complex-app-test)

## `complex-app`

An example command-line tool

**Usage:** `complex-app [OPTIONS] [NAME] [COMMAND]`

###### **Subcommands:**

* `test` — does testing things

###### **Arguments:**

* `<NAME>` — Optional name to operate on

###### **Options:**

* `-c`, `--config <FILE>` — Sets a custom config file
* `--target <TARGET>`

  Default value: `local`

  Possible values:
  - `local`:
    Do the operation locally
  - `remote`

* `-d`, `--debug` — Turn debugging information on



## `complex-app test`

does testing things

**Usage:** `complex-app test [OPTIONS]`

###### **Options:**

* `-l`, `--list` — lists test values



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
