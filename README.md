<h1 align="center">ox</h1>

<p align="center">
    <img src="https://img.shields.io/badge/license-MIT-green" alt="License" title="License">
    <img src="https://img.shields.io/badge/license-Apache-blue" alt="License" title="License">
</p>

> Open source build system for biitle.nl projects

Ox is biitle.nl's build system for projects such as FluidServer and Libracino.
It makes it easy to clone and start working on one of our projects.

# Primer

Ox has 3 commands:

- `run`: Run scripts
- `recommend`: Recommend software
- `clone`: Clone git repositories

### The `.ox` directory structure

The `.ox` directory is the directory that contains all your scripts.

A script is a file that can be inside the `.ox` directory or another folder inside.

## Run

Usage: `ox run <SCRIPT> [SUBSCRIPT] [EXTRAS]...`

The `run` command runs a script inside the `.ox` directory.


## Clone

Usage: `ox clone [OPTIONS] <REPOSITORY> [EXTRAS]...`

The `clone` command is just an alias for
`git clone <REPOSITORY> --recurse-submodules -j<threads> [EXTRAS]...`.

# Meta

Built by clue <<lost@biitle.nl>>.

Distributed under the MIT license.

# Contributing

You will need to have cargo installed to build and test.

1. Fork the repo (`https://github.com/biitlenl/ox`)
2. Create your feature branch (`git checkout -b feature/yournewfeature`)
3. Commit your changes (`git commit -am 'Add some change'`)
4. Push to the branch (`git push origin feature/yournewfeature`)
5. Create a new pull request

`SPDX-License-Identifier: MIT or Apache-2.0`
