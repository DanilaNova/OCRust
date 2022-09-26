use raylib::prelude::*;
use hex::FromHex;
use std::any::Any;
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

  let code = "F0";
  for v in hex::decode(code).unwrap().iter() {
    let decode = format!("{:#b}", v);
    
    println!("{}", decode[2..])
  }
  let stdin = std::io::stdin();
  let resolution_list = [
    Resolution{x: 50, y: 16},
    Resolution{x: 80, y: 25},
    Resolution{x: 160, y: 50}
  ];
  let mut str = String::from("Choose screen tier:");
  let mut i = 0;
  for resolution in resolution_list.into_iter() {
    str += &format!("\n{} - {}x{}", i, resolution.x, resolution.y);
    i += 1;
  }
  println!("{}", str);
  let mut stop = false;
  let mut input = String::new();
  let mut resolution = Resolution{x: 50, y: 16};
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
  let true_resolution = resolution * Resolution{x: 10, y: 20};
  // Initialize lua interpreter
  /*let lua = Lua::new();
  let print = lua.context(|lua_context| {
    lua_context.load(r#"
      print(text)
    "#)
  }).unwrap();*/
  // Initialize raylib
  let fps_limit = 30;
  let (mut rl, thread) = raylib::init()
    .size(resolution.x * 10, resolution.y * 20)
    .title("Hello, world!")
    .build();
  // Limit framerate
  rl.set_target_fps(fps_limit);
  // Runloop
  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);

    let width = resolution.x.to_string();
    let heigth = resolution.y.to_string();
    
    d.draw_rectangle_lines(0,0,true_resolution.x,true_resolution.y, Color::WHITE);
    d.draw_fps(2,2);
    d.draw_text(&width[..], 2, 22, 20, Color::WHITE);
    d.draw_text(&heigth[..], 2, 42, 20, Color::WHITE);
  }
}
