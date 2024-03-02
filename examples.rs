/* Slices in Rust are a reference to a contiguous 
sequence of elements in a collection. 
They are a view into the original collection,
and do not store any data themselves. 
Slices are used to give a part of a collection to a function 
or to iterate over a part of a collection. */

//Slice syntax: &[T]
//T is the type of the elements in the collection.
//& is a reference to the collection.

fn main() {
    
    //first example: slice of an array of chars
    let s = ['h', 'e', 'l', 'l', 'o'];
    let slice = &s[1..3]; // syntax for slicing &[T]
    println!("{:?}", slice);

    //second example: slice of a vector of integers
    //vectors are resizable arrays
    let vec: Vec<i32> = vec![10, 20, 30, 40, 50]; 
    let slice: &[i32] = &vec[3..4];
    println!("{:?}", slice); // [40]

    //third example: slices of a String
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{:?}", hello);
    println!("{:?}", world);
}

//Ranges shourtcuts for slices

let s = String::from("Francesco");

// initial index
let slice = &s[0..3];
println!("{}", slice);
let slice = &s[..3]; // same as &s[0..3]
println!("{}", slice);

// final index
let len = s.len();
let slice = &s[4..len];
println!("{}", slice);
let slice = &s[4..]; //same as &s[4..len]
println!("{}", slice);

// full string
let slice = &s[0..len];
println!("{}", slice);
let slice = &s[..]; //same as &s[0..len]
println!("{}", slice);

// no slices: get the first word in a string
fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The string is {}", s);
    println!("The first word is {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// slices: get the first word in a string
fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The string is {}", s);
    println!("The first word is {}", word);

    //string literal as input
    let s2 = "second world";
    let word2 = first_word(s2);
    println!("The string is {}", s2);
    println!("The first word is {}", word2);
}

// gets a reference to a string in input
// returns the length of the first word in the string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..] //return the whole string
}
