# xc

[![Build Status](https://travis-ci.com/nikofil/xc.svg?branch=master)](https://travis-ci.com/nikofil/xc)
[![Build status](https://ci.appveyor.com/api/projects/status/not3oblqs7ere5p4?svg=true)](https://ci.appveyor.com/project/nikofil/xc)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A lightweight calculator for your CLI.

Supports binary, hex and decimal inputs and outputs.

![xc](https://i.imgur.com/BKtJfuS.png)

## Usage

`xc [-dhb] expression`

* `-d` `-h` `-b` control the output format (dec, hex and/or bin) - if none are specified, all are outputted in a pretty format
* `expression` is a the expression to be calculated

## Planned features for the future

* multiple expressions in one invocation of `xc`
* store calculation results in variables for reusing in future expressions
* read / write values from and to a file given a filename, offset and number of bytes
