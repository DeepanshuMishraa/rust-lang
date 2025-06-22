enum Color{
  Red,
  Green,
  Blue,
  Custom{
    r: u8,
    g: u8,
    b: u8,
  },
  Custom2(u8, u8, u8),
}

// methods => methods are functions that are defined on a type.

impl Color{
  fn rgb(color:Color)->(u8, u8, u8){
    match color{
      Color::Red => (255, 0, 0),
      Color::Green => (0, 255, 0),
      Color::Blue => (0, 0, 255),
      Color::Custom{r, g, b} => (r, g, b),
    }
  }

  fn print_color(color:Color){
    let (r, g, b) = Color::rgb(color);
    println!("Color: r={}, g={}, b={}", r, g, b);
  }

  fn print_color_str(color:Color){
    let color_str = match color{
      Color::Red => "Red",
      Color::Green => "Green",
      Color::Blue => "Blue",
      Color::Custom{r, g, b} => "Custom color",
      Color::Custom2(r, g, b) => "Custom2 color",
    };
    println!("Color: {}", color_str);
  }

  fn rgb2(self)->(u8, u8, u8){
    match self{
      Color::Red => (255, 0, 0),
      Color::Green => (0, 255, 0),
      Color::Blue => (0, 0, 255),
      Color::Custom{r, g, b} => (r, g, b),
    }
  }

  // self is a keyword that refers to the instance of the struct.
  // self is used to access the fields of the struct.
  // self is used to call methods on the struct.
  // self is used to return the struct.
  // self is used to print the struct.
  // self is used to compare the struct.
  // self is used to hash the struct.
  // self is used to clone the struct.
}

fn main(){
  let color = Color::Red;
  color.print_color();
  color.rgb2();
}
