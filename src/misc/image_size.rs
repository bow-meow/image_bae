use std::str::FromStr;
use rand::{self, Rng};
use super::param_value_parser;
use super::param_types_enum::ParamType;

#[derive(Clone)]
pub struct ImageSizeCollection{
    image_sizes: Vec<ImageSize>
}

#[derive(Clone)]
pub struct ImageSize{
    widths: Vec<u32>,
    heights: Vec<u32>,
    rand_width: bool,
    rand_height: bool
}
impl ImageSizeCollection{
    pub fn get_sizes(&self, numof_sizes: usize) -> Vec<Size>{
        let mut size_list: Vec<Size> = Vec::new();
        for _ in 0..numof_sizes{
            for image_size in self.image_sizes.iter(){
                let height = Self::parse_width_or_height(&image_size.heights, image_size.rand_height);
                let width = Self::parse_width_or_height(&image_size.widths, image_size.rand_width);
                size_list.push(Size{width, height});
            }
        }
        size_list
    }

    fn parse_width_or_height(width_or_height: &Vec<u32>, is_rand: bool) -> u32{
        let mut rng = rand::thread_rng();
        if is_rand{
            if width_or_height.len() == 0{
                return rng.gen_range(100..4000) as u32;
            }
            else{
                let width_or_height1 = width_or_height[0] as i32;
                let width_or_height2 = width_or_height[1] as i32;

                if width_or_height1 > width_or_height2{
                    return rng.gen_range(width_or_height2..width_or_height1) as u32;
                }
                else{
                    return rng.gen_range(width_or_height1..width_or_height2) as u32;
                }
            }
        }
        width_or_height[0]
    }
}

#[derive(Clone)]
pub struct Size{
    pub width: u32,
    pub height: u32
}



pub fn parse_size(s: &str) -> Result<ImageSizeCollection, String> {
    match s.parse::<ImageSizeCollection>(){
        Ok(size) => Ok(size),
        Err(e) => Err(e.to_string())
    }
}

#[derive(Clone)]
pub struct ParseSizeErr;

impl std::fmt::Display for ParseSizeErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value was not the correct format")
    }
}

impl FromStr for ImageSizeCollection{
    type Err = ParseSizeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let sizes = s.split(',').map(|s| s.to_string()).collect::<Vec<String>>();
        let mut size_list: Vec<ImageSize> = Vec::new();
        for size in sizes.iter(){
            if size.contains('x'){
                let parts : Vec<String> = size.split('x').map(|part| part.to_string()).collect();
                if parts.len() == 2 {
                    let mut list : Vec<(Vec<String>, bool)> = Vec::new();
                    for part in parts{
                        match param_value_parser::parse_param_value(&part, ParamType::ImageSize){
                            Ok(item) => list.push(item),
                            Err(_) => return Err(ParseSizeErr)
                        }
                    }
    
                    let (width_param, width_rand) = &list[0];
                    let (height_param, height_rand) = &list[1];
    
                    let width_result = width_param.iter().map(|text| text.parse::<u32>()).collect::<Result<Vec<u32>, std::num::ParseIntError>>();
                    let height_result = height_param.iter().map(|text| text.parse::<u32>()).collect::<Result<Vec<u32>, std::num::ParseIntError>>();
                    if width_result.is_err() || height_result.is_err(){
                        return Err(ParseSizeErr);
                    }
    
                    let width_vec = width_result.unwrap();
                    if width_vec.len() == 2 && width_vec[0] > width_vec[1]{
                        return Err(ParseSizeErr);
                    }
                    let height_vec = height_result.unwrap();
                    if height_vec.len() == 2 && height_vec[0] > height_vec[1]{
                        return Err(ParseSizeErr);
                    }

                    size_list.push(ImageSize{widths: width_vec, heights: height_vec, rand_width: *width_rand, rand_height: *height_rand});
                }
                else{
                    return Err(ParseSizeErr)
                }
            }
            else{
                match param_value_parser::parse_param_value(size, ParamType::ImageSize){
                    Ok(item) => {
                        let val = item.0.iter().map(|size| size.parse::<u32>()).collect::<Result<Vec<u32>, std::num::ParseIntError>>();
                        if val.is_err(){
                            return Err(ParseSizeErr);
                        }
                        let val = val.unwrap();
                        let y_val = val.clone();
    
                        match val.len(){
                            1 => size_list.push(ImageSize{widths: val, heights: y_val, rand_height: false, rand_width: false}),
                            2 => {
                                if val[0] > val[1]{
                                    return Err(ParseSizeErr); 
                                }

                                size_list.push(ImageSize{widths: val, heights: y_val, rand_width: true, rand_height: true});
                            },
                            _ => return Err(ParseSizeErr)
                        }
                    },
                    Err(_e) => return Err(ParseSizeErr)
                }
            }
        }
        Ok(Self{ image_sizes: size_list})
    }
}
