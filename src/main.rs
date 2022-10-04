use std::{fs::File, io::Read, collections::HashMap, fmt, fmt::{Display, Formatter}};
use raylib::prelude::*;
use hex::FromHex;
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
impl Display for Resolution {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}x{}", self.width, self.height)
  }
}

// Glyph operations
// Errors
#[derive(Debug)]
pub enum GlyphWidthError {
    TooSmall{l: usize},
    NotAMultiple{l: usize},
}
impl std::error::Error for GlyphWidthError {}

impl Display for GlyphWidthError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            GlyphWidthError::TooSmall{l} => {
                write!(f, "Vector is too small to be height of 16: {}", l)
            }
            GlyphWidthError::NotAMultiple{l} => write!(f, "Vector length is not a multiple of 16: {}", l),
        }
    }
}
// Function
fn get_glyph_width(vector: Vec<u8>) -> Result<usize, GlyphWidthError> {
  let length = vector.len();
  if length < 16 {
    return Err(GlyphWidthError::TooSmall{l: length});
  } else if length % 16 != 0 {
    return Err(GlyphWidthError::NotAMultiple{l: length});
  }
  return Ok(length / 16);
}



fn main() {
  println!("Generating glyph hashmap from font.hex...");
  let mut glyph_map: HashMap<u32, Vec<u8>> = HashMap::new();

  match File::open("font.hex") {
    Ok(mut file) => {
      let mut buffer = String::new();
      let _size = file.read_to_string(&mut buffer).unwrap();
      for line in buffer.split('\n') {
        if line.len() <= 2 { continue }
        let split: Vec<&str> = line.split(':').collect();
        if split.len() > 2 {
          println!("Unexpected split on line:\n{}", line);
          return;
        }
        let index;
        let glyph;

        match <[u8; 4]>::from_hex(format!("{:0>8}", split[0])) {
          Ok(bytes) => index = u32::from_be_bytes(bytes),
          Err(err) => {
          println!("Error while converting index:\n{}\n{}", split[0], err);
            return;
          }
        }

        match <Vec<u8>>::from_hex(split[1].trim()) {
          Ok(bytes) => glyph = bytes,
          Err(err) => {
            println!("Error while converting glyph:\n{}\n{}", split[1], err);
            return;
          }
        }
        glyph_map.insert(index, glyph);
      }
    },
    Err(err) => {
      println!("Error while opening a font file:\n{}", err);
      return;
    }
  }
  
  println!("Hashmap generated.\n\
            Hashmap memory size = {}\n\
            Hashmap capacity = {}",
            glyph_map.capacity() * 484 / 10, 
            glyph_map.capacity());

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
    str += &format!("{} - {}\n", i, resolution);
    i += 1;
  }
  
  let mut stop = false;
  while !stop {
    let mut input = String::new();
    println!("{}Choose tier:", str);
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
    d.draw_text("Test initialized.", 3, 22, 16, Color::WHITE);
    d.draw_text(&text, 3, 40, 16, Color::WHITE);
    d.draw_text(&text2, 3, 58, 16, Color::WHITE);
  }
}
