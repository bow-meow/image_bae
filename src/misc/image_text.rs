use std::{str::FromStr};

#[derive(Clone)]
pub struct ImageText{
    pub text : Vec<(String, TextType)>
}
#[derive(Clone)]
pub enum TextType {
    Word,
    Num,
    Char
}

#[derive(Clone)]
pub struct ParseImageTextErr;

pub fn parse_text(s: &str) -> Result<ImageText, String> {
    match s.parse::<ImageText>(){
        Ok(image_text) => Ok(image_text),
        Err(e) => Err(e.to_string())
    }
}

impl std::fmt::Display for ParseImageTextErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value was not the correct format")
    }
}

impl FromStr for ImageText{
    type Err = ParseImageTextErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss : &str = s;
        let mut result: Vec<(String, TextType)> = Vec::new(); 
        if ss.len() > 0{
            loop{
                match ss.find('['){
                    Some(first_idx) =>{
                        match ss.find(']'){
                            Some(last_idx) => {
                                let expresion = &ss[first_idx + 1..last_idx];
                                if expresion.contains(".."){
                                    let parts = expresion.split("..").map(|i| i).collect::<Vec<&str>>();
                                    if parts.len() == 2{
                                        match parts.iter()
                                                    .map(|i| i.parse::<i32>())
                                                    .collect::<Result<Vec<i32>, std::num::ParseIntError>>(){
                                                Ok(items) =>{
                                                    let item1 = &items[0];
                                                    let item2 = &items[1];
                                                    let string_nums : Vec<String>;
                                                    if item1 > item2{
                                                        string_nums = get_num_range_as_string(item2, item1, true);
                                                    }
                                                    else{
                                                        string_nums = get_num_range_as_string(item1, item2, false);
                                                    }
                                                    result.push((ss[..first_idx].to_string(), TextType::Char));
                                                    for item in string_nums{ //TODO do inside generation loop
                                                        result.push((item, TextType::Num));
                                                    }
                                                    ss = &ss[last_idx + 1..];
                                                },
                                                Err(_e) =>{
                                                    let item1 = &parts[0];
                                                    let item2 = &parts[1];

                                                    if item1.len() == 1 
                                                    && item2.len() == 1 
                                                    && char::is_alphabetic(item1.chars().next().unwrap()) 
                                                    && char::is_alphabetic(item2.chars().next().unwrap()){
                                                        let char1 = item1.chars().next().unwrap();
                                                        let char2 = item2.chars().next().unwrap();
                                                        let ascii_1 = char1 as u32;
                                                        let ascii_2 = char2 as u32;
                                                        let mut ascii_alphabet = String::new();
                                                        if ascii_1 > ascii_2{
                                                            for i in (ascii_2..ascii_1 + 1).rev(){
                                                                ascii_alphabet.push(char::from_u32(i).unwrap());
                                                            }
                                                        }
                                                        else{
                                                            for i in ascii_1..ascii_2 + 1{
                                                                ascii_alphabet.push(char::from_u32(i).unwrap());
                                                            }
                                                        }
                                                        result.push((ss[..first_idx].to_string(), TextType::Char));
                                                        result.push((ascii_alphabet, TextType::Char));
                                                    }
                                                    else{
                                                        result.push((ss[0..last_idx + 1].to_string(), TextType::Char));
                                                        
                                                    }
                                                    ss = &ss[last_idx + 1..];
                                                }
                                        }
                                    }
                                    else{
                                        result.push((ss[0..last_idx + 1].to_string(), TextType::Char));
                                        ss = &ss[last_idx + 1..];
                                    }
                                }
                                else{
                                    // parse word
                                    result.push((ss[0..first_idx].to_string(), TextType::Char));
                                    result.push((ss[first_idx + 1..last_idx].to_string(), TextType::Word));

                                    ss = &ss[last_idx + 1..];
                                }
                            },
                            None => {
                                result.push((ss[0..first_idx + 1].to_string(), TextType::Char));
                                ss = &ss[first_idx + 1..];
                            }
                        }
                    },
                    None => {
                        result.push((ss.to_string(), TextType::Char));
                        break;
                    }
                }
            }
            
            return Ok(Self { text: result});
        }
        Err(ParseImageTextErr)
    }
        
}

fn get_num_range_as_string(item1: &i32, item2: &i32, reverse: bool) -> Vec<String>{
    let mut s: Vec<String> = Vec::new();
    if reverse{
        for i in (*item1..*item2 + 1).rev(){
            s.push(i.to_string());
        }
    }
    else{
        for i in *item1..*item2 + 1{
            s.push(i.to_string());
        }
    }
    s
}