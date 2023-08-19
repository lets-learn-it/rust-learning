# Data Types

- by default, all variables are **immutable**.
- use `mut` keyward to make variables **mutable**
  ```rust
  let mut x = 5;
  ```
- constants are values that are bound to name and are not allowed to change. `mut` can't be used.
- **Shadowing**: You can declare new variable with same name as previous variable. It is different from `mut`. you have completely new variable which may have different type than previous one.

---

## Scalar Types

### Integers

- signed integers (stored as 2's complement): `i8`, `i16`, `i32` (default), `i64`, `i128`
- unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`
- architecture depend (indexing collection): `isize` & `usize`
- Integer literals: Decimal (10, 10_000), Hex (0xff), Octal (0o77), Binary (0b1111_0000), Byte (b'A')

### Floats

- `f32` (sigle precision) & `f64` (double precision)
- `f64` is default
- represented according IEEE-754 standard

### Booleans

- `bool` with values `true` & `false`
- One byte in size

### Character

- `char` using single quote
- `let z: char = 'Z'`
- four bytes in size & represent unicode scalar value.

## Compound Types

### Tuple

- way to group together number of values.
- They are fixed length. Once declared, can't change size.
- useful in function return values. tuple without value is called *unit*

### Array

- multiple elements of same type.
- they are of fixed length
- Example
  ```rs
  let arr = [1,2,3,4,5];

  let brr: [i32; 5] =  [1,2,3,4,5];
  ```

## Constants

- starts with `const` keyward.
- datatype should be specified.
- should be upper case. (else will produce warning)
