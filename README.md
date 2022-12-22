# todocli

A todo list from terminal. Compatible with Linux and Windows, not tested on macOS (not planing on support tho).

~~**Note:** _Currently being rewritten in a different branch due to problematic and spaghetti code_ :)~~

## Features

- Create/Read/Delete/Clear todos;

## Requirements

- Rust 1.65.0 or later;

## Build

To build, follow these steps:

1. Clone the repository using `git clone https://github.com/devkcud/todocli.git`
2. cd into the _cloned folder_ and build it using `cargo build --release`

## Usage

Assuming you've built it and you're on the _cloned folder_, run:

```bash
./target/release/todocli[EXE] [COMMAND]
```

> You can go to the release folder and run copy the executable to anywhere you want.

### Commands

| Command | Description                                | (Semi-)Example                   | Example
|      -: | :-                                         | :-                               | :-
|     add | Add a todo.                                | todocli add [name: _String_]     | todocli **add** "_Here it goes... My todo_"
|  remove | Remove a todo. (see each index on list)    | todocli remove [index: _Number_] | todocli **remove** _5_
|  toggle | Toggle 'done' status.                      | todocli toggle [index: _Number_] | todocli **toggle** _2_
|   reset | Reset todos.                               | todocli clear                    | todo **clear**
|    help | Show help menu.                            | todocli help                     | todocli help

> If you don't pass any command in the CLI, it will show the to-do list with each index and size

## Known issues

-

## To do

- ~~Rewrite source;~~
- Create a temporary todo (available only on current session);
- ~~STOP USING JSON (causing most of the problems);~~
- Create an update todo command;
- Add an insert mode.
