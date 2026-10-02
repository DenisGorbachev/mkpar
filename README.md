# mkpar

Create missing parent directories for a path, leaving the final component untouched.

For example, `mkpar logs/app/output.log` creates `logs/app` without creating or modifying `output.log`.

## Installation

Requires Rust 1.85 or later.

```shell
cargo install --locked mkpar
```

## Usage

```shell
mkpar <PATH>
```

```shell
mkpar logs/app/output.log
mkpar "logs/my app/output.log"
mkpar -- -logs/output.log
mkpar --help
mkpar --version
```

Relative paths are resolved from the current working directory. Existing directories are accepted. A bare filename or a path without a parent succeeds without creating anything. The final component is never created, even when the path ends in a separator.

Successful execution is silent and exits with status 0. Filesystem errors, such as insufficient permissions or a parent component that is a file, are reported on standard error and exit with status 1. Invalid command-line arguments exit with status 2. If directory creation fails partway through, directories already created are left in place.

## Library

```shell
cargo add mkpar
```

```rust
use mkpar::create_parent_directories;
use std::io;

fn main() -> io::Result<()> {
    create_parent_directories(&"logs/app/output.log")
}
```

The function accepts a reference to a type implementing `AsRef<Path>` and returns `std::io::Result<()>`.

See the [API documentation](https://docs.rs/mkpar) and [source code](https://github.com/DenisGorbachev/mkpar).

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
