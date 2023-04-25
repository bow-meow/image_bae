use std::{str::FromStr, fmt::Display};
use rand::{self, Rng};
use super::param_value_parser;
use super::param_types_enum::ParamType;

const EXTENSIONS:  [&str;3] = [".jpg", ".png", ".tif"];

#[derive(Clone)]
pub struct ImageFormat{
    pub formats : Vec<String>,
    pub rand : bool
}

impl ImageFormat{
    pub fn get_formats(&self, numof_formats: usize) -> Vec<String>{
        //TODO make this method a trait, impl it for fonts,color and have one function deal with it
        let mut formats : Vec<String> = Vec::new();
        for _ in 0..numof_formats{
            if self.rand{
                let mut rng = rand::thread_rng();
                if self.formats.len() > 1{
                    for _ in 0..numof_formats{
                        let idx = rng.gen_range(0..self.formats.len());
                        let format = &self.formats[idx];
                        formats.push(format.to_string());
                    }
                }
                else{
                    for _ in 0..numof_formats{
                        let idx = rng.gen_range(0..EXTENSIONS.len());
                        let format = EXTENSIONS[idx];
                        formats.push(format.to_string());
                    }
                }
            }
            else{
                let mut num = numof_formats;

                while num > 0{
                    for i in 0..self.formats.len(){
                        formats.push((&self).formats[i].to_string());
                        num-=1;
                        if num == 0{
                            break;
                        }
                    }
                }
            }
        }

        formats
    }

    pub fn get_formattype_for_str(format_str: &String) -> Option<image::ImageFormat>{
        if format_str == ".jpg"{
            return Some(image::ImageFormat::Jpeg);
        }
        else if format_str == ".png"{
            return Some(image::ImageFormat::Png);
        }
        else if format_str == ".tif"{
            return Some(image::ImageFormat::Tiff);
        }
        None
    }
}

pub struct ImageFormatParseErr;

impl Display for ImageFormatParseErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The value provided was not in the correct format.")
    }
}


impl FromStr for ImageFormat{
    type Err = ImageFormatParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match param_value_parser::parse_param_value(&s, ParamType::ImageFormat){
            Ok((formats, rand)) => {
                let formats = formats.iter().map(|format| {
                    if !format.starts_with('.'){
                        return format!(".{}", format).to_owned();
                    }
                    else{
                        return format.to_owned();
                    }
                }).collect::<Vec<String>>();


                for format in formats.iter(){
                    let mut has_format = false;
                    for ex in EXTENSIONS.iter(){
                        if *ex == format{
                            has_format = true;
                        }
                    }
                    if !has_format{
                        return Err(ImageFormatParseErr)
                    }
                }


                Ok(Self { formats, rand })
            },
            Err(_e) => return Err(ImageFormatParseErr)
        }
    }
}

pub fn parse_formats(s: &str) -> Result<ImageFormat, String> {
    match s.parse::<ImageFormat>(){
        Ok(format) => Ok(format),
        Err(e) => Err(e.to_string())
    }
}