use std::{fs::File, io::{Read, Result}};

fn main() -> Result<()> {
    let mut file = File::open("../../input/week1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut d = contents.split('\n');
    let mut increasing = 0;
    let mut last: u32 = d.next().unwrap().parse().unwrap();
    d.by_ref().for_each(|d| {
        let i: u32 = d.parse().unwrap();
        if i > last {
            increasing += 1;
        }
        last = i;
    });

    println!("{}", increasing);
    
    Ok(())
}
