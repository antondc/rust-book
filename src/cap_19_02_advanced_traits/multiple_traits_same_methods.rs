pub fn run() {
  // When there are multiple implementations sharing homonimous methods and taking `&self`
  // we we can clarify it with `Pilot::fly(&[person])`

  trait Pilot {
    fn fly(&self);
  }

  trait Wizard {
    fn fly(&self);
  }

  struct Human;

  impl Pilot for Human {
    fn fly(&self) {
      println!("This is your captain speaking.");
    }
  }

  impl Wizard for Human {
    fn fly(&self) {
      println!("Up!");
    }
  }

  impl Human {
    fn fly(&self) {
      println!("*waving arms furiously*");
    }
  }

  let person = Human {};

  Pilot::fly(&person);
  Wizard::fly(&person);
  person.fly();

  // When there are multiple implementations sharing homonimous methods not taking `&self`
  // we need to clarify which implementation we want to use with `as`
  trait Animal {
    fn baby_name() -> String;
  }

  struct Dog;

  impl Dog {
    fn baby_name() -> String {
      String::from("Spot")
    }
  }

  impl Animal for Dog {
    fn baby_name() -> String {
      String::from("puppy")
    }
  }

  let dog_name = <Dog as Animal>::baby_name();
  println!("A baby dog is called a {}", dog_name);
}
