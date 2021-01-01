# sully_input: Rich input for parsers

This is a helper library for the `sully_peg` parser generator proc macro.

It exposes the following functionality:
- Structs `Input` and `Span` parameterized on the lifetime of a string slice.
- Terse debugging info for each structure (useful for embedding in tree nodes).
- Trait `Exact` to allow type-defined parsing for structures with exact string representations (such as single characters or `str`s).
