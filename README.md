# todocli

A todo list from terminal. Compatible with Linux and Windows, not tested on macOS (not planing on support tho).

**Note:** _Currently being rewritten in a different branch due to problematic and spaghetti code_ :)

## Features

- Create/Read/Delete todos;
- Completely reset todos.

## Requirements

- Rust 1.65.0 or later;

## Build

To build, follow these steps:

1. Clone the repository using `git clone https://github.com/devkcud/todocli.git`
2. cd into the _cloned folder_ and build it using `cargo build --release --quiet`

## Usage

Assuming you've built it and you're on the _cloned folder_, run:

```bash
./target/release/todocli <...arguments>
```

### Commands

| Command | Aliases | Description                                | (Semi-)Example                   | Example                                   | Deprecated
|      -: | :-      | :-                                         | :-                               | :-                                        |    :-:
|     add | a       | Add a todo.                                | todocli add [name: _String_]     | todocli **add** _Here it goes... My todo_ |    ❌
|  remove | r       | Remove a todo. (see each index on list)    | todocli remove [index: _Number_] | todocli **remove** _5_                    |    ❌
|   reset | R       | Reset todos.                               | todocli reset                    | todo **reset**                            |    ❌
|    list | l       | Show todo list.                            | todocli                          | todocli                                   |    ✅
|    help | h       | Show help menu.                            | todocli help                     | todocli help                              |    ❌

## Known issues

1. When trying to remove a negative number;
2. When trying to remove an string instead of a number.

## To do

- Rewrite source;
- Create a temporary todo (available only on current session);
- STOP USING JSON (causing most of the problems);
- Create an update todo command;
- Add an insert mode.
