# The SPac Documentation
## 2. Setting Up

### 2.1. Installinng SPac

SPac was built with [Cargo](https://doc.rust-lang.org/cargo/) and can be installed with it:
```sh
cargo install --git https://github.com/Science52101/spac.git
```
It also can be uninstalled with:
```sh
cargo uninstall spac
```

### 2.2. Configuring The Basics

SPac saves its repositories, configuration and temporary files at the SUD (SPac User Directory).
By running the `spac` command, its directories and files will be created.

Before running `spac`, you can edit the file at `$HOME/.spac_user_dir` (or `%APPDATA%/.spac_user_dir` for MS Windows) and change the SUD path from `.` to your preferred path.
(Some recommended SUD paths are `~/.spac_sud` and `~/.spac_user`. `~` needs to be changed for your `$HOME` path or any other preferred path for Windows.) 
This configuration method prevents SPac directories and files to be created at `.` before the change of the SUD.

Another way to change the SUD path is to use the `spac sud` command with your preferred path for SUD.
Please check the given example below:
```sh
spac sud ~/.spac_user
```
This method, however, uses the `spac` command, so, before the SUD path is changed, the SPac directories and files will be created at `.`.
