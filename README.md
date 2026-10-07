# logicequation

A small experimental Rust library for building and evaluating symbolic Boolean expressions over a 32-bit input.

Instead of immediately calculating values, `logicequation` builds a graph of logic operations. An `Expression` represents one or more output bits whose values are determined symbolically from the input bits. The resulting graph can then be evaluated for concrete 32-bit inputs.

This was my **first Rust library**. It started as a tool for a concrete problem I was working on at the time and also became a way to learn Rust while exploring symbolic logic and bit-vector arithmetic.

The original use case is no longer current, so the project is **on ice for now**. Before putting it aside, I am giving it some maintenance and polish: fixing known bugs, improving tests and documentation, and making the original ideas easier to understand. It is not abandoned, but active feature development is currently not planned.

## Architecture

The library has a few layers:

- **`BaseOp`** describes a Boolean condition as an `isset` bit pattern plus a `mask`.
- **`AndOp`, `OrOp`, and `UnionOp`** represent and evaluate masked conditions.
- **`OpNode`** combines conditions and references to other nodes using AND or OR plus an optional NOT polarity.
- **`OpTree`** owns the nodes. Node references are indices into this shared graph.
- **`Expression`** is the high-level bit-vector interface. It contains up to 32 root-node references and shares its `OpTree` through `Rc<RefCell<_>>`.

Conceptually:

```text
32-bit input
     |
     v
 BaseOp / masked conditions
     |
     v
   OpNode  <---- references other OpNodes
     |                 |
     +------ OpTree ---+
              |
              v
         Expression
       [root0 ... rootN]
              |
              v
           eval()
```

Bit 0 is the first expression root, so bit vectors are stored least-significant-bit first.

Several expressions can share one tree. This is important: combining expressions from different trees is invalid. Expressions that should interact are therefore normally created from the same context expression using `new_input()` and `new_empty()`.

## Example: symbolic bit-vector arithmetic

`new_input()` creates a symbolic 32-bit view of the input value. Here the low byte and the next byte are interpreted as two separate 8-bit values and added:

```rust
use logicequation::*;

let context = Expression::default();
let mut input = context.new_input();

let sum = input.add(0, &input.clone(), 8, 8).unwrap();

// low byte = 5, next byte = 3
let value = 5 | (3 << 8);

assert_eq!(sum.eval(value).extract(), 8);
```

The call

```text
add(0, ..., 8, 8)
    ^       ^  ^
    |       |  +-- width: 8 bits
    |       +----- right operand starts at bit 8
    +------------- left operand starts at bit 0
```

constructs the adder as symbolic logic. The result can then be evaluated for different concrete input values without rebuilding the expression.

## Example: constructing Boolean logic directly

Lower-level nodes can also be built explicitly. This creates a one-bit XOR of input bits 0 and 1:

```rust
use logicequation::*;

let mut expr = Expression::default();

let xor = expr.create_node(OpType::OR, OpPolarity::KEEP, 0, 2);
{
    let mut node = expr.get_node_mut(xor);

    node.push_op(0b01, 0b11); // bit 0 = 1, bit 1 = 0
    node.push_op(0b10, 0b11); // bit 0 = 0, bit 1 = 1
}

expr.push_node(xor);

assert_eq!(expr.eval(0b00).extract(), 0);
assert_eq!(expr.eval(0b01).extract(), 1);
assert_eq!(expr.eval(0b10).extract(), 1);
assert_eq!(expr.eval(0b11).extract(), 0);
```

## Supported operations

At the `Expression` level the library includes:

- AND, OR, XOR, NAND, NOR, and NOT
- left/right shifts and rotations
- bit-vector addition and subtraction
- full-width multiplication, including a carry-save implementation
- unsigned division with quotient and remainder

Expression vectors are limited to **32 bits**. Full multiplication returns the 64-bit result as two 32-bit expressions.

`UNSET` is a special node used for an absent operand in combinators and for shifted-in positions. When evaluated as a bit, it is false.

## Evaluation

An expression is a symbolic graph until a concrete input is supplied:

```rust
let result = expression.eval(input);
let value = result.extract();
```

Evaluation recursively walks the graph and caches already evaluated nodes for that input, so shared subgraphs are evaluated only once during a single evaluation.

## Current limitations

This is an experimental library rather than a production symbolic-execution engine.

In particular:

- the graph is single-threaded because it uses `Rc<RefCell<_>>`;
- the internal representation is still fairly exposed;
- some invalid ranges or manually constructed node references can panic;
- expression graphs can grow substantially for arithmetic operations;
- there is currently no structural hashing, common-subexpression elimination, or general algebraic simplifier.

Those limitations also reflect the history of the project: it grew while I was learning Rust and solving a specific problem rather than from a finished library design.

## Build and test

```sh
cargo test
```

The test suite covers the primitive logic operations as well as expression evaluation, arithmetic, multiplication, division, tree consistency, and important boundary cases.

## Project status

**On ice / maintenance only.**

The problem this library was originally written to solve is no longer relevant to my current work. The code and the underlying approach are still interesting to me, so I am keeping the repository and documenting it rather than deleting or archiving the ideas.

I may return to it if symbolic Boolean graphs, equivalence checking, SAT/SMT-related experiments, or another suitable use case becomes interesting again.
