use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut _answer = 0;
    if args.len() == 2
        {
        let lines = lines_from_file(&args[1]);
        for line in lines {
            _answer += line.parse::<i32>().unwrap();
        }
         println!("{}", _answer);    
    }
}