bgfx-rs [![travis-ci status](https://travis-ci.org/rhoot/bgfx-rs.svg?branch=master)](https://travis-ci.org/rhoot/bgfx-rs) [![appveyor status](https://ci.appveyor.com/api/projects/status/github/rhoot/bgfx-rs?branch=master&svg=true)](https://ci.appveyor.com/project/rhoot/bgfx-rs/branch/master)
=======

Rust wrapper around [bgfx], providing a clean, safe API for rust applications.

*Please read the [crate documentation][docs] for build requirements and crate
limitations before using.*

**Note:** I was originally using this crate as a way of learning Rust. Since I
personally stopped using Rust again however, this crate has ended up more or
less unmaintained. There are a couple of forks that have some more work put
into it already, but if someone wants to take over the project for real,
please let me know so I can direct users to your fork instead.

Documentation
-------------

[API Documentation][docs]

## Prerequisites

On Windows you need to have LLVM installed, as it is needed by bindgen.  
To do that you can Install [Chocolatey](https://chocolatey.org/) and then from command line:
```
choco install llvm
```

## Building

Clone repository and Update submodules
```
git clone https://github.com/jazzay/bgfx-rs.git
cd bgfx-rs
git submodule update --init --recursive
```
### Building for Non-Windows
```
cargo build
```
### Building for Windows
Due to a dependency issue (shaderc) you need to build the sys first so that it copies the shaderc binary properly on Windows. Hopefully we can improve this in the future. After this you can successfully build and run the examples as below.

```
cd bgfx-sys
cargo build
cd ..
cargo build
```

## Examples

To run the examples, invoke them through cargo:

```
cargo run --example 00-helloworld
cargo run --example 01-cubes
```


License
-------
Copyright (c) 2015-2016, Johan Sköld

Permission to use, copy, modify, and/or distribute this software for any  
purpose with or without fee is hereby granted, provided that the above  
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES  
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF  
MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR  
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES  
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN  
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF  
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.


[#468]:   https://github.com/tomaka/glutin/issues/468   "tomaka/glutin #468"
[#520]:   https://github.com/tomaka/glutin/issues/520   "tomaka/glutin #520"
[bgfx]:   https://github.com/bkaradzic/bgfx             "bgfx"
[docs]:   https://rhoot.github.io/bgfx-rs/bgfx/         "Bindings documentation"
[glutin]: https://github.com/tomaka/glutin              "glutin"