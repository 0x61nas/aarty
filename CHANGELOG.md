# Changelog

All notable changes to this project will be documented in this file.


> [!Note]
> this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [unreleased]

### 🐛 Bug Fixes

- *(deps)* Update rust crate image to 0.25 ([#62](https://github.com/0x61nas/aarty/issues/62))

### ⚙️ Miscellaneous Tasks

- *(cliff)* Setup git-cliff
- *(changelog)* Introduce the changelog
- *(release)* Configure release plz
- *(changelog)* Re-generate changelog





## [0.7.0-alpha.2] - 2024-03-08

### ⚙️ Miscellaneous Tasks

- *(docker)* Use alpine:latest as a base image
- *(docker)* Just copy the needed files
- *(cargo)* Bump the ver to 0.7.0-alpha.2
- *(cargo)* Setup the release profile
- *(docker)* Create docker ignore list
- *(docker)* Patch Cargo.toml
- *(release)* Release plz
- *(release)* Rename the workflow





> **Full Changelog**: https://github.com/0x61nas/aarty/compare/0.7.0-alpha.1...v0.7.0-alpha.2

## [0.7.0-alpha.1] - 2024-02-20

### 🐛 Bug Fixes

- *(docker)* Fix urls
- *(docker)* Rm man
- *(cargo)* Include benches
- *(docker ci)* Add the permissions

### ⚙️ Miscellaneous Tasks

- *(justfile)* Depth 70
- *(docker)* Creat docker file
- *(docker)* Create docker workflow
- *(cargo)* Bump the ver to 0.7.0-alpha.1





> **Full Changelog**: https://github.com/0x61nas/aarty/compare/0.6.1-alpha.2...0.7.0-alpha.1

## [0.6.1-alpha.2] - 2024-02-02

### ⚙️ Miscellaneous Tasks

- *(bench)* Create benches
- *(msrv)* Bump msrv to 1.70
- *(license)* Allow Zlib and Unicode-DFS-2016
- *(cargo)* Bump the ver to 0.6.1-alpha.2





> **Full Changelog**: https://github.com/0x61nas/aarty/compare/0.6.1-alpha.1...0.6.1-alpha.2

## [0.6.1-alpha.1] - 2024-01-30

### 🐛 Bug Fixes

- *(mergify)* Remove the conflicted config

### 📚 Documentation

- The code is very readable btw
- *(readme)* Re-generate the readme
- Add documentation based on the youtube video ([#10](https://github.com/0x61nas/aarty/issues/10))
- Add the missing docs
- Fix typo

### ⚙️ Miscellaneous Tasks

- Automatically cancel superseded Actions runs ([#5](https://github.com/0x61nas/aarty/issues/5))
- Fix typos ([#17](https://github.com/0x61nas/aarty/issues/17))
- *(check)* Update the branch name and chang the msrv to 1.65
- *(nostd)* Rm nostd.yml
- *(safety)* Update the branch name
- *(schedule)* Change the branch name
- *(github)* Codeowners create
- *(github)* Create a pull request template
- *(github)* Put useless funding stuff
- *(mergify)* Create `mergify` config
- *(github)* Create issues templates
- *(safety)* Remove loom job
- *(lint)* Deny rust2018idoms and none_ascii_ident and missing debug impl
- *(lint)* Warn missing docs
- *(cargo)* Bump the ver to 0.6.1-alpha.1

### Nit

- Selecting direct minimal versions flag is -Zdirect-minimal-versions ([#16](https://github.com/0x61nas/aarty/issues/16))

### Clean

- Rm DOCS.md





> **Full Changelog**: https://github.com/0x61nas/aarty/compare/0.6.1...0.6.1-alpha.1

## [0.6.1] - 2024-01-28

### 🚀 Features

- *(bin)* -v works

### 🐛 Bug Fixes

- Fix typos

### ⚙️ Miscellaneous Tasks

- *(cargo)* Bump the ver to 0.6.1





> **Full Changelog**: https://github.com/0x61nas/aarty/compare/0.6...0.6.1

## [0.6] - 2024-01-28

### 🚀 Features

- *(rgba)* Make fields pub

### 🐛 Bug Fixes

- Makey cargo-hack happy
- *(minimal-versions)* Add rayon v1.1 and cfg-if v0.1.2 as a transitive dependencies
- *(image)* Make cargo-hack happy
- *(bin)* Require colors feature to build the binary

### 🚜 Refactor

- Annotate structs with non exhaustive
- Use new api

### 📚 Documentation

- Its --help sorry
- Fix the examples
- *(readme)* Re-generate the readme
- Fix typos
- Update code examples

### ⚙️ Miscellaneous Tasks

- *(bin)* Require the image feature to be enabled when building the bin
- *(justfile)* Add cargo-hack tO check recipe
- *(cargo)* Bump the ver to 0.6
- *(cargo)* Update cargo.lock

### Clean

- Rm -rf .github





> **Full Changelog**: https://github.com/0x61nas/aarty/compare/0.5...0.6

## [0.5] - 2024-01-27

### 🚀 Features

- *(crate)* Provide a lib and add the reverse mode
- *(empty)* Add `is_empty` method to `TextImage`
- *(serde)* Impl `serde::Serialize` and `serde::Deserialize`
- *(args)* Use bit manipulation to handle our flags
- *(api)* Take slice instead of ownership the sym set
- Make `image` crate optional and add a uniform api
- *(api)* Improve the colors api
- Use less memory
- *(bin)* Read from stdin if there is no provided path
- Config/buffer
- Text_image impl
- *(text_image)* Add insert/put/get functions
- *(sympols)* Empty set
- Re-export stuff
- *(resize filter)* Add he ability to change the resize filter

### 🐛 Bug Fixes

- *(deps)* Update rust crate log to 0.4.20
- *(deps)* Update rust crate image to 0.24.7
- *(deps)* Update rust crate clap to 4.3.21
- *(char selection)* Handle the edge cases
- *(serde)* Implement `_no_ref` feature and fix `impl_serde`
- *(render)* Increment after check
- *(render)* Don't render the transparent pixels

### 🚜 Refactor

- *(args)* Remove the `clap` crate and use our custom parser
- Remove `log` and `pretty_env_logger` and `colored` crates from our deps
- Replace bool's with one u8
- *(args)* Be smarter
- *(bin)* Handle the possible errors
- *(bin)* 2mb
- *(serde)* Rename `impl_serde` feature to `serde`
- *(config)* Re-name `bc` to `background`
- *(text_image)* Rm the unused lifetime
- Remove unused import

### 📚 Documentation

- *(crate)* The crate intro
- Add the (main) structs/functions docs
- Modules docs
- Fix typos
- *(sympols)* Sympols set docs
- *(color)* ANSI color docs
- *(sympols)* Pub EMPTY_CHAR
- *(text_image)* Text image docs
- H1 title
- Correct the example
- *(readme)* Generate the readme
- Fix bang
- *(readme)* Re-generate the readme

### ⚙️ Miscellaneous Tasks

- Remove the garbage
- *(cargo)* Disables binary auto discovery
- *(bin)* Remove the unnecessary sub-dir
- *(cargo)* Un-specifying the `image` crate patch version
- *(git)* Remove `Cargo.lock` entry
- *(cargo)* Track the `Cargo.lock` file
- *(typos)* Create `typos` config
- *(just)* Just create the justfile
- *(git)* Remove AUR submodule
- *(deny)* Create `deny` config
- *(committed)* Create the committed config
- *(codespell)* Create the codespell ignore file
- *(typos)* Ignore .git dir
- *(justfile)* Run codespell in check recipe
- *(justfile)* Just patch the generaed readme
- *(samples)* RM UN-NEEDED SAmples
- Add the contributing guide
- *(readme)* Create readme template
- *(samples)* Add the sample images
- *(cargo)* Update the crate metadata
- *(cargo)* Bump the ver to 0.5
- *(cargo)* Update cargo.lock
- *(cargo)* Msrv is 1.65

### Clean

- Remove unused Fragment struct
- Remove unused filed
- Rm the screenshots dir
- Rm -rf images





> **Full Changelog**: https://github.com/0x61nas/aarty/compare/0.4.9...0.5

## [0.4.9] - 2023-07-16

### 🐛 Bug Fixes

- *(deps)* Update rust crate image to 0.24.6
- *(deps)* Update rust crate clap to 4.3.1
- *(deps)* Update rust crate pretty_env_logger to 0.5.0
- *(deps)* Update rust crate log to 0.4.18
- *(deps)* Update rust crate clap to 4.3.2
- *(deps)* Update rust crate clap to 4.3.9
- *(deps)* Update rust crate log to 0.4.19
- *(deps)* Update rust crate colored to 2.0.4
- *(deps)* Update rust crate clap to 4.3.12

### ⚙️ Miscellaneous Tasks

- *(aur)* Remove the AUR submodule
- *(cargo)* Bump the crate version to 0.4.9





> **Full Changelog**: https://github.com/0x61nas/aarty/compare/0.4.8...0.4.9

## [0.4.8] - 2023-03-31

### 🐛 Bug Fixes

- *(deps)* Update rust crate clap to 4.2.1

### ⚙️ Miscellaneous Tasks

- *(Mergify)* Configuration update





> **Full Changelog**: https://github.com/0x61nas/aarty/compare/0.4.7...0.4.8

<!-- generated by git-cliff -->
