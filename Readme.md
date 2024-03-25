# Readme

This git Repository contains Example-Code used in the talk "Let's get Interdisciplinary - Rust Design Patterns for Chemical Plants" on RustNation UK 2024 (2024-03-28). Beside that, the Readme acts as a small handout/summary of the presentation. 

**RECORDING TBA (to be announced)**.

If you enjoyed the talk and want to checkout the mentioned additional materials:

- Video Series - Pragmatic Rust for Engineers
- Blog Post - The Haber-Bosch Process: Revolutionizing Agriculture and Industry

## What are Design Patterns?

“Design patterns are typical solutions to common problems in software design. Each pattern is like a blueprint that you can customize to solve a particular design problem in your code.” [Refactoring Guru](https://refactoring.guru/design-patterns)

The consist at least of a name, problem description and a solution. Often their relationships to other patterns and examples are given.

Historically they were introduced by the so-called gang of four (Eric Gamma, Richard, Helm, Ralph Johnson, John Vlissides) in the book [**"Design Patterns - Elements of Resuable Object-Oriented Software"**](https://amzn.to/43z5SWq).

In my opinion: Design Patterns are useful for communication IF you questioning the common understanding. (Questioning the common understanding get's very important in interdisciplinary work-environments)

## What are Chemical Plants?

Chemical Plants are modeled as cyclic graph structure called **flowsheet**:

- Nodes represent so called unit operations, e.g. a reactor, a distillilation column, etc.
- Edges represent pipes for transporting liquids/gases, e.g. a mixture of Hydrogen, Nitrogen and Ammonia.

A distillation column is the most common seperation operation in the chemical industry.

## Builder Pattern

- Problem: Complex Object with interdepending initialization of many fields
- Solution: Use a separate object responsible for the building

Very common in Rust due to the missing of function overloaded. Leads to a safe&ergonomic API.

[Goto Example Code](./src/01_builder_column.rs)

## Typestate Pattern

- Problem: Object behavior is depending on a finite number of states
- Solution: Extract state-specific behavior for each state into own (internal) type

Recommended for Library code in Rust. Leads to a safe&ergonomic API backed by the compiler.

[Goto Example Code](./src/02_typestate_column.rs)

## Visitor Pattern

This pattern contains some interesting concepts, however it is easily misused because of it's complexity. Refer to the presentation to see the discussion.

- Problem: A (well-)defined object structure, shall support and might be extended with several algorithms
- Solution: Add a 2nd hierarchy that “visits” the objects to implement the algorithm for each object type

## License

All the code is dual-licensed under either:

- MIT License
- Apache License, Version 2.0

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem.