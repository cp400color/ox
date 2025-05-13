<p align="center">

<h1 align="center">ox</h1>

<p align="center">
<img src="https://img.shields.io/badge/license-MIT-green" alt="License" title="License" >
<img src="https://img.shields.io/badge/license-Apache-blue" alt="License" title="License" >
</p>

</p>

> Open source build system for biitle.nl projects

Ox is biitle.nl's build system for projects such as FluidServer and Libracino. It makes it easy to clone and start working on one of our projects.

# Usage Example
`ox clone`: Instead of cloning using git, clone using ox - It downloads all the submodule it needs and prepares everything for development.

`ox recommend`: Instead of reading the README and download everything one by one, ox recommends certain extensions and development tools that the developer recommends. If a project uses mypy, ox recommend will recommend it, as long as it is in the ox config.

# Integration
Documentation will be available soon

# Meta
Built by clue <<lost@biitle.nl>>.

Distributed under the MIT license.

# Contributing
You will need to have cargo installed to build and test.
1. Fork the repo (https://github.com/biitlenl/ox)
2. Create your feature branch (`git checkout -b feature/yournewfeature`)
3. Commit your changes (`git commit -am 'Add some change'`)
4. Push to the branch (`git push origin feature/yournewfeature`)
5. Create a new pull request

`SPDX-License-Identifier: MIT or Apache-2.0`