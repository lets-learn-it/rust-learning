# Ownership & Borrowing

## Ownership helps with

- keeping track of what part of code are using what data on heap
- minimizing amount of duplicate data on heap
- cleaning up unused data on heap

## Ownership Rules

- Each value in Rust has owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references
- References must always be valid.