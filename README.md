# PMR CLI (RS)

**Rust** rewrite of [jascha030/pmr-cli](https://github.com/jascha030/pmr-cli), which is _a small project management CLI utility, written in PHP to store project management related links._

The version mentioned above was originally intended as a prototype to be re-written in rust, I hadn't found the motivation to do so.

***Up till now that is 🎉!***

## Installation

```sh
cargo install pmr
```

## Commands

The CLI tool consists of two console commands.

### Init

Use the init command to create a `.pm.toml` resource file.

```sh
Usage: pmr init: [OPTIONS]

Options:
  -h, --help Print help information
```

This will run you through a couple of questions asking if you want to add a project url for any of the following
categories:

* **Task Management**
* **Time tracking**
* **Git repo**

### Open

The open command provides quick access to your provided resources.

```sh
Usage: pmr open [OPTIONS]

Options:
  -a, --all   Open all resources
  -h, --help  Print help information
```

## License 

This composer package is an open-sourced software licensed under the [MIT License](https://github.com/jascha030/pmr-rs/blob/master/LICENSE)

