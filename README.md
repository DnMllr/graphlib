# GraphLib

A simple utility for both generating and pathfinding in paths. The tool outputs and consumes text which defines a directed graph.

## Text Format

Each line in the text is a node in the graph. Each line contains a possibly empty list of indices separated
by whitespace. These indices are edges which point to other lines. Indices are zero indexed. The following text:

```
1 2
3
3
5

0
```

Defines the following graph:

```
       ┌───┐
       │   ◄───────┐
       │ 0 │       │
       │   │       │
    ┌──┴───┴─┐     │
    │        │     │
  ┌─▼─┐    ┌─▼─┐   │
  │   │    │   │   │
  │ 1 │    │ 2 │   │
  │   │    │   │   │
  └───┴─┐  └─┬─┘   │
        │    │     │
      ┌─▼─┐  │     │
      │   ◄──┘     │
      │ 3 │        │
      │   │        │
      └─┬─┘        │
        │          │
        │          │
┌───┐   │   ┌───┐  │
│   │   │   │   ├──┘
│ 4 │   └───► 5 │
│   │       │   │
└───┘       └───┘
```

## Commands

### gen

`gen` generates a graph and prints it to stdout. It takes two arguments `--count` which is the number of nodes in the
generated graph and `--probability` which correlates with the odds that any two nodes will be connected.

### pathfind

`pathfind` consumes a graph over stdin and finds the shortest path between the `--start` index and the `--end` index.

## Building

Written in [rust](https://www.rust-lang.org/) and managed with [just](https://github.com/casey/just). Install rust and
cargo with [rustup](https://rustup.rs/) by following the [instructions](https://www.rust-lang.org/learn/get-started).
Similarly `just` can be installed by following the instructions on its github (through cargo `cargo install just`).

Once it's all installed you can build by simply calling `just build`. You can install the tool via cargo with `just install`.
