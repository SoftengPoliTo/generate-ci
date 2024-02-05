# generate-ci

[![Actions Status][actions badge]][actions]
[![CodeCov][codecov badge]][codecov]
[![LICENSE][license badge]][license]
[![dependency status][status badge]][status]

This library generates specific build systems
or Continuous Integration configuration files with the use of templates.

Templates define a project layout while its data is inserted
at runtime.

Each template can be:
- A configuration file needed to build a project
- A file needed to set up a Continuous Integration system
- A `Dockerfile` used to create a docker image

## Supported build systems

We support the most common build systems and we have set up a code structure
which allows to add more of them with simplicity just modifying determined
files. We will call this build systems with the term `toolchain`.

| Build system | Languages | Project template | CI style checks | CI build | CI test | CI coverage upload | CI static analysis | CI dynamic analisys | CI license checks |
| - | - | - | - | - | - | - | - | - | - |
| meson | C / C++ | provided | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |:heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| poetry | Python | provided | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark:  | :heavy_check_mark: | :white_check_mark: | :heavy_check_mark: |
| maven | Java | provided | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark:  | :white_check_mark: | :heavy_check_mark: |
| cargo | Rust | offloaded | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| yarn | Javascript / Typescript| offloaded | :x: | :heavy_check_mark:  | :x: | :x: | :x: | :white_check_mark: | :heavy_check_mark:  |

:white_check_mark:: Not necessary for the considered language

## API

In the [src](src/) directory, you can find:
- An API which creates a build system project in addition to determined
Continuous Integration systems files. This API is called `create_project`.
- An API which creates **only** Continuous Integration files called `create_ci`

These APIs needs a specific information to create a project being
called. Below an example of its usage:

```rust
// Saves necessary information for project creation
let data = TemplateData::new(&project_path, "project-name") // Project path and name
    .license("MIT") // Project license
    .branch("main"); // Default branch name

// Constructs `cargo` instance and creates the project
Cargo::new().create_project(data)
```

The code above first defines the necessary information to create a project through
`TemplateData` structure and then constructs the `cargo` project which
calls `create_project` API.
All files will be created within `path` directory with `name` as project name,
`MIT` as license, and `main` as default branch.

## Testing

There are two main types of testing: unit and integration tests.

### Unit test

We can ensure the proper functioning of each unit of code by incorporating
unit tests in the library. The use of unit tests instills trust in the accuracy
and dependability of single code units.

### Integration Test

We use [insta] (https://insta.rs) for our integration tests: a snapshot tests
library for Rust.
`insta` serves the purpose of highlighting any content-level differences
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
[actions]: https://github.com/SoftengPoliTo/generate-ci/actions
[codecov]: https://codecov.io/gh/SoftengPoliTo/generate-ci
[license]: LICENSES/MIT.txt
[status]: https://deps.rs/repo/github/SoftengPoliTo/generate-ci

<!-- Badges -->
[actions badge]: https://github.com/SoftengPoliTo/generate-ci/workflows/generate-ci/badge.svg
[codecov badge]: https://codecov.io/gh/SoftengPoliTo/generate-ci/branch/master/graph/badge.svg
[license badge]: https://img.shields.io/badge/license-MIT-blue.svg
[status badge]: https://deps.rs/repo/github/SoftengPoliTo/generate-ci/status.svg
