use std::{fs, collections::HashMap};
use regex::Regex;

fn day_1_1(input: &str) -> i32{
    let mut data = 0;
    for l in input.lines(){
        let mut digits = String::new();
        if let Some(first_digit) = l.chars().find(|c| c.is_numeric()){
            digits.push(first_digit);
        }
        else{
            "Problem parsing line";
            continue;
        }
        if let Some(last_digit) = l.chars().rfind(|c| c.is_numeric()){
            digits.push(last_digit);
        }
        else{
            "Problem parsing line";
            continue;
        }
        match digits.parse::<i32>(){
            Ok(parsed_num) => {
                data += parsed_num;
            }
            Err(e) => {
                println!("Something weird happened with {}, idk", e);
            }
        }
    }
    data
}

fn day_1_2(input: &str) -> i32{
    let mut result = 0;
    let numbers = [
        "1","2","3","4","5","6","7","8","9","0",
        "one","two","three","four","five","six","seven","eight","nine"
    ];
    let num_conv = HashMap::from([
        ("one","1"),
        ("two","2"),
        ("three","3"),
        ("four","4"),
        ("five","5"),
        ("six","6"),
        ("seven","7"),
        ("eight","8"),
        ("nine","9"),
    ]);
    
    for l in input.lines(){
        println!("{}",l);
        let mut digits = String::new();
        let mut l_pos = l.len();
        let mut h_pos = 0;
        let mut first_digit = String::new();
        let mut last_digit = String::new();
        for e in numbers{

            if let Some(digit) = l.find(e){
                if digit < l_pos{
                    l_pos = digit;
                    first_digit.replace_range(..,e);
                }
            }
            if let Some(digit) = l.rfind(e){
                if digit >= h_pos{
                    h_pos = digit;
                    last_digit.replace_range(..,e);
                }
            }
        }
        if let Some(new_digit) = num_conv.get(&*first_digit){
            first_digit.replace_range(..,new_digit);
        }
        if let Some(new_digit) = num_conv.get(&*last_digit){
            last_digit.replace_range(..,new_digit);
        }
        println!("{}{}",first_digit,last_digit);
        digits.push_str(&first_digit);
        digits.push_str(&last_digit);

        match digits.parse::<i32>(){
            Ok(parsed_num) => {
                result += parsed_num;
            }
            Err(e) => {
                println!("Something weird happened with {}, idk", e);
            }
        }
    }
    result
}

fn day_2_1(input: &str,cube_count: &HashMap<&str, i32>) -> i32{
    let mut result = 0;
    'line_loop: for l in input.lines(){
        let game_substr: Vec<&str> = l.split(':').collect();
        let game_title: String = game_substr[0]
            .chars()
            .filter(|c| c.is_numeric())
            .collect();
        let game_no: i32 = game_title.parse().unwrap();
        println!("{}", game_no);
        let grabs: Vec<&str> = game_substr[1].split(&[';',','][..]).collect();
        for g in grabs{
            let val_col: Vec<&str> = g.split(' ').collect();
            // need to compare the value of the grab to the cube count of a given color and reject the game
            // if the value exceeds the value in cube_count.
            let g_val: &i32 = &val_col[1].parse().unwrap();
            if let Some(val) = cube_count.get(&val_col[2]){
                if g_val > val{
                    println!("Game is no good! {}",l);
                    continue 'line_loop
                }
            }
            else{
                println!("Weird result for {}", g);
            }
        }
        result += game_no;    
    }
    result
}

fn day_2_2(input: &str) -> i32{
    let mut result = 0;
    for l in input.lines(){
        let mut rgb = HashMap::from([
            ("red",0),
            ("green",0),
            ("blue",0)
        ]);
        let game_substr: Vec<&str> = l.split(':').collect();
        let grabs: Vec<&str> = game_substr[1].split(&[';',','][..]).collect();
        for g in grabs{
            let val_col: Vec<&str> = g.split(' ').collect();
            let g_val: &i32 = &val_col[1].parse().unwrap();
            if let Some(val) = rgb.get(&val_col[2]){
                if *val == 0 || *val < *g_val{
                    rgb.insert(&val_col[2],*g_val);
                }
            }
        }
        result += rgb.values().product::<i32>();
    }
    result
}
// collect coordinates for each symbol and number, as well as the length of the number. 
// For each number, check a set of coordinates for the existence of a symbol. 
// (say coordinates for the number are r,c with l=3. you would then check [r-1][c-1:c+1],[r][c-1,c+1],[r+1][c-1:c+1])
fn day_3_1(input: &str) -> i32 {
    let mut result = 0;
    let d_pattern = Regex::new(r"\d+").unwrap();
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut symbol_coords:Vec<(usize,usize)> = vec![];
    let mut num_coords:Vec<(i32,usize,usize,usize)> = vec![];

    for (l_no, line) in schematic.iter().enumerate(){
        for (c_no, c) in line.iter().enumerate(){
            if !c.is_alphanumeric() && *c != '.'{
                symbol_coords.push((l_no,c_no));
            }
        }
        let hay: String = line.iter().collect();
        for found in d_pattern.find_iter(&hay /*fuck*/){
            let length = found.as_str().len();
            let index = found.start();
            let number = found.as_str().parse::<i32>().unwrap();
            num_coords.push((number,l_no,index,length));
        }
    }

    for coords in num_coords{
        let mut char_dump:Vec<char> = vec![];
        let number = coords.0;
        let line_no = coords.1;
        let n_index = coords.2;
        let n_len = coords.3;
        let mut prev_line: Option<&Vec<char>> = None;
        let mut next_line: Option<&Vec<char>> = None;
        if line_no != 0{
            prev_line = schematic.get(line_no - 1);
        }
        let curr_line = schematic.get(line_no).unwrap(); 
        if line_no != schematic.len(){
            next_line = schematic.get(line_no + 1);
        }
        let l_index = if n_index > 0 {
            n_index - 1
        }
        else{
            n_index
        };
        let r_index = if n_index + n_len < curr_line.len(){
            n_index + n_len + 1
        }
        else{
            n_index + n_len
        };
        if let Some(prev_line) = prev_line{
            let trunc_prev_line = &prev_line[l_index..r_index];
            for c in trunc_prev_line{
                char_dump.push(*c);
            }
        }
        let trunc_curr_line = &curr_line[l_index..r_index];
            for c in trunc_curr_line{
                char_dump.push(*c);
            }

        if let Some(next_line) = next_line{
            let trunc_next_line = &next_line[l_index..r_index];
            for c in trunc_next_line{
                char_dump.push(*c);
            }
        };
        for c in char_dump{
            if ! c.is_alphanumeric() && c != '.'{
                result += number;
            }
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("src/input_3.txt")
        .expect("unable to read file");
    //let cube_count = HashMap::from([("red",12),("green",13),("blue",14)]);
    println!("{:?}",day_3_1(&input));
}
