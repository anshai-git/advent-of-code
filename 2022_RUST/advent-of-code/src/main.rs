use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::FromStr,
};

fn main() -> io::Result<()> {
    let path = Path::new("./input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}, {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut result: Vec<u32> = vec![0, 0, 0];
    let mut current: u32 = 0;
    for line in reader.lines().filter_map(|x| x.ok()) {
        match &line[..] {
            "" => {
                insert_total(&mut result, current);
                current = 0;
            }
            _ => current += u32::from_str(&line).unwrap_or(0)
        }
    }

    println!("{:?}", sum_result(result));
    Ok(())
}

fn insert_total(vec: &mut Vec<u32>, value: u32) -> () {
    let mut current = value;
    for item in vec.iter_mut() {
        if *item < value {
            let temp = *item;
            *item = current;
            current = temp;
        }
        // println!("{:?}", item);
    }
}

fn sum_result(vec: Vec<u32>) -> u32 {
    return vec.into_iter().sum();
}
