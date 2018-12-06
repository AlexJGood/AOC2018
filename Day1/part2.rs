use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashMap
};

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn calculation_frequency(mut current_frequency:i32, already_existing:&mut HashMap<i32,String>,input_lines:&Vec<String>) -> (bool,i32)
{
    for line in input_lines {
            current_frequency += line.parse::<i32>().unwrap();
            if !already_existing.contains_key(&current_frequency)
            {
                already_existing.insert(current_frequency,"".to_string());
            }
            else
            {
                println!("{} This is the dup", current_frequency);
                return (true,current_frequency)  
            }
        }

    return (false,current_frequency);
}



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut _current_frequency = 0;
    let mut _already_existing = HashMap::new();
    let mut _found_frequency = false;
    if args.len() == 2
        {
        let lines = lines_from_file(&args[1]);
         while !_found_frequency
         {
             let (found_frequency,current_frequency) = calculation_frequency(_current_frequency,&mut _already_existing,&lines);
             _found_frequency = found_frequency;
             _current_frequency = current_frequency;
         }
    }
}