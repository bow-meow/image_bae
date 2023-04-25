use std::{str::FromStr, fmt::Display};
use rand::{self, Rng};
use csscolorparser::{Color, ParseColorError};

use super::param_value_parser;
use super::param_types_enum::ParamType;



#[derive(Clone)]
pub struct ImageColor{
    pub colors : Vec<[u8; 4]>,
    pub rand : bool
}

impl ImageColor{
    pub fn get_colors(&self, numof_colors: usize) -> Vec<[u8; 4]>{
        let mut colors : Vec<[u8; 4]> = Vec::new();
        for _ in 0..numof_colors{
            if self.rand{
                let mut rng = rand::thread_rng();
                if self.colors.len() > 1{
                    for _ in 0..numof_colors{
                        let idx = rng.gen_range(0..self.colors.len());
                        colors.push((&self).colors[idx]);
                    }
                }
                else{
                    for _ in 0..numof_colors{
                        let r = rng.gen_range(0..255);
                        let g =  rng.gen_range(0..255);
                        let b =  rng.gen_range(0..255);
                        colors.push([r,g,b,1]);
                    }
                }
            }
            else if self.colors.len() > 0{
                let mut num = numof_colors;

                while num > 0{
                    for i in 0..self.colors.len(){
                        colors.push((&self).colors[i]);
                        num-=1;
                        if num == 0{
                            break;
                        }
                    }
                }
            }
            else{
                colors.push([0; 4]);
            }
        }

        colors
    }
}

pub struct ImageColorParseErr;

impl Display for ImageColorParseErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The value provided was not in the correct format.")
    }
}

impl FromStr for ImageColor{
    type Err = ImageColorParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "none" {
            return Ok(Self{colors: Vec::new(), rand: false })
        }
        return match param_value_parser::parse_param_value(s, ParamType::ImageColor){
            Ok(items) => {
                let (colors, rand) = items;
                match colors.iter()
                            .map(|color| csscolorparser::parse(color))
                            .collect::<Result<Vec<Color>, ParseColorError>>(){
                    Ok(colors) =>{
                        let hex_colors = colors.iter().map(|c| c.to_rgba8()).collect::<Vec<[u8; 4]>>();
                        Ok(Self{colors: hex_colors, rand })
                    },
                    Err(_e) => Err(ImageColorParseErr)
                }
            },
            Err(_e) => Err(ImageColorParseErr)
        }
    }
}

pub fn parse_colors(s: &str) -> Result<ImageColor, String> {
    match s.parse::<ImageColor>(){
        Ok(color) => Ok(color),
        Err(e) => Err(e.to_string())
    }
}