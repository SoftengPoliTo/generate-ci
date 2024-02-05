# ci-generate

This tool makes use of the `generate-ci` library to create either new build
systems projects or only configuration files for some Continuous Integration
systems.
Templates used to create every file are contained in the tool's binary.

It can be configured both through CLI parameters and through a TOML file
which avoids repeating the same entry patterns many times.

## Commands

To see the list of supported commands, run: `ci-generate --help`

Each command has:
- An optional argument to define the license of a project (default: `MIT`)
- An optional argument to set up the branch name (default: `main`)

## Building

To build all commands run:

```sh
cargo build
```

To build **only** specific commands, for example `cargo` and `poetry` run:

```sh
cargo build --no-default-features --features cargo,poetry
```

### cargo

```
$ ci-generate cargo [--docker-image-description DESCRIPTION] [ --lib --ci] [--license LICENSE --branch GITHUB_BRANCH] --name NAME project-path
```

The optional `--docker-image-description` argument sets up the description of a Docker image.
If `--lib` and `--ci` arguments are not inserted, by default, a newly `cargo` project is created through the `cargo new` command.
If the `--lib` option is enabled, the tool generates a `Rust` library project.
If the `--ci` option is enabled, the tool produces only Continuous Integration files.
If both `--lib` and `--ci` options are enabled, Continuous Integration is prioritized,
so only those files will be generated.

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
`license` and `branch` items and those settings corresponds to this CLI command:

```sh
$ ci-generate meson -b master -l BSD-3-Clause
```

The cli arguments take priority over built-in defaults and `config.toml` overrides so

```sh
$ ci-generate meson -l LGPL-2.1
```

would take the `branch = master` from the `config.toml` file and `LGPL-2.1` license
from command line.

## License

Released under the [MIT License](../LICENSES/MIT.txt).
