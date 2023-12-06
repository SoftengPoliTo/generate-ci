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

There are two main types of testing: unit testing and integration testing.

### Unit test

We can ensure the proper functioning of each unit of code by incorporating unit tests in the library. The use of unit tests instills trust in the accuracy and dependability of our code, resulting in a superior end product.

### Integration Test

We confidently use [insta] (https://insta.rs) as our integration tests, which is a powerful library of snapshot tests for Rust.
Insta serves the purpose of highlighting any content-level differences between two versions of the same file. This way, you can easily compare and contrast the two versions and make informed decisions based on the differences.

In the tests folder, you can find several .rs files that enable you to set up tests for each template individually. 
In this directory, two other subdirectories are located. The first is called 'common' and contains an .rs file. This particular file is responsible for using common code to run the tests for each template. This means that we use this file to simplify the testing process and avoid duplicating code on multiple templates.
The second subdirectory is located within the 'repositories' folder and is called 'snapshots'. It contains snapshots for all project templates and is used to identify future changes. 

To execute the tests, launch the following command:

``` sh
cargo insta test --include-hidden
```

When you run the command for the first time, you might notice that the tests return a failure. This is perfectly normal and expected as the snaps haven't been created yet. Including the "--include-hidden" option is crucial when working with templates that have hidden folders, as these folders also need to be captured in the process. This option ensures that all the hidden folders are included while taking a snapshot of the templates.

You can review the contents of each individual snap using this command

``` sh
cargo insta review --include-hidden
```

When running the review command, it is possible to manually accept, reject, or skip each snap that is generated. 
However, if you want to automatically accept the generated snaps, you can add the "--accept" option at the end of the command while launching the tests, as follows

``` sh
cargo insta test --include-hidden --accept
```
After creating the base snapshots, you can use the test command to generate new snapshots for comparison. Any differences between the new and previous snapshots will be displayed on the screen, allowing you to review and analyze the changes. In case no differences are found, insta will return a success message, indicating that there are no snapshots that require review.

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
