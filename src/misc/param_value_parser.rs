use super::param_types_enum::ParamType;

#[derive(Debug)]
pub struct ParseParamError;

pub fn parse_param_value(s : &str, param_type : ParamType) -> Result<(Vec<String>, bool), ParseParamError>{
    if s.starts_with("rand"){
        let parsable = &s[4..];
        if parsable.len() == 0{
            return match param_type{
                ParamType::ImageSize => Err(ParseParamError),
                _ => Ok((vec![], true))
            }
            
        }
        else if parsable.starts_with('(') && parsable.ends_with(')'){
            let parsable = &parsable[1..parsable.len() - 1];
            return parse_param_contents(parsable, true, &param_type);
        };
        return Err(ParseParamError);
    }
    return parse_param_contents(s, false, &param_type); 
}

fn parse_param_contents(s : &str, is_rand: bool, param_type: &ParamType) -> Result<(Vec<String>, bool), ParseParamError>{
    return match parse_to_param(s, is_rand, param_type){
        Ok(item) => {
            match item{
                Some(i) => return Ok(i),
                None => {
                    return match param_type{
                        ParamType::ImageSize => Ok((vec![s.to_string()], false)),
                        ParamType::ImageColor =>{
                            let mut color_list : Vec<String> = Vec::new();
                            let mut sss : &str = s.trim();
                            loop{
                                if sss.len() > 0{
                                    if sss.starts_with('('){
                                        let index = sss.find(')').unwrap();
                                        let (part, other) = sss.split_at(index + 1);
                                        if other.len() > 0{
                                            sss = other[1..].trim();
                                        }
                                        else{
                                            sss = other.trim();
                                        };
                                        
                                        color_list.push(format!("rgb{}", part).to_string());
                                    }
                                    else{
                                        let split = sss.split_once(',');
                                        match split{
                                            Some((part, other)) =>{
                                                color_list.push(part.to_string());
                                                sss = other.trim();
                                            },
                                            None => {
                                                color_list.push(sss.to_string());
                                                break;
                                            }
                                        }
                                    }
                                    
                                }
                                else{
                                    break;
                                }
                            }
                            Ok((color_list, is_rand))
                        },
                        _ => {
                            let items : Vec<String> = s.trim().split(',').map(|f| f.to_string()).collect();
                            Ok((items, is_rand))
                        }
                    }
                    
                }
            };
        }
        Err(e) => Err(e)
    }
}

fn parse_to_param(s : &str, is_rand : bool, param_type: &ParamType) -> Result<Option<(Vec<String>, bool)>, ParseParamError>{
    if s.contains(".."){
        return match param_type{
            ParamType::ImageSize => {
                let data : Vec<String> = s.trim().split("..").map(|d| d.to_string()).collect();
                if data.len() == 2{
                    return Ok(Some((data, is_rand)));
                }
                Err(ParseParamError)
            },
            _ => Err(ParseParamError)
        }
    }
    Ok(None)
}