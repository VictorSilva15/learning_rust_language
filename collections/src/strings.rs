pub mod manipulating_strings {
    pub fn creating_strings(s: &str) -> String{
        return s.to_string();
    }
    pub fn updating_strings(s: &str, new_s: &str) -> String{
        let mut s = String::from(s);
        //push_str concotenate strins
        s.push_str(new_s);
        s
    }
    pub fn updating_character(s: &str, new_char: char) -> String {
        let mut s = String::from(s);
        //push concatenate characters
        s.push(new_char);
        s
    }
}
