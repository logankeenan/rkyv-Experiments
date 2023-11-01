# Rkyv Experiments


Experimenting with [rkyv](https://github.com/rkyv/rkyv), a zero-copy deserialization framework for Rust.


This app runs a server with two endpoint (see below). It will log out how long the serialization to bytes too for each endpoint

1. http://localhost:3000/rkyv
2. http://localhost:3000/json