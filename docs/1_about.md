# The SPac Documentation
## 1. About

SPac is a software repository management and use interface.

### 1.1. The Reasons

- Setting up software from a repository can be laborious.
- Many package managers rely on platforms, environments or purposes.
- Many package managers can be complex for package development.

### 1.2. The Implementation

Since most of Science52101 software is uploaded to GitHub, SPac uses Git to manage its repositories.
Its installation instructions are given for every platform by system commands in the `.spac` directory within the repository with minimum configuration information.

SPac is being developed with [Rust](https://www.rust-lang.org) and [Cargo](https://doc.rust-lang.org/cargo).
The safety of Rust with the variety of crates that can be used with Cargo make the development easier.

The SPac CLI provides an easy use of the implemented functionality.
Each command is designed to be recognizable and short for a better experience.

### 1.3. The Goals

To be facilitated:

- [x] Download of repositories.
- [x] Management of repositories.
- [x] Installation of software from repositories.
- [x] Use of installed software from repositories.
- [ ] Dependency management for repositories.
- [ ] Uninstallation of unused dependencies.
- [ ] Management of dependencies which are incompatible to SPac.
