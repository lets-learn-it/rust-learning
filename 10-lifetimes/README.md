# Lifetimes

## Rules

- Each parameter that is a reference, gets its own lifttime parameter
- If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime paramters
- If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.

## `'static` lifetime

- reference lives for duration of program.
