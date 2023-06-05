use file_access::AsFile;
use std::io::Result;

pub fn ref_cell_t() -> Result<()> {
    Ok({
        // Set-up
        let log_path = "NOT_EXIST";
        log_path.as_file().delete()?;

        let logger = FileLogger::new(log_path);
        let mut tracker = LimitTracker::new(&logger, 100);
        tracker.set_value(95);
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
/// but struct LimitTracker.messenger was declared as &'a T instead of &'a mut T where T: Messenger trait and you have no access to modify the code.
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

    impl Messenger for MockMessenger {
        /// use `.borrow_mut()` to mutably borrow the `Vec<String>`, panicking if any borrows exist: `.borrow_mut()`
        /// ```
        /// // --snip--
        ///     self.sent_messages.push(String::from(message));
        /// // --snip--
        /// ```
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
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
    }
}
