# ci-generate

[![Actions Status][actions badge]][actions]
[![LICENSE][license badge]][license]
[![dependency status][status badge]][status]

This tool generates either new projects for some build systems or configuration
files for some Continuous Integration with the use of templates.

Templates define the layout for a project and allow developers to insert data
at runtime.

Each template contains all files necessary to build a project with a build
system, in addition to Continuous Integration and Docker files used to run
tests and implement further checks.

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

Each command has an optional argument to define a license and an optional argument to
 override the project name instead of using the last component of the project-path.
 The default value for the license argument is `MIT`.

### cargo

```
$ ci-generate cargo [--license LICENSE --name NAME --branch GITHUB_BRANCH] project-path
```

### maven

```
$ ci-generate maven [--license LICENSE --name NAME --branch GITHUB_BRANCH] project-group project-path
```

### meson

```
$ ci-generate meson [--kind meson-project-kind] [--license LICENSE --name NAME --branch GITHUB_BRANCH] project-path
```

Admitted values for the `kind` argument:

- `c`
- `c++`

### poetry

```
$ ci-generate poetry [--license LICENSE --name NAME --branch GITHUB_BRANCH] project-path
```

### yarn

```
$ ci-generate yarn [--license LICENSE --name NAME --branch GITHUB_BRANCH] project-path
```

## Configuration

It is possible to save a `config.toml` in `${XDG_CONFIG_HOME}/ci-generate` (Usually `~/.config/ci-generate`) with overrides for
 all the default and optional values, e.g:

``` toml
[default]
license = "BSD-3-Clause"

[meson]
kind = "c++"
```

Will override the default `license` and `meson.kind` configuration items and it would be equivalent to call:

``` sh
$ ci-generate meson -k c++ -l BSD-3-Clause
```

The cli arguments take priority over the built-in defaults and the `config.toml` overrides so
```
$ ci-generate meson -l LGPL-2.1
```

Would take the `kind = c++` from the `config.toml` and `LGPL-2.1` from the command line.

## Testing

We use [insta] (https://insta.rs), a library of snapshots tests for Rust, as integration tests.
To run tests, launch the following command:

``` sh
cargo insta test 
```

is executed, snaps are created with the current content of the files being tested, which will be considered as basic snapshots. 
Once the basic snapshots have been created, you can run the previous command again to perform a content test by producing new snapshots that will then be compared with the basic snapshots.

### Updating insta tests

When the comparison is made between snaps, tests return a **failure** if there are changes from the base snap, or return a **success** if the two snaps coincide. 
In case of failure, the command 

``` sh
cargo insta review --include-hidden
```

will allow you to review the differences between the two snaps and possibly accept the changes. The `--include-hidden` option is important to add in this context because it allows you to verify the snap of hidden files or folders, present in template creation for example.

With this in mind, it is possible to test a series of snapshots of different contents.

#### Configuration files

It is possible to test the content of a custom configuration `.toml` file. Refer to the section [Configuration](#configuration) to set up a tamplate with personal `config.toml`. For example, it is possible to use:

``` sh
ci-generate -c tests/repositories/config_template/config.toml meson -l=APL-1.0 -b=master tests/repositories/config_template/meson_template_config
```
to create a meson template with a previously created configuration file.

#### Templates

It is possible to perform snapshot tests on the templates created. For example:  

``` sh
ci-generate cargo --docker-image-description=docker_image --license=EUPL-1.2 --name=Project --branch=main tests/repositories/cargo_template
```
The command mentioned above allows the creation of a `cargo` template. Through snapshot tests, it is possible to create a basic version of the snap and later verify any changes by rerunning the insta test command. 

## License

Released under the [MIT License](LICENSES/MIT.txt).

<!-- Links -->
[actions]: https://github.com/SoftengPoliTo/ci-generate/actions
[license]: LICENSES/MIT.txt
[status]: https://deps.rs/repo/github/SoftengPoliTo/ci-generate

<!-- Badges -->
[actions badge]: https://github.com/SoftengPoliTo/ci-generate/workflows/ci-generate/badge.svg
[license badge]: https://img.shields.io/badge/license-MIT-blue.svg
[status badge]: https://deps.rs/repo/github/SoftengPoliTo/ci-generate/status.svg
