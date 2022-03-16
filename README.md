# TmpToml

A Rust CLI tool for parsing config toml files to use in templated files.

## Installation

First and foremost you will need to install Rust, recommended via Homebrew:

```sh
brew install rust
```

Then build the tool:

```sh
cargo build
```

OR run the tool directly:

```sh
cargo run config.toml template.yaml qa system1
```

## TmpToml Breakdown

_**Config.toml**_

config.toml is the main configuration file for the tool. It contains the variables and values that are used to render the templates. This file contains PUBLIC values that can be checked into source control. This file contains the following sections and secondary sections:

_**Sections**_

Sections are used to define the environment and anything that is shared amongst the templates.

- qa
- production

_**Secondary Sections**_

Secondary sections are used to define the specifics for each service/system.

- system1
- system2

_**Templates**_

Each template file contains variables and values are defined in the `config.toml` file. Variables are specificed using the `{{variableName}}` syntax. If a variable exists in the templated file but not in the `config.toml`, TmpToml will throw an error and the template will not be rendered. If a variable exists in the `config.toml` file but not in the referenced templated file, TmpToml will still render the template.

_**Example:**_

First render the template:

```sh
./tmptoml config.toml template.yaml qa system1
```

The breakdown of the above command:

- `./tmptoml` is the TmpToml binary.
- `config.toml` is the path to the configuration file.
- `template.yaml` is the path to the template file.
- `qa` is the primary section/environment.
- `system1` is the name of the secondary section.

TmpToml renders the template file to STDOUT.

## License

`TmpToml` is Copyright © 2022 Uptech Works LLC. It is free software, and
may be redistributed under the terms specified in the LICENSE file.

## About <img src="http://uptechstudio.com/img/logo.png" alt="uptech studio" height="48">

`TmpToml` is maintained and funded by [Uptech Studio][uptechstudio], a
software design & development agency & consultancy.

We love open source software. See [our other projects][community] or
[hire us][hire] to design, develop, and grow your product.

[community]: https://github.com/uptech
[hire]: http://upte.ch
[uptechstudio]: http://uptechstudio.com
