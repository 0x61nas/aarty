# Contributing

A big welcome and thank you for considering contributing to `aarty`! It is people like you that make it a reality for users in the open source community.

Reading and following these guidelines will help us make the contribution process easy and effective for everyone involved. It also communicates that you agree to respect the time of the developers managing and developing this project. In return, we will reciprocate that respect by addressing your issue, assessing changes, and helping you finalize your pull requests.

## Quicklinks

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
  - [Issues](#issues)
  - [Pull Requests](#pull-requests)
- [License](#license)

## Code of Conduct

We take our open source community seriously and hold ourselves and other contributors to high standards of communication. By participating and contributing to this project, you agree to uphold our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Getting Started

Contributions are made to this repo via Issues and Pull Requests (PRs). A few general guidelines that cover both:

- First, discuss the change you wish to make via creating an [issue](https://github.com/0x61nas/aarty/issues/new/choose), [email](mailto:anas.elgarhy.dev@gmail.com), or any other method with the owners of this repository before making a change.
- Search for existing issues and PRs before creating your own.
- We work hard to make sure issues are handled in a timely manner but, depending on the impact, it could take a while to investigate the root cause. A friendly ping in the comment thread to the submitter or a contributor can help draw attention if your issue is blocking.

### Issues

Issues should be used to report problems with the project, request a new feature, or discuss potential changes before a PR is created. When you create a new issue, a template will be loaded that will guide you through collecting and providing the information we need to investigate.

If you find an issue that addresses the problem you're having, please add your own reproduction information to the existing issue rather than creating a new one. Adding a [reaction](https://github.blog/2016-03-10-add-reactions-to-pull-requests-issues-and-comments/) can also help be indicating to our maintainers that a particular problem is affecting more than just the reporter.

### Pull Requests

* For changes that address core functionality or would require breaking changes (e.g. a major release), it's best to open an issue to discuss your proposal first. This is not required but can save time creating and reviewing changes.
* Be accompanied by a complete Pull Request template (loaded automatically when a PR is created).


#### Commits

* All commits in a Pull Request must be [signed](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits) and Verified by Github.
* All commits in a Pull Request must follow our [commit message convention](#commit-messages).
* All authors of all commits in a Pull Request must abide by our [Code of Conduct](CODE_OF_CONDUCT.md).
* In most cases, a Pull Request should contain a single commit or tree commits at most. Please squash multiple commits into one before submitting.
* We follow a linear commit history in our git repositories, a pull request cannot contain merge commits. To apply upstream changes to a branch, please rebase it to the base branch.

#### Branch names

We follow the `type/change` format for branch names, for example `feat/time-backend` or `fix/simple-backend`.
There is no strict requirement to the `change` section as long as it doesn't contain `/` and uses ASCII.

#### Commit Messages

Each commit message consists of a header, a body and a footer. The header includes a type, a scope and a subject:

```
   <type>(<scope>): <subject>
   <BLANK LINE>
   <body>
   <BLANK LINE>
   <footer>
```

* `<type>(<scope>): <subject>` must not be longer that 100 characters.
* type is required, must be in lower case and have one of the below values.
  - build: changes that affect our build system or external dependencies
  - ci: changes to our continuous integration configuration files
  - feat: a new feature
    - Please add a implementation scope to a feature commit `feat(parser):`
    - If a commit affects multiple implementations, please break it into two commits.
  - fix: a fix to a bug in an existing feature
    - Please add an implementation scope to a bug fix commit `fix(parser):`
    - If a commit affects multiple implementations, please break it into two commits.
  - refactor: code change that neither fixes a bug nor adds a feature
    - Please add an implementation scope to a refactor commit `refactor(parser):`
    - If a commit affects multiple implementations, please break it into two commits.
  - style: changes that do not affect the meaning of the code (white-space, formatting etc.)
  - test: add missing tests or correct existing tests
  - docs: a documentation only change
  - chore: some minor change that doesn't fall in any of the other types

#### Typical pull request workflow

In general, we follow the "[fork-and-pull](https://github.com/susam/gitpr)" Git workflow:

1. Fork the repository to your own GitHub account.
2. Clone the project to your local environment.
3. Create a branch locally with a succinct but descriptive name.
4. Make sure you have [Rust](https://rustup.rs) and [Just](https://just.systems) installed.
5. Run `just setup` to install the development tools.
6. Start committing changes to the branch.
7. Add your tests or update the existing tests according to the changes and check if the tests are passed.

```sh
just t
```

8. Make sure [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy) don't show any errors/warnings.

```sh
cargo fmt --all -- --check --verbose
```
```sh
cargo clippy --verbose -- -D warnings
```
or you can run both of them at once:
```sh
just lint
```

9. Push changes to your fork.
10. Open a PR in our repository and follow the [PR template](./.github/PULL_REQUEST_TEMPLATE.md) so that we can efficiently review the changes.
11. Wait for approval from the repository owners. Discuss the possible changes and update your PR if necessary.
12. The PR will be merged once you have the sign-off of the repository owners.

## Helpful References

* [How to contribute to open source](https://opensource.guide/how-to-contribute/)

## License

By contributing, you agree that your contributions will be licensed under [The MIT License](./LICENSE) or [Unlicense License](./LICENSE-UNLICENSE), at your option.
