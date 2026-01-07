//This is a design pattern in Rust that allows you to mutate data even when it disallowed by the borrowing rules in Rust
//To mutate data this pattern uses unsafe code inside a data strucutre to bypass the mutability rules.
//Unsafe code is code that is not checked at compile time for memory safety
//RefCell enforces borrowing rules at runtime. Note this is unsafe. Cos it comes with a performance draw back
//Borrowing rules are normally checked at compile time so  things can be enfored before application get to run.
//This allows mutable and immutable borrows at runtime.
//Interior mutability pattern allows you to mutate data when it passed in an immutable reference
//Unlike RefCell, Box smartpointer enforces borrowing rules at compile time.

fn main() {}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max: f64 = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over the quota!")
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You 've used up over 90%")
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You 've used up over 75% of our quota")
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messenges: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messenges: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messenges.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn test_mock_messages() {
        let mock_messenger: &MockMessenger = MockMessenger::new();

        let mut limit_tracker: LimitTracker<MockMessenger> = LimitTracker::new();

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messenges.borrow().len(), 1);
    }
}
