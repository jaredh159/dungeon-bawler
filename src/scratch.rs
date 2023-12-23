// "trait" - something a type can "DO"
//     ..in rust terms, "functions you can call"

use std::fmt::format;

pub struct Person {
  pub name: String,
  pub age: u8,
}

impl std::fmt::Debug for Person {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "struct::Person({}, {})", self.name, self.age)
  }
}

impl std::fmt::Display for Person {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

pub struct Dog {
  pub name: String,
  pub hypoallergenic: bool,
  pub num_paws: u8,
}

// if you are a "Greeter"...
trait Greeter {
  // ...you have to have a function called "greeting"
  // that returns a String
  fn greeting(&self) -> String;
}

// we need to "implement" (write the code for) "Greeter" for `Person`
impl Greeter for Person {
  fn greeting(&self) -> String {
    format!("Hi!, my name is {}", self.name)
  }
}

impl Greeter for Dog {
  // self refers to the "instance"
  fn greeting(&self) -> String {
    "THUMP THUMP THUMP THUMP".to_string()
  }
}
