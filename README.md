# xc

[![Build Status](https://travis-ci.com/nikofil/xc.svg?branch=master)](https://travis-ci.com/nikofil/xc)
[![Build status](https://ci.appveyor.com/api/projects/status/not3oblqs7ere5p4?svg=true)](https://ci.appveyor.com/project/nikofil/xc)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A lightweight calculator for your CLI.

Supports binary, hex and decimal inputs and outputs.

![xc](https://i.imgur.com/BKtJfuS.png)

## Usage

`xc [-dhb] [expression]`

* `-d` `-h` `-b` control the output format (dec, hex and/or bin) - if none are specified, all are outputted in a pretty format
* `expression` is an expression to be calculated, if one isn't given then `xc` opens in interactive mode

### Supported number formats:
```
    100 => 100 (dec)
    0x100 => 256 (hex)
    100h => 256 (hex)
    A0 => 160 (hex)
    0b100 => 4 (bin)
    100b => 4 (bin)
```

### Supported operators:
```
    Add => "+"
    Mul => "*"
    Lparen => "("
    Rparen => ")"
    Sub => "-"
    Div => "/"
    Remainder => "%"
    Pow => "**"
    Neg => "-"
    BNot => "~"
    BXor => "^"
    BOr => "|"
    BAnd => "&"
    LShift => "<<"
    RShift => ">>"
    Assign => "="
```

### Variable assignment:
Variables must begin with a `$` character and their names consist of alphanumeric characters and the `_` character.
```console
$ xc '$x = 1+1; $y = $x * 2; $y << 3;'
> $x = 1+1
> $y = $x * 2
> $y << 3
Dec        32  
Hex        20 h
Bin   10 0000 b
      --4----0 
```

### Higher-order functions:
Functions can be declared with the format `|arg1, arg2, ...| expr`. They can then be called with `$func_name(arg1, arg2, ...)`.
```console
$ xc '$x = |$i| $i*2; $y = |$f, $i| $f($i) + 3; $y($x, 1)'
> $x = |$i| $i*2
> $y = |$f, $i| $f($i) + 3
> $y($x, 1)
Dec     5  
Hex     5 h
Bin   101 b
      ---0 
```

## Features
- [x] interactive mode
- [x] show different formats for output
- [x] multiple expressions in one invocation of `xc`
- [x] store calculation results in variables for reusing in future expressions
- [x] functions
- [ ] read / write values from and to a file given a filename, offset and number of bytes
