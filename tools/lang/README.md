# dscp-lang

`dscp-lang` is a CLI tool to help with the writing and documentation of token models and process flows used in `dscp`. The following subcommands are available:

## parse

The `parse` subcommand takes as argument the path to a `dscp` token spec file (see [example](./examples/l3.dscp)) and parses the input. Usage is as follows:

```
Usage: dscp-lang parse [OPTIONS] <FILE_PATH>

Arguments:
  <FILE_PATH>  Path to dscp token specification file

Options:
  -v, --verbose  Output full token and function declaration
  -h, --help     Print help
```
