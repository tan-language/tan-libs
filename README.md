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
