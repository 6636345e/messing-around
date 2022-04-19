use std::io;

pub fn solve001() {
    println!("Program 001");
    let mut words_count = String::new();

    //declare an array here - name it words array
    let mut words_array: Vec<String> = Vec::new();
     

    io::stdin()
        .read_line(&mut words_count)
        .expect("Failed to read the line");

    let words_count_int: u32 = words_count.trim().parse().expect("Please type a number as integer type!");
    let mut counter: u32 = 1;
    while counter <= words_count_int {
        let mut user_string = String::new();
        io::stdin()
            .read_line(&mut user_string)
            .expect("Failed to read the line");
        words_array.push(user_string);
        counter += 1;
    }

    for word in words_array.iter(){
        process_word(word.to_string());
    }
}

fn process_word(word:String){

    
    if word.len() >= 1 && word.len() <= 100{    
        if word.len() > 10 {
        //convert a given string into an array of characters
    
        let mut array_chars: Vec<char> = word.chars().collect::<Vec<_>>();
        let first_char: char = array_chars[0];
        array_chars.remove(0);
        array_chars.pop();
        let middle_length = array_chars.len()-1;
        print!("{}",first_char);
        print!("{}",middle_length);
        println!("{}",array_chars[array_chars.len()-1]);
        
        }else{
            print!("{}",word);
        }
    }
}
