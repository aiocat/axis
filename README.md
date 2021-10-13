# axis

Modern and minimal tool for searching a text in folder.

## Installation

Since there is no binary-release for axis, You must compile it from source.

### Requirements

- `rustc` version >= 1.55.0
- `cargo`

### Step-by-Step

- Download the source code from gitlab via git or from releases.
- Open a terminal in the source directory.
- Run `cargo build --release` command.
- Check `/target/release` folder for binary.

## Usage(s)

`axis [--p|--path=<path>] [--w|--wait] <text>`

### Example(s)

- `axis "text to find" --path="~/docs"`
- `axis "text to find" --p="~/docs"`
- `axis "text to find" --p="~/docs" --w`
- `axis "text to find" --w`
- `axis "text to find"`

## Found a Bug / Error?

If you found a bug or an error, please create a new issue at gitlab repository.

## Contributing

If you want to contribute this project:

- Make sure you add the comments for your codes.
- Please do not something useless.

## Authors

- [Modeminal](https://gitlab.com/modeminal)

## License

This project is distributed under GPLv3 license.

## Project status

Under development.
