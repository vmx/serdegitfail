serdegitfail
============

This is a minimal example for reproducing Serde Bug [#1645].

Just run a `cargo build`. The error is:

```console
$ cargo build
   Compiling serdegitfail v0.1.0 (/home/vmx/src/rust/misc/serdegitfail)
error[E0277]: the trait bound `MyStruct: serde::ser::Serialize` is not satisfied
    --> src/main.rs:10:12
     |
10   |     to_vec(&MyStruct { data: true }).unwrap();
     |            ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `serde::ser::Serialize` is not implemented for `MyStruct`
     | 
    ::: /home/vmx/.cargo/registry/src/github.com-1ecc6299db9ec823/serde_json-1.0.41/src/ser.rs:2192:8
     |
2192 |     T: Serialize,
     |        --------- required by this bound in `serde_json::ser::to_vec`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `serdegitfail`.

To learn more, run the command again with --verbose.
```

It works when serde is pulled from crates.io and not directly from Git. Simply edit the `Cargo.toml` or go back one commit (`git checkout HEAD~1`).

```console
$ cargo build
    Updating crates.io index
   Compiling proc-macro2 v1.0.5
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.5
   Compiling ryu v1.0.0
   Compiling serde v1.0.101
   Compiling itoa v0.4.4
   Compiling quote v1.0.2
   Compiling serde_derive v1.0.101
   Compiling serde_json v1.0.41
   Compiling serdegitfail v0.1.0 (/home/vmx/src/rust/misc/serdegitfail)
    Finished dev [unoptimized + debuginfo] target(s) in 19.72s
```

[#1645]: https://github.com/serde-rs/serde/issues/1645


License
-------

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
