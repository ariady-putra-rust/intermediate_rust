//! # `RefCell<T>` and the Interior Mutability Pattern
//! _Interior mutability_ is a design pattern in Rust that allows you to mutate data even when there
//! are immutable references to that data; normally, this action is disallowed by the borrowing rules.
//! To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust’s usual rules
//! that govern mutation and borrowing. Unsafe code indicates to the compiler that we’re checking the
//! rules manually instead of relying on the compiler to check them for us.
//!
//! We can use types that use the interior mutability pattern only when we can ensure that the borrowing
//! rules will be followed at runtime, even though the compiler can’t guarantee that. The `unsafe` code
//! involved is then wrapped in a safe API, and the outer type is still immutable.

use file_access::AsFile;
use std::{
    cell::RefCell,
    io::Result,
    rc::{Rc, Weak},
};

/// # Enforcing Borrowing Rules at Runtime with `RefCell<T>`
/// With references and `Box<T>`, the borrowing rules’ invariants are enforced at compile time.
/// With `RefCell<T>`, these invariants are enforced _at runtime_.
/// With  references , if you break these rules, you’ll get a compiler error.
/// With `RefCell<T>`, if you break these rules, your program will panic and exit.
///
/// The advantages of checking the borrowing rules at compile time are that errors will be caught
/// sooner in the development process, and there is no impact on runtime performance because all
/// the analysis is completed beforehand. For those reasons, checking the borrowing rules at compile
/// time is the best choice in the majority of cases, which is why this is Rust’s default.
///
/// The advantage of checking the borrowing rules at runtime instead is that certain memory-safe
/// scenarios are then allowed, where they would’ve been disallowed by the compile-time checks.
/// Static analysis, like the Rust compiler, is inherently conservative. Some properties of code
/// are impossible to detect by analyzing the code: the most famous example is the Halting Problem.
///
/// Because some analysis is impossible, if the Rust compiler can’t be sure the code complies with
/// the ownership rules, it might reject a correct program; in this way, it’s conservative. If Rust
/// accepted an incorrect program, users wouldn’t be able to trust in the guarantees Rust makes.
/// However, if Rust rejects a correct program, the programmer will be inconvenienced, but nothing
/// catastrophic can occur. The `RefCell<T>` type is useful when you’re sure your code follows the
/// borrowing rules but the compiler is unable to understand and guarantee that.
///
/// Similar to `Rc<T>`, `RefCell<T>` is only for use in single-threaded scenarios and will give you a
/// compile-time error if you try using it in a multithreaded context.
///
/// Mutating the value inside an immutable value is the _interior mutability_ pattern.
pub fn ref_cell_t() -> Result<()> {
    Ok({
        // deliberately attempt to write to a directory because we don't really want to write a file
        let logger = FileLogger::new("."); // Is a directory (os error 21)
        let mut tracker = LimitTracker::new(&logger, 100);
        tracker.set_value(95);

        having_multiple_owners_of_mutable_data_by_combining_rc_t_and_ref_cell_t()?;
        reference_cycles_can_leak_memory()?;
        creating_a_tree_data_structure_a_node_with_child_nodes()?;
        visualizing_changes_to_strong_count_and_weak_count()?;
    })
}
struct FileLogger<'a> {
    log_path: &'a str,
}
impl FileLogger<'_> {
    pub fn new(log_path: &str) -> FileLogger {
        FileLogger { log_path }
    }
}
impl Messenger for FileLogger<'_> {
    fn send(&self, msg: &str) {
        if let Err(x) = self.log_path.as_file().append_lines(&vec![msg]) {
            eprintln!("{x}");
            println!("{msg}");
        }
    }
}
/// Consider the following scenario:
/// ```
/// pub trait Messenger {
///     fn send(&mut self, msg: &str);
/// }
/// pub struct LimitTracker<'a, T: Messenger> {
///     messenger: &'a mut T,
///     value: usize,
///     max: usize,
/// }
/// const ERROR_MSG: &str = "Error: You are over your quota!";
/// const URGENT_WARNING: &str = "Urgent warning: You've used up over 90% of your quota!";
/// const WARNING_MSG: &str = "Warning: You've used up over 75% of your quota!";
/// impl<'a, T: Messenger> LimitTracker<'a, T> {
///     pub fn new(messenger: &'a mut T, max: usize) -> LimitTracker<'a, T> {
///         LimitTracker {
///             messenger,
///             value: 0,
///             max,
///         }
///     }
///
///     pub fn set_value(&mut self, value: usize) {
///         self.value = value;
///
///         let percentage_of_max = self.value as f64 / self.max as f64;
///
///         if percentage_of_max >= 1.0 {
///             self.messenger.send(ERROR_MSG);
///         } else if percentage_of_max >= 0.9 {
///             self.messenger.send(URGENT_WARNING);
///         } else if percentage_of_max >= 0.75 {
///             self.messenger.send(WARNING_MSG);
///         }
///     }
/// }
///
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     struct MockMessenger {
///         sent_messages: Vec<String>,
///     }
///
///     impl MockMessenger {
///         fn new() -> MockMessenger {
///             MockMessenger {
///                 sent_messages: vec![],
///             }
///         }
///     }
///
///     impl Messenger for MockMessenger {
///         fn send(&mut self, message: &str) {
///             self.sent_messages.push(String::from(message));
///         }
///     }
///
///     #[test]
///     fn it_sends_an_over_75_percent_warning_message() {
///         let mut mock_messenger = MockMessenger::new();
///         let mut limit_tracker = LimitTracker::new(&mut mock_messenger, 100);
///
///         limit_tracker.set_value(80);
///
///         assert_eq!(mock_messenger.sent_messages.len(), 1);
///         assert_eq!(
///             mock_messenger.sent_messages.get(0),
///             Some(&WARNING_MSG.to_string()),
///             "sent message should be [{}]",
///             WARNING_MSG
///         );
///     }
/// }
/// ```
/// but now struct `LimitTracker.messenger` is declared as `&'a T` instead
/// of `&'a mut T` where T: `Messenger trait` and you have no access to modify
/// the code.
///
/// This is where `RefCell<T>` is useful.
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
const ERROR_MSG: &str = "Error: You are over your quota!";
const URGENT_WARNING: &str = "Urgent warning: You've used up over 90% of your quota!";
const WARNING_MSG: &str = "Warning: You've used up over 75% of your quota!";
impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send(ERROR_MSG);
        } else if percentage_of_max >= 0.9 {
            self.messenger.send(URGENT_WARNING);
        } else if percentage_of_max >= 0.75 {
            self.messenger.send(WARNING_MSG);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        /// mismatched types
        /// expected struct `RefCell<Vec<String>>`
        ///    found struct `Vec<_>`
        /// ```
        /// // --snip--
        ///     sent_messages: vec![],
        /// // --snip--
        /// ```
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    /// # Keeping Track of Borrows at Runtime with `RefCell<T>`
    /// When creating immutable and mutable references, we use the `&` and `&mut` syntax,
    /// respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods,
    /// which are part of the safe API that belongs to `RefCell<T>`.
    impl Messenger for MockMessenger {
        /// use `.borrow_mut()` to mutably borrow the `Vec<String>`, panicking if any borrows exist: `.borrow_mut()`
        /// ```
        /// // --snip--
        ///     self.sent_messages.push(String::from(message));
        /// // --snip--
        /// ```
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
            // The `borrow_mut` returns the smart pointer type `RefMut<T>`.
            // It implements `Deref`, so we can treat them like regular references.
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mut mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mut mock_messenger, 100);

        limit_tracker.set_value(80);

        // use `.borrow()` to borrow the `Vec<String>`, panicking if a mutable borrow exists: `.borrow()`
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        assert_eq!(
            mock_messenger.sent_messages.borrow().get(0),
            Some(&WARNING_MSG.to_string()),
            "sent message should be [{}]",
            WARNING_MSG
        );
        // The `borrow` method returns the smart pointer type `Ref<T>`.
        // It implements `Deref`, so we can treat them like regular references.
    }
    // The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active.
    // Every time we call `borrow`, the `RefCell<T>` increases its count of how many immutable borrows are active.
    // When a `Ref<T>` value goes out of scope, the count of immutable borrows goes down by one.
    // Just like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.

    /// If we try to violate the borrowing rules, rather than getting a compiler error as we would with references,
    /// the implementation of `RefCell<T>` will panic at runtime.
    impl MockMessenger {
        pub fn try_to_violate_the_borrowing_rules(&self) {
            let mut borrow1 = self.sent_messages.borrow_mut();
            let mut borrow2 = self.sent_messages.borrow_mut();

            borrow1.push(ERROR_MSG.to_string());
            borrow2.push(URGENT_WARNING.to_string());
        }
    }

    #[test]
    #[should_panic(expected = "already borrowed: BorrowMutError")]
    fn creating_two_mutable_references_in_the_same_scope_to_see_that_ref_cell_t_will_panic() {
        let messenger = MockMessenger::new();
        messenger.try_to_violate_the_borrowing_rules();
    }
    // Notice that the code panicked with the message `already borrowed: BorrowMutError`.
    // This is how `RefCell<T>` handles violations of the borrowing rules at runtime.
}
// Choosing to catch borrowing errors at runtime rather than compile time, as we’ve done here, means you’d
// potentially be finding mistakes in your code later in the development process: possibly not until your
// code was deployed to production. Also, your code would incur a small runtime performance penalty as a
// result of keeping track of the borrows at runtime rather than compile time. However, using `RefCell<T>`
// makes it possible to write a mock object that can modify itself to keep track of the messages it has
// seen while you’re using it in a context where only immutable values are allowed. You can use `RefCell<T>`
// despite its trade-offs to get more functionality than regular references provide.

/// # Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`
/// A common way to use `RefCell<T>` is in combination with `Rc<T>`. Recall that `Rc<T>`
/// lets you have multiple owners of some data, but it only gives immutable access to that data.
/// If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple
/// owners and that you can mutate!
#[derive(Debug)]
enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}
impl<T> List<T> {
    pub fn for_each(&self, f: impl Fn(&T) -> ()) {
        use List::*;

        if let Cons(t, next) = self {
            f(&t.borrow());
            Self::for_each(next, f);
        }
    }
}
fn having_multiple_owners_of_mutable_data_by_combining_rc_t_and_ref_cell_t() -> Result<()> {
    Ok({
        println!("i32");
        {
            use List::*;

            let value = Rc::new(RefCell::new(5));

            let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

            let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
            let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));

            *value.borrow_mut() += 10;

            a.for_each(|i| println!("a:{}", 0 + *i));
            b.for_each(|i| println!("b:{}", 0 + *i));
            c.for_each(|i| println!("c:{}", 0 + *i));

            println!("a after = {:?}", a);
            println!("b after = {:?}", b);
            println!("c after = {:?}", c);
        }

        println!("String");
        {
            use List::*;

            let value = Rc::new(RefCell::new(String::from("Rust")));

            let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

            let b = Rc::new(Cons(
                Rc::new(RefCell::new(String::from("Hello"))),
                Rc::clone(&a),
            ));
            let c = Rc::new(Cons(
                Rc::new(RefCell::new(String::from("World"))),
                Rc::clone(&a),
            ));

            value.borrow_mut().push_str("acean");

            a.for_each(|s| println!("a:{s}"));
            b.for_each(|s| println!("b:{s}"));
            c.for_each(|s| println!("c:{s}"));

            println!("a after = {:?}", a);
            println!("b after = {:?}", b);
            println!("c after = {:?}", c);
        }
    })
}
// This technique is pretty neat! By using `RefCell<T>`, we have an outwardly immutable `List` value.
// But we can use the methods on `RefCell<T>` that provide access to its interior mutability so we
// can modify our data when we need to. The runtime checks of the borrowing rules protect us from
// data races, and it’s sometimes worth trading a bit of speed for this flexibility in our data
// structures. Note that `RefCell<T>` does not work for multithreaded code! `Mutex<T>` is the thread-
// safe version of `RefCell<T>`

