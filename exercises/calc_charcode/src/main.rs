fn main() {
    let res = calc("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    println!("{}", res); // will return 96.
}

fn calc(s: &str) -> u32{
    //str.chars() return a iterator over the char of a string slice
    //str.as_bytes() covert a string slice to a byte slice.
    //to convert the byte slice back into a string slice, use the `from_utf8()` function

    let chars_code = s.as_bytes(); 
    let chars_code_string = chars_code.iter().map(|x| x.to_string()).collect::<String>();
    
    //I could also write in this way:
    //let chars: String = chars.iter().map(|x| x.to_string()).collect();
   
    let old_num: u32 = chars_code_string.clone().chars().map(|x| x as u32).sum();   

    let new_chars_code_string: String = chars_code_string.chars()
        .map(|c| match c {
            '7' => '1',
            _ => c,
        })
        .collect(); 

    let new_num:u32 = new_chars_code_string.chars().map(|x| x as u32).sum();
    //unwrap() returns the contained Some/Ok value, consuming the self value.
    //sum() in a iterator sum all the values
    //I could also write this way:
    //let chars = str::replace(chars, "7","1");
    
    let res: u32 = old_num - new_num;
    res
}
