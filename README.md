# TML Language Server

A Language Server Protocol (LSP) implementation for TML (Tensor Modeling Language). TML is a domain-specific language developed by Typhoon HIL for defining real-time signal processing models using tensor operations, control flow, and hardware I/O. For full language documentation, see the [official TML reference](https://www.typhoon-hil.com/documentation/tml-documentation/).

## Building

```bash
cargo build --release -p tml_lsp
```

The compiled binary is placed at `target/release/tml_lsp` (or `tml_lsp.exe` on Windows). The VS Code extension expects it at that path relative to the workspace root.

## Running Tests

```bash
# Run all tests
cargo test

# Run only tml_tools tests
cargo test -p tml_tools

# Run a specific test file
cargo test -p tml_tools --test collectors_block_span_test
```

## Features

### Diagnostics

The server reports errors on every document change. Three checkers are implemented:

- **Undefined variable** — reports use of variables not declared in the current or global scope. Namespace references (`p.x`, `t.y`, `n.z`, `v.w`) are always considered valid.
- **Function call** — reports calls to undeclared functions or calls with an incorrect number of arguments.
- **Empty body** — reports blocks with no statements inside (`fn`, `if`, `elseif`, `else`, `for`, `while`, `exists`, `not exists`, `feedthrough`, `not feedthrough`).

### Quick Fixes

For every empty body diagnostic, a quick fix is offered that inserts a `pass` statement on the correct indented line.

### Hover

Hovering over a variable shows its inferred or declared type and scope. Hovering over a function call shows the function signature with parameter types and return type.

### Goto Definition

Go to Definition (`F12`) on a variable reference or function call jumps to the declaration site within the same document.

### Document Formatting

Full document formatting (`Shift+Alt+F`) normalizes indentation and whitespace using the built-in TML formatter.

### On-Type Formatting

Pressing Enter inside a block automatically indents the new line to the correct column. The indentation is based on the exact column of the enclosing block's keyword, so it stays correct even if surrounding code is not perfectly formatted.

### Folding

All block constructs are foldable in the editor gutter.

### Completion

`Ctrl+Space` offers keyword snippets. Which snippets appear depends on the cursor context — for example, `fn` is only offered at global scope, and `elseif`/`else` are only offered inside an `if` body.

### Document Highlight

Clicking on a block keyword (`fn`, `if`, `for`, `while`, `else`, `elseif`, `exists`, `feedthrough`, `not exists`, `not feedthrough`, or `end`) highlights both the opening keyword and its matching `end`.