/// # Reference Cycles Can Leak Memory
/// Rust’s memory safety guarantees make it difficult, but not impossible,
/// to accidentally create memory that is never cleaned up (known as a _memory leak_).
/// Preventing memory leaks entirely is not one of Rust’s guarantees,
/// meaning memory leaks are memory safe in Rust.
/// We can see that Rust allows memory leaks by using `Rc<T>` and `RefCell<T>`:
/// it’s possible to create references where items refer to each other in a cycle.
/// This creates memory leaks because the reference count of each item in the cycle will never reach 0,
/// and the values will never be dropped.
fn reference_cycles_can_leak_memory() -> Result<()> {
    Ok({
        use CyclicList::*;

        println!("Reference Cycles Can Leak Memory");

        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    })
}
#[derive(Debug)]
enum CyclicList<T> {
    Cons(T, RefCell<Rc<CyclicList<T>>>),
    Nil,
}
impl<T> CyclicList<T> {
    pub fn tail(&self) -> Option<&RefCell<Rc<CyclicList<T>>>> {
        use CyclicList::*;

        return match self {
            Cons(_, tail) => Some(tail),
            Nil => None,
        };
    }
}

/// # Creating a Tree Data Structure: a Node with Child Nodes
#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    children: RefCell<Vec<Rc<TreeNode<T>>>>,
    //
    // To make the child node aware of its parent,
    // we need to add a `parent` field to our Node struct definition.
    // The trouble is in deciding what the type of parent should be.
    // We know it can’t contain an `Rc<T>`,
    // because that would create a reference cycle with `leaf.parent` pointing to
    // `branch` and `branch.children` pointing to `leaf`,
    // which would cause their `strong_count` values to never be 0.
    parent: RefCell<Weak<TreeNode<T>>>, // # Add a Reference from a Child to Its Parent
                                        // Thinking about the relationships another way,
                                        // a parent node should own its children: if a
                                        // parent node is dropped, its child nodes should
                                        // be dropped as well. However, a child should
                                        // not own its parent: if we drop a child node,
                                        // the parent should still exist. This is a case
                                        // for weak references!
}
impl<T> TreeNode<T> {
    pub fn for_each(&self, f: &impl Fn(&T) -> ()) {
        f(&self.value);
        for child in &*self.children.borrow() {
            Self::for_each(child, f);
        }
    }
}
fn creating_a_tree_data_structure_a_node_with_child_nodes() -> Result<()> {
    Ok({
        println!("Creating a Tree Data Structure: a Node with Child Nodes");

        let leaf = Rc::new(TreeNode {
            value: 3,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(TreeNode {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        branch.for_each(&|i| print!(" {{{}}} ", 0 + i));
        println!();

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    })
}
fn visualizing_changes_to_strong_count_and_weak_count() -> Result<()> {
    Ok({
        println!("Visualizing Changes to strong_count and weak_count");

        let leaf = Rc::new(TreeNode {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(TreeNode {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    })
}
