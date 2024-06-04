# typst-diff-tool

#### Authors:
- Krzysztof Fijałkowski
- Rafał Szczepaniak

#### Instructor:
- Łukasz Neumann

#### Project Concept:
CLI tool for comparing versions of Typst documents

### What is Typst
A markup-based language for creating elegant documents. Similar to LaTeX but more user-friendly and interactively editable, it can be described in the following points:

- Built-in tags for common formatting tasks
- Flexible functions for everything else
- A tightly integrated scripting system
- Mathematical composition, bibliography management, and more
- Fast compilation times due to incremental compilation
- User-friendly error messages in case of problems

### Problem Description
Unlike LaTeX, there is no tool for comparing generated files, which makes reviewing new versions of documentation challenging.

### Sub-problems and General Solutions
- Parsing the .typ file into an AST and understanding how elements are aggregated
- Converting the AST into JSON format using pandoc or Rust typst_ast
- Comparing ASTs - finding a way to comfortably compare files
- Displaying differences (potentially using libraries for comparing large text segments), parsing these differences back to the appropriate places in the AST
- Creating a resultant .typ file.

### Planned Tools and Technologies
- Extracting AST from a file using typst-syntax
- C4 for illustrating the architecture
- Rust
- Linters and static code analysis - clippy

### Project structure
![c4.drawio.png](docs%2Fc4.drawio.png)

### Usefully links:

[Typst](https://typst.app/) -- online editor

[Typst github](https://github.com/typst/typst) -- Typst repo

[Typst crates](https://crates.io/crates/typst-syntax) -- crates.io types syntax pack

[Typst docs](https://typst.app/docs/reference/visualize/color/) -- docs (use of colors)

[github content struct](https://github.com/typst/typst/blob/main/crates/typst/src/foundations/content.rs#L75) - content structure Typst

[ast in json](https://esdiscuss.org/topic/ast-in-json-format) - AST in JSON

[installing pandoc](https://pandoc.org/installing.htm) - in order to run proj install pandoc v. 3.1.13

[rust-docs](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) - how to create docs in rust

### Running project
0. Pre-requestions:
 - git
 - cargo
 - rust, how to install rust on linux:
``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


1. Clone repository with typst-diff-tool

``` bash
git clone https://github.com/kfijalkowski1/typst-diff-tool
```
2. Enter typst-diff-tool

``` bash
cd typst-diff-tool/typst_ast_parser
```

3. Run typst-diff

``` bash
cargo build --release
./target/release/typst_ast_parser <old_file.typ> <new_file.typ>
```

4. This will create result.typ file as a result of diff tool


### Creating docs

```bash
cargo doc
```

### Running tests
Enter repository directory and find directory with Cargo.toml in current scope (typst_ast_parser/).
If repository has just been cloned then enter the repository and execute following command

``` bash
cd typst_ast_parser && cargo test
```


### Running linter

As a linter we are using clippy, first use [instruction](https://github.com/rust-lang/rust-clippy) to install clippy

In order to run:

``` bash
cargo clippy
```

In order to apply clippy fixes:
```bash
cargo clippy --fix
```


### Running formatter
In order to use formatter simply run rust fmt

```bash
cargo fmt
```

### How our algorithm works?

Read about it in [this](docs/compareDocs.md) file