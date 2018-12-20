imap-search-test
================

This is just a test program to make search queries on an imap server:

Installation
------------

Install Rust: [https://rustup.rs/](https://rustup.rs/)

Then:
```
cargo build
```

Example Usage
-------------

```
$ ./target/debug/imap-search-test imap.example.org username INBOX 'SUBJECT "hello"'
Password:
Found: {}

// an empty list means nothing is found
```
