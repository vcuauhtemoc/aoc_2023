use std::{fs, collections::HashMap};

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
        dbg!(&result,&rgb,&l);


        //result += red * green * blue;
    }
    result //answer is too low
}
fn main() {
    let input = fs::read_to_string("src/input_2.txt")
        .expect("unable to read file");
    let cube_count = HashMap::from([("red",12),("green",13),("blue",14)]);
    println!("{:?}",day_2_2(&input));
}
