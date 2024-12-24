# Smart Pointers

>`Interior mutability` pattern where an immutable type exposes an API for mutating an interior value.

## `Box<T>`

- Boxes allow you to store data on heap rather than the stack. Pointer remains on stack which is pointing to data on heap.
- Used for
  - When you have a type whose size cant be known at compile time.
  - When you have large amount of data & you want to transfer ownership but ensure the data wont be copied when you do so.
  - When you want to own a value and you care only that its a type that implements a particular trait rather than being of a specific type.
