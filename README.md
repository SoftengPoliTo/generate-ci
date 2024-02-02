# ci-generate

[![Actions Status][actions badge]][actions]
[![CodeCov][codecov badge]][codecov]
[![LICENSE][license badge]][license]
[![dependency status][status badge]][status]

This tool generates either new projects for some build systems or configuration
files for some Continuous Integration systems with the use of templates.

The templates define the layout of a project while its data are inserted at runtime.

Each template contains all necessary files to build a project through a build
system, in addition to Continuous Integration and Docker files to run
tests and create safe containers.

## Supported build systems

| Build system | Languages | Project template | CI style checks | CI build | CI test | CI coverage upload | CI static analysis | CI dynamic analisys | CI license checks |
| - | - | - | - | - | - | - | - | - | - |
| meson | C / C++ | provided | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |:heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| poetry | Python | provided | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark:  | :heavy_check_mark: | :white_check_mark: | :heavy_check_mark: |
| maven | Java | provided | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark:  | :white_check_mark: | :heavy_check_mark: |
| cargo | Rust | offloaded | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| yarn | Javascript / Typescript| offloaded | :x: | :heavy_check_mark:  | :x: | :x: | :x: | :white_check_mark: | :heavy_check_mark:  |

:white_check_mark:: Not necessary for the considered language

## Commands

To see the list of supported commands, run: `ci-generate --help`

Each command has:
- An optional argument to define the license of a project (default: `MIT`)
- An optional argument to set up the branch name (default: `main`)

### cargo

```
$ ci-generate cargo [--docker-image-description DESCRIPTION] [ --lib --ci] [--license LICENSE --branch GITHUB_BRANCH] --name NAME project-path
```

The optional `--docker-image-description` argument sets up the description of a Docker image.
If `--lib` and `--ci` arguments are not inserted, by default, a newly `cargo` project is created through the `cargo new` command.
If the `--lib` option is enabled, the tool generates a `Rust` library project.
If the `--ci` option is enabled, the tool produces only Continuous Integration files.
If both `--lib` and `--ci` options are enabled, Continuous Integration is prioritized,
so only its files are generated.

### maven

```
$ ci-generate maven [--license LICENSE --branch GITHUB_BRANCH] --name NAME project-group project-path
```

### meson

```
$ ci-generate meson [--kind meson-project-kind] [--license LICENSE --branch GITHUB_BRANCH] --name NAME project-path
```

Admitted values for the `kind` argument:

- `c`
- `c++`

### poetry

```
$ ci-generate poetry [--license LICENSE --branch GITHUB_BRANCH] --name NAME project-path
```

### yarn

```
$ ci-generate yarn [--license LICENSE --branch GITHUB_BRANCH] --name NAME project-path
```

## Configuration

It is possible to save a `config.toml` in `${XDG_CONFIG_HOME}/ci-generate` (Usually `~/.config/ci-generate`) with overrides for all the default and optional values, e.g:

``` toml
[default]
license = "BSD-3-Clause"
branch = "master"

[meson]
kind = "c"

[cargo]
lib = false
ci = false
```

This is the default configuration file.
The first section contains all default arguments common to each toolchain.
The other sections contain default arguments **specific** to the toolchain
defined by the section name.

For each toolchain, this configuration will override the default
`license` and `branch` items and it is the same as this command:

```sh
$ ci-generate meson -b master -l BSD-3-Clause
```

The cli arguments take priority over built-in defaults and `config.toml` overrides so

```sh
$ ci-generate meson -l LGPL-2.1
```

would take the `branch = master` from the `config.toml` file and `LGPL-2.1` license
from command line.

## Testing

There are two main types of testing: unit and integration tests.

### Unit test

We can ensure the proper functioning of each unit of code by incorporating
unit tests in the library. The use of unit tests instills trust in the accuracy
and dependability of single code units.

### Integration Test

We use [insta] (https://insta.rs) for our integration tests: a snapshot tests
library for Rust.
Insta serves the purpose of highlighting any content-level differences
among two versions of the same file. In this way, you can easily compare and
contrast the two versions, making informed decisions based on the differences.

Inside the tests directory, you can find several `.rs` files that allow you to
set up tests for each template.
In this directory, there are two other subdirectories.
The first one is called `common` and contains just a single `.rs` file.
This particular file contains the common code which each model uses to run its own
tests. The main goal of this file consists of simplifying the testing process
and avoiding duplicated code among multiple models.
The second subdirectory instead is located within the `repositories` directory
and it is called `snapshots`. It contains all project templates snapshots and
it is used to detect and show future changes.

To execute the tests, launch the following command:

``` sh
cargo insta test --include-hidden
```

When you run the command for the first time, you might notice that the tests return a failure. This is perfectly normal and expected as the snaps haven't been created yet. Including the `--include-hidden` option is crucial when working with templates that have hidden folders, as these folders also need to be captured in the process. This option ensures that all the hidden folders are included while taking a snapshot of the templates.

You can review the content of each individual snapshot through this command

``` sh
cargo insta review --include-hidden
```

When running the `review` command, it is possible to manually accept, reject, 
or skip each generated snapshot.
However, if you want to automatically accept generated snapshots, you can add the `--accept`
option at the end of the command as follows

``` sh
cargo insta test --include-hidden --accept
```

Once base snapshots are created, you can use the `test` command to generate
new snapshots to be compared with. Any differences between new and base
snapshots are displayed on the screen, allowing you to review and analyze
the changes. In case no differences are found, `insta` will return a success message,
indicating that there are no snapshots to be reviewed.

## License

Released under the [MIT License](LICENSES/MIT.txt).

<!-- Links -->
[actions]: https://github.com/SoftengPoliTo/ci-generate/actions
[codecov]: https://codecov.io/gh/SoftengPoliTo/ci-generate
[license]: LICENSES/MIT.txt
[status]: https://deps.rs/repo/github/SoftengPoliTo/ci-generate

<!-- Badges -->
[actions badge]: https://github.com/SoftengPoliTo/ci-generate/workflows/ci-generate/badge.svg
[codecov badge]: https://codecov.io/gh/SoftengPoliTo/ci-generate/branch/master/graph/badge.svg
[license badge]: https://img.shields.io/badge/license-MIT-blue.svg
[status badge]: https://deps.rs/repo/github/SoftengPoliTo/ci-generate/status.svg
