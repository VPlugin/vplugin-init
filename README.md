# vplugin-init
`vplugin-init` is a command line utility to create new plugin templates, compatible with the VPlugin framework. It supports Rust and C/C++ support is under development too. This program supports the [VPlugin 1.0 Module specification](https://github.com/AndroGR/VPlugin), and Rust support is also available.

`vplugin-init` will NOT make use of Cargo since there is no API to use it, unless we fall back to `std::process::Command` which is a really bad trick. If you wish to use Cargo, then first initialize the directory with Cargo and then run `vplugin-init`.

# Installation
You can simply clone this repository and then build using Cargo:
```sh
$ git clone https://github.com/AndroGR/vplugin-init.git && cd vplugin-init/
$ cargo install --path .
```
This assumes `~/.cargo/bin` is in your `PATH` variable.

# Usage
In order to create a new plugin, you should run `vplugin-init`, passing the data required:
```
$ vplugin-init --name example-plugin --version 0.1.0 --directory example-plugin/ 
```
Parameters are as follows:
- `--name`: The name of your plugin.
- `--version`: The version of your plugin.
- `--directory`: The directory to create the plugin (Will be created if it doesn't exist.)

# License
This software is licensed under the GNU GPLv3. For more details, see [COPYING](./COPYING).