use raylib::prelude::*;
//use rlua::prelude::Lua;

#[derive(Copy, Clone)]
struct Resolution {
  x: i32, y: i32,
}

impl std::ops::Mul for Resolution {
    type Output = Resolution;

    fn mul(self, rhs: Resolution) -> Resolution {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        return Resolution{x: x, y: y};
    }
}

fn main() {
  
  let resolution_list = [
    Resolution{x: 50, y: 16},
    Resolution{x: 80, y: 25},
    Resolution{x: 160, y: 50}
  ];
  let mut resolution = Resolution{x: 50, y: 16};
  let resolution_scale = Resolution{x: 10, y: 20};
  let fps_limit = 30;
  
  let stdin = std::io::stdin();
  let mut str = String::from("Choose screen tier:");
  let mut i = 0;
  for resolution in resolution_list.into_iter() {
    str += &format!("\n{} - {}x{}", i, resolution.x, resolution.y);
    i += 1;
  }
  println!("{}", str);
  let mut stop = false;
  let mut input = String::new();
  while !stop {
    stdin.read_line(&mut input).unwrap();
    match input.trim().parse::<u16>() {
      Ok(n) => {
        if usize::from(n) < resolution_list.len() {
          resolution = resolution_list[usize::from(n)];
          stop = true;
        } else { print!("No such option."); }
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
    .size(true_resolution.x, true_resolution.y)
    .title("OCRust")
    .build();
  // Limit framerate
  rl.set_target_fps(fps_limit);
  let font = rl.get_font_default();
  // Runloop
  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);

    let text = format!("{}x{} - virtual", resolution.x, resolution.y);
    let text2 = format!("({}x{}) - true", true_resolution.x, true_resolution.y);
    
    d.draw_rectangle_lines(0,0,true_resolution.x,true_resolution.y, Color::WHITE);
    d.draw_fps(3,2);
    d.draw_text("Initialized.", 3, 22, 18, Color::WHITE);
    d.draw_text(&text, 3, 40, 18, Color::WHITE);
    d.draw_text(&text2, 3, 58, 18, Color::WHITE);
    d.draw_rectangle_lines(3,76, 54,18, Color::WHITE);
    d.draw_text_rec(&font, "XIA", Rectangle{x: 3.0, y: 76.0, width: 54.0, height: 18.0}, 18.0, 5.0, false, Color::WHITE);
  }
}
