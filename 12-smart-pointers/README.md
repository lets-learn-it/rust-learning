# Smart Pointers

**References `&` smart pointers**

- most common kind of pointer in Rust
- Indicated by `&` symbol and borrow the value they point to
- Don't have any special capabilities other than referring to data & have no overhead
- While references only borrow data, smart pointers own the data they point to in many cases
- Simple examples, `String` & `Vec<T>`
- Smart pointers are usually implemented using struct with implementation of `Deref` and `Drop` traits

## `Box<T>`

- Boxes allow you to **store data on heap** rather than the stack. Pointer remains on stack which is pointing to data on heap.
- Used for
  - When you have a type whose size cant be known at compile time.
  - When you have large amount of data & you want to transfer ownership but ensure the data wont be copied when you do so.
  - When you want to own a value and you care only that its a type that implements a particular trait rather than being of a specific type.

## `Rc<T>`

- Reference counting type that enables multiple ownership


## `RefCell<T>`

>`Interior mutability` pattern where an immutable type exposes an API for mutating an interior value.
