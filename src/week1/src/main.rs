use std::{fs::File, io::{Read, Result}};

fn main() -> Result<()> {
    let mut file = File::open("../../input/week1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", print_increasing(&contents));
    println!("{}", print_increasing_window(&contents));
    
    Ok(())
}

fn print_increasing(contents: &String) -> u32 {
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

    increasing

}

fn print_increasing_window(contents: &String) -> u32 {
    let mut increasing = 0;
    let mut d = contents.split('\n');
    let mut d2 = contents.split('\n').skip(1);
    let mut d3 = contents.split('\n').skip(2);
    let mut last: u32 = d.next().unwrap().parse().unwrap();
    let mut last2: u32 = d2.next().unwrap().parse().unwrap();
    let mut last3: u32 = d3.next().unwrap().parse().unwrap();
    while d3.clone().peekable().peek().is_some() {
        let i: u32 = d.next().unwrap().parse().unwrap();
        let i2: u32 = d2.next().unwrap().parse().unwrap();
        let i3: u32 = d3.next().unwrap().parse().unwrap();

        if i + i2 + i3 > last + last2 + last3 {
            increasing += 1;
        }
        last = i;
        last2 = i2;
        last3 = i3;
    }
    
    increasing
}

#[test]
fn test_increasing() {
    let mut file = File::open("../../input/week1.short.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    assert_eq!(print_increasing(&contents), 2);
}

#[test]
fn test_increasing_window() {
    let mut file = File::open("../../input/week1.shortwindow.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    assert_eq!(print_increasing_window(&contents), 6);
}