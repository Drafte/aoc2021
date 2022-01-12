use std::{marker::PhantomData, fs::File, io::Read};

use serde::{de::{Error, Visitor}, Deserialize, Deserializer};
use serde_json;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
enum Direction {
    Forward,
    Up,
    Down,
}

struct Move {
    d: Direction,
    v: i32,
}


// impl<'de> Deserialize<'de> for Move {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct MoveVisitor {
//             marker: PhantomData<fn() -> Move>
//         }
        
//         impl MoveVisitor {
//             fn new() -> Self {
//                 MoveVisitor {
//                     marker: PhantomData
//                 }
//             }
//         }
       
//         impl<'de> Visitor<'de> for MoveVisitor {
//             type Value = Move;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("move")
//             }

//             fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//             where
//                     E: Error, {
//                 let mut r = v.split(' ');
//                 let d: Direction = serde_json::from_str(r.next().unwrap()).unwrap();
//                 let v: i32 = match d {
//                     Direction::Forward => r.next().unwrap().parse().unwrap(),
//                     Direction::Up => -1 * r.next().unwrap().parse::<i32>().unwrap(),
//                     Direction::Down => r.next().unwrap().parse().unwrap(),
//                 };
//                 Ok(Move { d, v } )
//             }
//         }
       
//         let item: Move = deserializer.deserialize_str(MoveVisitor::new()).unwrap();
//         Ok(item)
//     }
// }


impl<'de> Deserialize<'de> for Move {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        let mut r = s.split(' ');
        let d: Direction = serde_json::from_str(r.next().unwrap()).unwrap();
        let v: i32 = match d {
            Direction::Forward => r.next().unwrap().parse().unwrap(),
            Direction::Up => -1 * r.next().unwrap().parse::<i32>().unwrap(),
            Direction::Down => r.next().unwrap().parse().unwrap(),
        };
        Ok(Move { d, v } )
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit newtype_struct
        ignored_any unit_struct tuple_struct tuple option identifier
    }
}
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    pub fn relocate(self: &mut Self, m: &Move) {
        match m.d {
            Direction::Forward => self.x += m.v,
            Direction::Up | Direction::Down => self.y += m.v,
        }
    }
}

fn main() {
    let mut file = File::open("../../input/week2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    println!("{:?}", meep(&contents));
}

fn meep(contents: &String) -> i32 {
    let mut l = Location { x: 0, y: 0};
    let mut m = contents.split('\n');

    m.by_ref().for_each(|i| {
        l.relocate(&serde_json::from_str(i).unwrap());
    });
    l.x * l.y
}

#[test]
fn test_meep() {
    let mut file = File::open("../../input/week2.test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    assert_eq!(meep(&contents), 54);

}