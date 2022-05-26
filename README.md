# rudg

> **Rust UML Diagram Generator**

Tools that parsing Rust code into UML diagram (in dot format currently).

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Example
Please notice on version v0.1.1 the file name is "aggregation..dot", which will be fixed in the next release
```
$ rudg.exe tests\examples\aggregation.rs
$ cat tests\examples\aggregation.dot
digraph ast {
    "Amut"[label="{Amut|b: *mut B}"][shape="record"];
    "Aconst"[label="{Aconst|b: *const B}"][shape="record"];
    "B"[label="B"][shape="record"];
    "Amut" -> "B"[label=""][arrowtail="odiamond"];
    "Aconst" -> "B"[label=""][arrowtail="odiamond"];
}
```
And if you use [Graphviz Online](https://dreampuf.github.io/GraphvizOnline/) or other tools like [Graphviz add-on for VSCode](https://marketplace.visualstudio.com/items?itemName=joaompinto.vscode-graphviz), then the dot file could be rendered as below:

![aggregation_example](https://raw.githubusercontent.com/SelamaAshalanore/rudg/add_example_in_readme/aggregation_example.svg)

Currently, this tool could only parse single .rs file, and support for parsing whole crate will be released on v0.2.0, which is under development.

## Usage
```
$ rudg.exe --help
rudg 0.1.0

USAGE:
    rudg.exe [file] [OPTIONS]

ARGS:
    <file>    Rust source code file path

OPTIONS:
    -h, --help            Print help information
    -o, --output <DIR>    Sets a custom output directory
    -V, --version         Print version information
```

## Roadmap (TODO list)
- comprehensive tests and bug fix
- support for modelling the whole crate's source code

## Contributing
- All sorts of contributing are welcome. Please feel free to open issue and/or PR.
- We belive that TDD(Test-Driven Development) approach is helpful not only in development, but also in communication with each other. So adding more tests might be a good way to report a bug or even suggest a new feature.

## License
rudg is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

## Related Project
[dot_graph](https://github.com/SelamaAshalanore/dot_graph): A library for generating Graphviz DOT language files.