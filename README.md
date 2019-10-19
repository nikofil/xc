# xc

[![Build Status](https://travis-ci.com/nikofil/xc.svg?branch=master)](https://travis-ci.com/nikofil/xc)
[![Build status](https://ci.appveyor.com/api/projects/status/not3oblqs7ere5p4?svg=true)](https://ci.appveyor.com/project/nikofil/xc)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A lightweight calculator for your CLI.

Supports binary, hex and decimal inputs and outputs.

![xc](https://i.imgur.com/BKtJfuS.png)

## Usage

`xc [-i] [-dhb] expression`

* `-i` go into interactive mode instead of parsing a single expression
* `-d` `-h` `-b` control the output format (dec, hex and/or bin) - if none are specified, all are outputted in a pretty format
* `expression` is a the expression to be calculated

Supported number formats:
```
    100 => 100 (dec)
    0x100 => 256 (hex)
    100h => 256 (hex)
    A0 => 160 (hex)
    0b100 => 4 (bin)
    100b => 4 (bin)
```

Supported operators:
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
```

## Planned features for the future

* multiple expressions in one invocation of `xc`
* store calculation results in variables for reusing in future expressions
* read / write values from and to a file given a filename, offset and number of bytes
