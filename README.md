# michelson-ast

## Overview
`michelson-ast` is a Rust library for generating Michelson code. This library can handle the Abstract Syntax Tree (AST) of Michelson, the smart contract language for Tezos.

## Usage
To generate Michelson code using this library, you can write a program like the following:
```rust
use michelson_ast::{
    instruction::Instruction,
    program::Program, ty::Ty,
    wrapped_instruction::WrappedInstruction,
};

fn main() {
    let program = Program {
        storage: Ty::Unit,
        parameter: Ty::Unit,
        code: vec![
            WrappedInstruction {
                comment: Some("=> Unit".to_owned()),
                instruction: Instruction::Cdr,
            },
            WrappedInstruction {
                comment: Some("=> {} : Unit".to_owned()),
                instruction: Instruction::Nil { ty: Ty::Operation },
            },
            WrappedInstruction {
                comment: Some("=> (Pair {} Unit)".to_owned()),
                instruction: Instruction::Pair,
            },
        ],
    };

    println!("{}", program.to_string());
}
```

## Example output
```
parameter unit;
storage unit;
code {
       CDR; # => Unit
       NIL operation; # => {} : Unit
       PAIR; # => (Pair {} Unit)
     }
```
