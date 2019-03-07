# tcp-channel

[![Crates.io](http://meritbadge.herokuapp.com/tcp-channel)](https://crates.io/crates/tcp-channel)
[![Docs.rs](https://docs.rs/tcp-channel/badge.svg)](https://docs.rs/tcp-channel)
[![Build Status](https://travis-ci.org/4lDO2/tcp-channel.svg?branch=master)](https://travis-ci.org/4lDO2/tcp-channel)

SPSC channels in Rust, transmitted through anything that implements `Read` and `Write`.
It uses `bincode` and `serde` for serialization and deserialization.
