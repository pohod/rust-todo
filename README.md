# TODO App in Rust

Note: I have no experience with Rust as of writing this.

## Building

```
$ cargo build
```

## Running

```
$ cargo run
```

## Basic usage

Note: Use Ctrl+D to abort an operation instead of Ctrl+C. Ctrl+D with no operation pending will try to exit.
```
$ cargo run
Enter a command (? for help):
> a
TODO Text: Implement HTTP2 for my web server
Added todo #0: Implement HTTP2 for my web server
> a
TODO Text: Nuke Microsoft HQ
Added todo #1: Nuke Microsoft HQ
> a
TODO Text: Buy milk at the grocery store
Added todo #2: Buy milk at the grocery store
> l
All TODOs:
  (0): Implement HTTP2 for my web server
  (1): Nuke Microsoft HQ
  (2): Buy milk at the grocery store
> c
TODO ID to remove: 0
Removed TODO #0: Implement HTTP2 for my web server
> save
Save file: todos.txt
> q
Quitting.
```

This example creates 3 TODOs, removes the first one and writes them to a file called `todos.txt`
