mod button;
mod draw;
mod screen;
mod select_box;
use button::Button;
use screen::Screen;
use select_box::SelectBox;

pub fn run() {
  println!("\n• cap_17_02_traits_and_types");

  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
      }),
    ],
  };

  screen.run();
}
