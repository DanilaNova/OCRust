use raylib::prelude::*;
//use rlua::prelude::Lua;

// Struct for storing and manipulating resolutons
#[derive(Copy, Clone, Debug)]
struct Resolution {
  width: i32, height: i32,
}
// Multiplying two resolutions
impl std::ops::Mul for Resolution {
  type Output = Resolution;

  fn mul(self, rhs: Resolution) -> Resolution {
    return Resolution{
      width: self.width * rhs.width, 
      height: self.height * rhs.height
    };
  }
}
// Displaying resolution in user-friendly format
impl std::fmt::Display for Resolution {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}x{}", self.width, self.height)
  }
}

fn main() {

  let resolution_list = [
    Resolution{width: 50, height: 16},
    Resolution{width: 80, height: 25},
    Resolution{width: 160, height: 50}
  ];
  let mut resolution = Resolution{width: 50, height: 16};
  let resolution_scale = Resolution{width: 8, height: 16};
  let fps_limit = 30;
  
  let stdin = std::io::stdin();
  let mut str = String::from("Avaliable screen tiers:\n");
  let mut i = 0;
  for resolution in resolution_list.into_iter() {
    str += &format!("{} - {}x{}\n", i, resolution.width, resolution.height);
    i += 1;
  }
  
  let mut stop = false;
  while !stop {
    let mut input = String::new();
    println!("{}Choose:", str);
    stdin.read_line(&mut input).unwrap();
    match input.trim().parse::<u16>() {
      Ok(n) => {
        if usize::from(n) < resolution_list.len() {
          resolution = resolution_list[usize::from(n)];
          stop = true;
        } else { println!("No such option."); }
      }
      Err(e) => { println!("Cannot convert to integer(i32): {}.\n\
                            Input: {}", e, input); }
    }
  }
  let true_resolution = resolution * resolution_scale;
  // Initialize lua interpreter
  /*let lua = Lua::new();
  let print = lua.context(|lua_context| {
    lua_context.load(r#"
      print(text)
    "#)
  }).unwrap();*/
  // Initialize raylib
  let (mut rl, thread) = raylib::init()
    .size(true_resolution.width, true_resolution.height)
    .title("OCRust")
    .build();
  // Limit framerate
  rl.set_target_fps(fps_limit);
  // Runloop
  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);

    let text = format!("{} - virtual", resolution);
    let text2 = format!("({}) - true", true_resolution);
    
    d.draw_rectangle_lines(0,0,true_resolution.width,true_resolution.height, Color::WHITE);
    d.draw_fps(3,2);
    d.draw_text("Test initialized.", 3, 22, 18, Color::WHITE);
    d.draw_text(&text, 3, 40, 18, Color::WHITE);
    d.draw_text(&text2, 3, 58, 18, Color::WHITE);
  }
}
