rust-xdr 
==========

Rust library for decoding XDR data. The XDR format is documented at https://tools.ietf.org/html/rfc4506 

*This is a toy project that I started to teach myself the Rust programming language. Awkward and non-idiomatic newbie Rust code is the theme of the day*


Usage
-----

```rust
extern crate xdr;

let x = xdr::Xdr::new(bytes);

let string = try!(x.unpack_string());
let integer = try!(x.unpack_int());
let myint:XdrResult<f64> = try!(x.unpack_primitive());
...
```

Refer to the source or generated docs to find out about other handy `unpack_*` methods.


Building and Testing
--------------------

Use the provided make file.

- `make` : Run tests, build lib and create docs
- `make test` : Run tests
- `make doc` : Build docs
- `make clean` : Cleanup


