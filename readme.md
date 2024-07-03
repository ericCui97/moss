# MOSS

moss is a simple programming language written in rust and c. it's a personal project for learning rust and compiler.
just for curiosity.

## guide

### introduction of moss

moss version of fibonacci implementation.

``` js
fun fib(n) {
  if (n < 2) return n;
  return fib(n - 1) + fib(n - 2); 
}

var before = clock();
print fib(40);
var after = clock();
print after - before;

```

as we can see,it implement the minimal feature of a function based programming language. since now, `print` is not a function, it's a keyword, and `clock` is a function, it's a built-in function.

we can declare a function by `fun` keyword, and call a function by `function_name(arguments)`.
we can declare a variable by `var` keyword, and assign a value to a variable by `variable_name = value`.
moss support `if`,`while`,`for` statement, and `+ - * /` operator.
the primitive type:

- number: which is f64 in rust
- string: just a string
- boolean: true or false


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

## cmoss

cmoss is moss lang backend,written in c.
[document](./cmoss/README.md)

### plan

[ ] implement a backend of moss,based on byteCode,which is more efficient than ast.
[ ] implement a standard library of moss,which is more useful than just a print and clock.
[ ] build a wasm version for moss, and build a web playground for it;

## todo

### feature

- [x] lexer ( aka scanner )
- [x] var expression
- [x] math compute
- [x] assignment expression
- [ ] i++ i-- ++i --i += -= *= /=
- [x] if expression
- [x] while expression
- [x] function call expression
- [x] function define expression
- [ ] array expression
- [ ] object expression
- [x] scoping and scop resolve
- [x] class statement

### bug

- [x] blank string parse

### refact

- [ ] use nom refact lexer

## reference

###  basic data type
- number: f64
- string: String
- boolean: bool
- nil
