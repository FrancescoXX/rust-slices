## The Slice type in Rust

Rust has a built-in type called `Slice` that is used to reference a contiguous sequence of elements in a collection. Slices are a very important concept in Rust, and they are used extensively in the standard library. In this lesson, we will explore the `Slice` type and how it is used in Rust.

If you prefer a video format, you can watch the lesson on YouTube:

## What is a Slice?

A slice is a reference to a contiguous sequence of elements in a collection. Slices are used to reference a portion of a collection, and they are used extensively in the standard library. Slices are a very important concept in Rust, and they are used extensively in the standard library. In this lesson, we will explore the `Slice` type and how it is used in Rust.

## A first example

Let's see a simple example of a slice. We will use an array:

```rust
fn main() {
    let a = ["a", "b", "c", "d", "e"];
    let slice = &a[1..3];
    println!("{:?}", slice);
}

```


## A second example

Let's write a program that takes a string and returns the first word in the string. Here is the code:

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

Explanation:

- The `as_bytes` method returns a slice of the string's bytes.
- The `iter` method returns an iterator over the slice.
- The `enumerate` method returns an iterator that yields the index and the value of each element in the slice.
- The `b' '` syntax is used to create a byte literal.
- The `s.len()` method returns the length of the string.

Now, there is a problem: we are returning a `usize` but it's a meaningful number only in the context of the original string.

Check this:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

Having to worry about the index in word getting out of sync with the data in s is tedious and error prone! Managing these indices is even more brittle if we write a second_word function. Its signature would have to look like this:

```rust
fn second_word(s: &String, start: usize) -> usize {
    ...
}
```

This is not a good solution.

## String Slices

The solution is to use string slices.

A string slice is a reference to part of a string. Here is an example:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

[IMAGE]

## The `..` Range Syntax

The `..` syntax is used to create a range. Here are some examples:

If you want to start at index 0, you can omit the first number:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

Also, if you want to go to the end of the string, you can omit the second number:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

You can also get the entire string by using the `..` syntax:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

Let's go back to our programming example.

Using slices, we can rewrite the `first_word` function like this:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Now, the `first_word` function returns a string slice, which is a reference to part of the original string. 

If now we have somerthing like this:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

We will get a compile error, because we are trying to use `word` after `s` has been cleared.

## String literals as Slices

Recall that we talked about string literals being stored inside the binary.

Now that we know about slices, we can understand string literals:

```rust
let s = "Hello, world!";
````

The type of `s` is `&str`, which is a slice of a string. This means that `s` is a reference to a contiguous sequence of characters in memory.

## String Slices as Parameters

Let's take a loo at this function:

```rust
fn first_word(s: &String) -> &str {
  ...
}
```

This is correct, but it would be better to use a string slice as the parameter type:

```rust
fn first_word(s: &str) -> &str {
  ...
}
```

In this case, the function can accept both `String` and `&str` types, which makes it more flexible.

## Other Slices

Slices are not limited to strings.

They can be used with any collection type, such as arrays and vectors. Here is an example of a slice of an array:

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```
