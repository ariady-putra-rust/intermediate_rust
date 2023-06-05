//! # Using `Box<T>` to Point to Data on the Heap
//! The most straightforward smart pointer is a box, whose type is
//! written `Box<T>`. Boxes allow you to store data on the heap rather
//! than the stack. What remains on the stack is the pointer to the
//! heap data.
//!
//! Boxes don’t have performance overhead, other than storing their data
//! on the heap instead of on the stack. But they don’t have many extra
//! capabilities either. You’ll use them most often in these situations:
//!
//! 1. When you have a type whose size can’t be known at compile time
//!    and you want to use a value of that type in a context that requires
//!    an exact size
//! 2. When you have a large amount of data and you want to transfer ownership
//!    but ensure the data won’t be copied when you do so
//! 3. When you want to own a value and you care only that it’s a type that
//!    implements a particular trait rather than being of a specific type

use std::io::Result;

pub fn box_t() -> Result<()> {
    Ok({
        storing_an_i32_value_on_the_heap_using_a_box()?;
        enabling_recursive_types_with_boxes()?;
    })
}

fn storing_an_i32_value_on_the_heap_using_a_box() -> Result<()> {
    Ok({
        let b = Box::new(5);
        println!("b = {}", b);
    })
}

/// # Enabling Recursive Types with `Boxes`
/// A value of recursive type can have another value of the same type as part of itself.
/// Recursive types pose an issue because at compile time Rust needs to know how much
/// space a type takes up. However, the nesting of values of recursive types could
/// theoretically continue infinitely, so Rust can’t know how much space the value needs.
/// Because boxes have a known size, we can enable recursive types by inserting a box in
/// the recursive type definition.
///
/// As an example of a recursive type, let’s explore the _cons list_. This is a data
/// type commonly found in functional programming languages. The cons list type we’ll
/// define is straightforward except for the recursion; therefore, the concepts in the
/// example we’ll work with will be useful any time you get into more complex situations
/// involving recursive types.
///
/// # Without `Box<T>`
/// recursive type `box_t::List` has infinite size
/// ```
/// enum List<T> {
///     Cons(T, List<T>),
///     Nil,
/// }
/// ```
/// insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle: `Box<`, `>`
///
/// The error shows this type “has infinite size.” The reason is that we’ve defined `List`
/// with a variant that is recursive: it holds another value of itself directly. As a result,
/// Rust can’t figure out how much space it needs to store a `List` value.
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>), // The `Cons` variant needs the size of a `T` plus the space to store the box’s pointer data.
    Nil, //                   The `Nil` variant stores no values, so it needs less space than the `Cons` variant.
}
// We now know that any `List` value will take up the size of a `T` plus the size of a box’s pointer data.
// By using a box, we’ve broken the infinite, recursive chain,
// so the compiler can figure out the size it needs to store a `List` value.
impl<T> List<T> {
    pub fn for_each(&self, f: impl Fn(&T) -> ()) {
        use List::*;

        if let Cons(t, next) = self {
            f(t);
            Self::for_each(next, f);
        }
    }
}
fn enabling_recursive_types_with_boxes() -> Result<()> {
    Ok({
        println!("i32");
        {
            use List::*;

            let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
            list.for_each(|i| println!("{}", 0 + *i));
            dbg!(&list);

            println!("do this twice to make sure nothing was moved");
            list.for_each(|i| println!("{}", 0 + *i));
            dbg!(&list);
        }

        println!("String");
        {
            use List::*;

            let list = Box::new(Cons(
                String::from("one"),
                Box::new(Cons(
                    String::from("two"),
                    Box::new(Cons(String::from("three"), Box::new(Nil))),
                )),
            ));
            list.for_each(|s| println!("{s}"));
            dbg!(&list);

            println!("do this twice to make sure nothing was moved");
            list.for_each(|i| println!("{i}"));
            dbg!(&list);
        }
    })
    // Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities.
    // They also don’t have the performance overhead that these special capabilities incur, so they can be useful
    // in cases like the cons list where the indirection is the only feature we need.
}
// The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>`
// values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the
// box is pointing to is cleaned up as well because of the `Drop` trait implementation.
