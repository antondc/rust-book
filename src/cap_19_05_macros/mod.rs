use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

pub fn run() {
  println!("\n• cap_19_05_macros");

  // · Declarative macros

  // Simplified version of `vec!` macro
  // Will be compiled as
  //
  //    {
  //      let mut temp_vec = Vec::new();
  //      temp_vec.push(1);
  //      temp_vec.push(2);
  //      temp_vec.push(3);
  //      temp_vec
  //    }
  //
  #[macro_export]
  macro_rules! vec_simplified {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
  }

  let vector_1: Vec<u32> = vec_simplified![1, 2, 3];

  println!("vector_1 from vec_simplified: {:?}", vector_1);

  // · Procedural macros

  // Pancake derives HelloMacro from both the trait `hello_macro` and `hello_macro_derive`
  #[derive(HelloMacro)]
  struct Pancakes;

  let vector_2 = vec_simplified![1, 2, 3];

  println!("print vector_2: {:?}", vector_2);

  // We can call `hello_macro`, as Pancakes implements it
  Pancakes::hello_macro();

  // · Attribute-like macros

  // · Function-like macros

  // Similar footprint as function calls
  //
  //    let sql = sql!(SELECT * FROM posts WHERE id=1);
  //
  // Would be implemented as:
  //
  //    #[proc_macro]
  //    pub fn sql(input: TokenStream) -> TokenStream {
  //
}
