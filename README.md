<div align="center">
  <h1><code>rustix-libc-wrappers</code></h1>

  <p>
    <strong>Safe Rust bindings to `libc` functions</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/rustix-libc-wrappers/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/rustix-libc-wrappers/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/rustix-libc-wrappers"><img src="https://img.shields.io/crates/v/rustix-libc-wrappers.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/rustix-libc-wrappers"><img src="https://docs.rs/rustix-libc-wrappers/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

rustix-libc-wrappers is a wrapper around functions in [`libc`] that can only be
implemented in libc.

## Minimum Supported Rust Version (MSRV)

This crate currently works on the version of [Rust on Debian stable], which is
currently Rust 1.63. This policy may change in the future, in minor version
releases, so users using a fixed version of Rust should pin to a specific
version of this crate.

[Rust on Debian stable]: https://packages.debian.org/stable/rust/rustc
[`libc`]: https://docs.rs/libc
