# glimpse - a rust implementation of a simple lisp

## getting started
```shell
$ cargo build --release
$ ./target/release/glimpse repl
glimpse => 
(+ 1 2)
result: Integer(3.0)
```

## syntax
```
variable definition: (def x 1)
function definition: (def sqr (lambda (x) (* x x)))
calling functions: (sqr 2)
if statement: (if (> big small) big small) 
```
