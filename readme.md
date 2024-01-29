# MOSS

moss is a simple programming language written in rust. it's a personal project for learning rust and compiler.
just for curiosity.

## guide

### introduction of moss

### install

```bash
cargo install
```

### have a try

if you want a repl to try moss, you can use `cargo run` to run moss.

```bash
cargo run
```

if you want to run a moss file, you can use `cargo run` to run moss.

```bash
cargo run ./test_file/test_while.moss
```

run test,both unit test and integration test.

``` bash
cargo test
```

## todo

### feature

- [x] lexer ( aka scanner )
- [x] var expression
- [x] math compute
- [x] assignment expression
- [] i++ i-- ++i --i += -= *= /=
- [x] if expression
- [x] while expression
- [x] function call expression
- [x] function define expression
- [ ] array expression
- [ ] object expression

### bug

- [ ] blank string parse

### refact

- [ ] use nom refact lexer
