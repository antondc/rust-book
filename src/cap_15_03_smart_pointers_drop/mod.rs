mod custom_smart_pointer;
use custom_smart_pointer::CustomSmartPointer;

pub fn run() {
  println!("\n• cap_15_03_smart_pointers_drop-------");

  // «a» will be manually dropped before its turn thanks to the manuall call to `std::mem::drop`
  let a = CustomSmartPointer {
    data: String::from("Premature stuff"),
  };
  drop(a); // <- Dropping manually: `std::mem::drop`

  // «b» and «c» will be automatically dropped by the implemented Drop trait
  let b = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  let c = CustomSmartPointer {
    data: String::from("other stuff"),
  };
  println!("CustomSmartPointers created.");
}
