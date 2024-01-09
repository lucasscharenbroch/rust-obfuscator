# Labyrinth Macros

`labyrinth_macros` is a procedural macro crate designed to complement the `labyrinth` super crate. It provides powerful compile-time string and control flow obfuscation capabilities, aimed at enhancing the security and complexity of Rust code. Not meant to be used standalone, necessary obfuscation features are in the super crate `labyrinth`

## Features

- **String Obfuscation**: Automatically encrypts string literals in your code at compile time, making them harder to read and understand.
- **Flow Obfuscation**: Introduces dummy loops and random variables into control flows, enhancing the overall obfuscation of the logic.