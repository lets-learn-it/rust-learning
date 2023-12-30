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

- At any given time, you can have **either one mutable reference or any number of immutable references**
- References must always be valid.

## Mutable Reference

- A mutable reference is a way to borrow data from an owner with the intent to modify it. Multiple mutable references to the same data are not allowed at the same time.
- The owner retains ownership of the data, but during the borrowing period, the owner cannot access or modify the data.
- Mutable references are subject to Rust's strict borrowing rules, which ensure data safety by preventing data races.
