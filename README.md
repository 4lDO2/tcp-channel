# tcp-channel

[![crates.io](https://img.shields.io/crates/v/tcp-channel.svg)](https://crates.io/crates/tcp-channel)
[![Released API docs](https://docs.rs/tcp-channel/badge.svg)](https://docs.rs/tcp-channel)
[![Test](https://github.com/4lDO2/tcp-channel/actions/workflows/test.yml/badge.svg)](https://github.com/4lDO2/tcp-channel/actions/workflows/test.yml)

SPSC channels in Rust, transmitted through anything that implements `Read` and `Write`.
It uses `bincode` and `serde` for serialization and deserialization.
