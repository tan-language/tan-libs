# Tan Foreign Libraries

A collection of Rust libraries that support several Tan Library modules.
Includes core libraries that are statically linked, and other libraries that are
dynamically linked on demand.

## Setup

Move dynamic libraries to a Tan 'well-known' directory:

```sh
./install
```

## Upgrade

#IMPORTANT

Whenever Rust is upgraded to a newer version, all dynamic libraries need to be
reinstalled:

```sh
./install
```

## Status

This is an _experimental_ project, not intended for production use. However, the
project is under active development.

## Contributing

Pull requests, issues, and comments are welcome! Make sure to add tests for new
features and bug fixes.

## License

This work is licensed under the Apache-2.0 License with LLVM Exceptions. See
[LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/LLVM-exception.html>
for details.

## Copyright

Copyright © 2024 [Georgios Moschovitis](https://gmosx.com).
