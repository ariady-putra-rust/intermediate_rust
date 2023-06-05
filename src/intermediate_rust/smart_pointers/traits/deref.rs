//! #Treating Smart Pointers Like Regular References with the `Deref` Trait
//! Implementing the `Deref` trait allows you to customize the behavior of the dereference operator `*`
//! (not to be confused with the multiplication or glob operator). By implementing Deref in such a way
//! that a smart pointer can be treated like a regular reference, you can write code that operates on
//! references and use that code with smart pointers too.

use std::{io::Result, ops::Deref};

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    /// associated type in `impl` without body
    /// ```
    /// // --snip--
    ///     type Target;
    /// // --snip--
    /// ```
    /// provide a definition for the type: ` = <type>;`
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn deref_trait() -> Result<()> {
    Ok({
        let x = String::from("Rust");
        let y = MyBox::new(x.clone());

        assert_eq!("Rust", x);

        //                         if without the Deref implementation for MyBox:
        assert_eq!("Rust", *y); // type `MyBox<{integer}>` cannot be dereferenced

        // behind the scenes:
        // *(y.deref())

        deref_coercion(&y); //        with implicit Deref coercion
        deref_coercion(&(*y)[..]); // without Deref coercion implemented by Rust
    })
}

/// # Implicit `Deref Coercions` with Functions and Methods
/// _Deref coercion_ converts a reference to a type that implements the `Deref` trait into a
/// reference to another type. For example, deref coercion can convert `&String` to `&str` because
/// String implements the `Deref` trait such that it returns `&str`. Deref coercion is a convenience
/// Rust performs on arguments to functions and methods, and works only on types that implement
/// the `Deref` trait. It happens automatically when we pass a reference to a particular type’s
/// value as an argument to a function or method that doesn’t match the parameter type in the
/// function or method definition. A sequence of calls to the `deref` method converts the type
/// we provided into the type the parameter needs.
///
/// Deref coercion was added to Rust so that programmers writing function and method calls don’t
/// need to add as many explicit references and dereferences with `&` and `*`. The deref coercion
/// feature also lets us write more code that can work for either references or smart pointers.
fn deref_coercion(name: &str) {
    println!("Hello, {name}!")
}
// When the `Deref` trait is defined for the types involved, Rust will analyze the types and
// use `Deref::deref` as many times as necessary to get a reference to match the parameter’s type.
// The number of times that `Deref::deref` needs to be inserted is resolved at compile time,
// so there is no runtime penalty for taking advantage of deref coercion!

// # How Deref Coercion Interacts with Mutability
// Similar to how you use the `Deref` trait to override the `*` operator on immutable references,
// you can use the `DerefMut` trait to override the `*` operator on mutable references.
//
// Rust does deref coercion when it finds types and trait implementations in three cases:
//
// 1. From `&T`     to `&U`     when `T: Deref<Target=U>`
// 2. From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
// 3. From `&mut T` to `&U`     when `T: Deref<Target=U>`
//
// The first two cases are the same as each other except that the second implements mutability.
// The first case states that if you have a `&T`, and `T` implements `Deref` to some type `U`,
// you can get a `&U` transparently. The second case states that the same deref coercion happens
// for mutable references.
//
// The third case is trickier: Rust will also coerce a mutable reference to an immutable one.
// But the reverse is not possible: immutable references will never coerce to mutable references.
// Because of the borrowing rules, if you have a mutable reference, that mutable reference must
// be the only reference to that data (otherwise, the program wouldn’t compile). Converting one
// mutable reference to one immutable reference will never break the borrowing rules. Converting
// an immutable reference to a mutable reference would require that the initial immutable reference
// is the only immutable reference to that data, but the borrowing rules don’t guarantee that.
// Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable
// reference is possible.
