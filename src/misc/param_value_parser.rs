use super::param_types_enum::ParamType;

#[derive(Debug)]
pub struct ParseParamError;

pub fn parse_param_value(str : &str, param_type : ParamType) -> Result<(Vec<String>, bool), ParseParamError>{
    if str.starts_with("rand"){
        let parsable = &str[4..];
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
    return parse_param_contents(str, false, &param_type); 
}

fn parse_param_contents(str : &str, is_rand: bool, param_type: &ParamType) -> Result<(Vec<String>, bool), ParseParamError>{
    return match parse_to_param(str, is_rand, param_type){
        Ok(item) => {
            match item{
                Some(i) => return Ok(i),
                None => {
                    return match param_type{
                        ParamType::ImageSize => Ok((vec![str.to_string()], false)),
                        ParamType::ImageColor =>{
                            let mut color_list : Vec<String> = Vec::new();
                            let mut trimmed_str : &str = str.trim();
                            loop{
                                if trimmed_str.len() > 0{
                                    if trimmed_str.starts_with('('){
                                        let index = trimmed_str.find(')').unwrap();
                                        let (part, other) = trimmed_str.split_at(index + 1);
                                        if other.len() > 0{
                                            trimmed_str = other[1..].trim();
                                        }
                                        else{
                                            trimmed_str = other.trim();
                                        };
                                        
                                        color_list.push(format!("rgb{}", part).to_string());
                                    }
                                    else{
                                        let split = trimmed_str.split_once(',');
                                        match split{
                                            Some((part, other)) =>{
                                                color_list.push(part.to_string());
                                                trimmed_str = other.trim();
                                            },
                                            None => {
                                                color_list.push(trimmed_str.to_string());
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
                            let items : Vec<String> = str.trim().split(',').map(|f| f.to_string()).collect();
                            Ok((items, is_rand))
                        }
                    }
                    
                }
            };
        }
        Err(e) => Err(e)
    }
}

fn parse_to_param(str : &str, is_rand : bool, param_type: &ParamType) -> Result<Option<(Vec<String>, bool)>, ParseParamError>{
    if str.contains(".."){
        return match param_type{
            ParamType::ImageSize => {
                let data : Vec<String> = str.trim().split("..").map(|d| d.to_string()).collect();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_param_value_image_size() {
        // Test valid image size without random
        let result = parse_param_value("100..200", ParamType::ImageSize);
        assert!(result.is_ok());
        let (values, is_rand) = result.unwrap();
        assert_eq!(values, vec!["100", "200"]);
        assert!(!is_rand);

        // Test random image size with valid range
        let result = parse_param_value("rand(100..200)", ParamType::ImageSize);
        assert!(result.is_ok());
        let (values, is_rand) = result.unwrap();
        assert_eq!(values, vec!["100", "200"]);
        assert!(is_rand);
    }

    #[test]
    fn test_parse_param_value_image_color() {
        // Test valid image color without random
        let result = parse_param_value("255,0,0", ParamType::ImageColor);
        assert!(result.is_ok());
        let (values, is_rand) = result.unwrap();
        assert_eq!(values, vec!["255", "0", "0"]);
        assert!(!is_rand);

        // Test valid image color with RGB format
        let result = parse_param_value("(255,0,0)", ParamType::ImageColor);
        assert!(result.is_ok());
        let (values, is_rand) = result.unwrap();
        assert_eq!(values, vec!["rgb(255,0,0)"]);
        assert!(!is_rand);

        // Test random image color with valid format
        let result = parse_param_value("rand(255,0,0)", ParamType::ImageColor);
        assert!(result.is_ok());
        let (values, is_rand) = result.unwrap();
        assert_eq!(values, vec!["255", "0", "0"]);
        assert!(is_rand);

        // Test random image color with empty range
        let result = parse_param_value("rand()", ParamType::ImageColor);
        assert!(result.is_ok());
        let (values, is_rand) = result.unwrap();
        assert!(values.is_empty());
        assert!(is_rand);
    }
}