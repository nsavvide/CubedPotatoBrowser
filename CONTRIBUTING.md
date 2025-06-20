# Contributing Guide

Hi! We, the maintainers, are really excited that you are interested in contributing to this project. Before submitting your contribution though, please make sure to take a moment and read through the [Code of Conduct](CODE_OF_CONDUCT.md), as well as the appropriate section for the contribution you intend to make:

- [Issue Reporting Guidelines](#issue-reporting-guidelines)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Development Guide](#development-guide)

## Issue Reporting Guidelines

- The issue list of this repo is **exclusively** for bug reports and feature requests of this project only. General cef issues should go to [CEF issue list](https://github.com/chromiumembedded/cef/issues).

- If you have any question, please visit [CEF Forum](https://magpcss.org/ceforum/index.php) for answers.

- Issue must follow the issue template format. If it's a bug report, reproduce steps must be provided. Otherwise, the issue will be closed.

## Pull Request Guidelines

- The title and commits should follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0-beta.4/).

- Add changes in CHANGELOG.md. It should be the same as the title.

- It's OK to have multiple small commits as you work on the PR - we will let GitHub automatically squash it before merging.

## Development Guide

### Architecture

Like many FFI crates, there are two crates in this repo: `cef-rs` in root and `cef-sys` in sys directory. `cef-sys` contains C APIs generated by `bindgen.sh`. You shouldn't edit the code directly in this crate. `cef-rs` is where type safe bindings land. Most features and implementation should go here.

### Types

There are a few types we recommend you learn first before adding any implementation. This could help you understand the interface of CEF better. We also welcome everyone who can add tests for these types:

- [`cef::string`](https://docs.rs/cef/latest/cef/rc/index.html)
- [`cef::rc`](https://docs.rs/cef/latest/cef/string/index.html)
