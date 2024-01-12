use nice_colors::Color;
use nice_colors::serializers::hex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Fruit {
    #[serde(with = "hex")]
    color: Color,
}

fn main() {
    let apple = Fruit {
        color: Color::new(255, 0, 0),
    };
    
    let json = serde_json::to_string(&apple).unwrap();
    println!("{}", json);
    
    let apple: Fruit = serde_json::from_str(&json).unwrap();
    println!("{:?}", apple);
}