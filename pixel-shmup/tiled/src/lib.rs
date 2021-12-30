use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct JsonMap {
    width: i32,
    height: i32,
    tilewidth: i32,
    tileheight: i32,
}

pub struct Tiled {}

impl Tiled {
    pub fn load_map(path: &str) -> Result<Tiled, ()> {
        let json_string = std::fs::read_to_string(path)
            .expect(&format!("Something went wrong reading {:}", path));

        // Convert the JSON string back to a Point.
        let tiled: JsonMap = serde_json::from_str(&json_string).unwrap();
        println!("{:?}", tiled);
        Ok(Tiled {})
    }
}
