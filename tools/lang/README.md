# sqnc-lang

`sqnc-lang` is a CLI tool to help with the writing and documentation of token models and process flows used in `Sequence` (SQNC). The following subcommands are available:

## parse

The `parse` subcommand takes as argument the path to a `sqnc` token spec file (see [example](./examples/l3.sqnc)) and parses the input. Usage is as follows:

```
Usage: sqnc-lang parse [OPTIONS] <FILE_PATH>

Arguments:
  <FILE_PATH>  Path to sqnc token specification file

Options:
  -v, --verbose  Output full token and function declaration
  -h, --help     Print help
```

## build

The `build` subcommand takes as argument the path to a `sqnc` token spec file (see [example](./examples/l3.sqnc)) and builds a set of process flow restrictions. Usage is as follows:

```
Usage: sqnc-lang build [OPTIONS] <FILE_PATH>

Arguments:
  <FILE_PATH>  Path to sqnc token specification file

Options:
  -o, --output-file <OUTPUT_FILE>  Path of JSON file to output programs to
  -v, --verbose                    Output full token and function declaration
  -h, --help                       Print help
```

The output from this can then be used in conjunction with [sqnc-process-management](https://github.com/digicatapult/sqnc-process-management) to ingest these into a `sqnc` network.
