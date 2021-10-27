// Trait that defines how Messenger has to be implemented
pub trait Messenger {
  fn send(&self, msg: &str);
}

// Struct LimitTracker defined by a messenger function, a value and a limit
pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

// Implementation of LimitTracker struct with `new` and `set_value` functions
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

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 0.9 {
      self
        .messenger
        .send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
      self
        .messenger
        .send("Warning: You've used up over 75% of your quota!");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  // MockMessenger struct that handles sent messages
  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        // Instead of `vec![]` we use `RefCell::new(vec![])`, which allow use to borrot the red
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  // MockMessenger holds a state with messages sent
  // Complies with Messenger implementation
  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      // Use `borrow_mut` to be able to mutate state at `sent_messages`
      // Otherwise we would write: self.sent_messages.push(String::from(message));
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    // Use `borrow` to be able to mutate `sent_messages`
    // Otherwise we would write: self.sent_messages.push(String::from(message));
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}
