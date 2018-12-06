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

 fn find_duplicate_numbers(input_lines:&Vec<String>) -> (i32,i32)
{
    let mut _count_2_times = 0;
    let mut _count_3_times = 0;
    for line in input_lines {
            let (ln_cnt_2,ln_cnt_3) = find_line_duplicates(line.to_string());
            println!("{:?}", (line,ln_cnt_2,ln_cnt_3));
            if ln_cnt_2 > 0
            {
                _count_2_times += 1;
            }
            if ln_cnt_3 > 0
            {
                _count_3_times += 1;
            }
    }

    return (_count_2_times,_count_3_times);
} 

fn find_line_duplicates(input_string:String) -> (i32,i32)
{
    let mut _already_existing = HashMap::new();
    let mut _count_2_times = 0;
    let mut _count_3_times = 0;
    for ch in input_string.chars()
    {
        if _already_existing.contains_key(&ch){
            if let Some(x) = _already_existing.get_mut(&ch) {
                *x += 1;
            }
        }
        else{
            _already_existing.insert(ch,1);
        }
    }

    for val in _already_existing.values() {
        if val == &2
        {
            _count_2_times += 1;
        }
        if val > &2
        {
            _count_3_times += 1;
        }
    }

    return (_count_2_times,_count_3_times);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut _current_frequency = 0;
    
    let mut _found_frequency = false;
    if args.len() == 2
    {
        let lines = lines_from_file(&args[1]);
        let (_total_2_times,_total_3_times) = find_duplicate_numbers(&lines);
        println!("{:?}", (_total_2_times * _total_3_times,_total_2_times,_total_3_times));
    }
}