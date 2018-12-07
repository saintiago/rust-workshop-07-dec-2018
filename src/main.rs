use std::fs::File;
use std::io::prelude::*;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    x: i32,
    y: i32,
}

fn main() -> std::io::Result<()> {
  let data = Data { x: 1, y: 2 };
  let result = serde_json::to_string(&data);

  match result {
    Ok(serialized) => {
      println!("serialized {}", serialized);

      let mut file = File::create("data.json")?;
      file.write_all(serialized.as_bytes())?;

      let mut input = File::open("input.json")?;
      let mut contents = String::new();
      input.read_to_string(&mut contents)?;
      let point: Data = serde_json::from_str(&contents)?;
      println!("point is {:?}", point);
    },
    Err(e) => println!("Error happened {:?}", e),
  }
  Ok(())
}
