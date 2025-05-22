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

## Compilation process

The compilations process can be thought of in the following steps:

1. [Tokenisation](#tokenisation)
2. [Construction of an abstract-syntax-tree (AST)](#ast-construction)
3. [Compilation](#compilation) of the AST to a set of process-flows

### Tokenisation

Tokenisation is performed using [pest.rs](https://pest.rs/) which uses a [parsing expression grammar](https://en.wikipedia.org/wiki/Parsing_expression_grammar). This type of grammar has the advantage that the result of tokenisation is unambiguous; the grammar either parses with a single possible output for a given input or it fails. The grammar for `Sequence` can be found at [./src/sqnc.pest](./src/sqnc.pest). Development of further language features should always start by updating the grammar appropriately. It is strongly recommended that you read the `pest.rs` documentation and [book](https://pest.rs/book/intro.html) which provide excellent reference points. It is also recommended to install the [Pest IDE tools](https://marketplace.visualstudio.com/items?itemName=pest.pest-ide-tools) if using vscode.

The tokenisation process results in parsed expression tree of the matched rules based on a language input. You can think of this as a structured representation of the rule matches for a given input.

### AST construction

The parsed output simply represents the grammatical interpretation of the raw string input against the Sequence grammar. The next stage of compilation involves iterating through the matched rules and translating that to a more meaningful [abstract-syntax-tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree). The code for this can be found at [./arc/ast](./src/ast). Essentially this constructs structs representing token declarations and function declarations defined within a program input. Note the use of the `AstNode` struct to maintain the linkage between a given definition section and the span in the original input it corresponds to. This allows us to produce good errors in the output when compilation fails.

### Compilation

The [compilations](./src/compiler) step ensures that the input is logically consistent and then produces the output of the compiler. These checks include:

- assuring that token, field and function names are unique within their context
- ensuring that types are valid including that referenced token types exist
- ensuring that fields on tokens are compared correctly
- ensuring that tokens passed to nested functions are of the correct type

Once these checks are complete any nested functions are flattened. Finally the functions are converted to restrictions and documented in the output as a set of arguments.
