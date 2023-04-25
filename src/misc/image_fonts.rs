use std::borrow::Cow;
use std::{str::FromStr, fmt::Display};
use rand::{self, Rng};
use crate::Fonts;

use super::param_value_parser;
use super::param_types_enum::ParamType;

#[derive(Clone)]
pub enum FontType {
    Special,
    Normal
}

#[derive(Clone)]
pub struct ImageFont{
    pub fonts : Vec<(String, FontType)>,
    pub rand : bool
}

impl ImageFont{
    pub fn get_fonts(&self, numof_fonts: usize) -> Vec<(String, FontType)>{
        let mut formats : Vec<(String, FontType)> = Vec::new();
        for _ in 0..numof_fonts{
            if self.rand{
                let mut rng = rand::thread_rng();
                if self.fonts.len() > 1{
                    for _ in 0..numof_fonts{
                        let idx = rng.gen_range(0..self.fonts.len());
                        let font = self.fonts[idx].clone();
                        formats.push(font);
                    }
                }
                else{
                    let fonts = Fonts::iter().filter(|f| !f.contains("sesame")).collect::<Vec<Cow<str>>>();
                    for _ in 0..numof_fonts{
                        let num = rng.gen_range(0..fonts.len());
                        let font = fonts[num].to_string();

                        formats.push((font.to_string(), FontType::Normal));
                    }
                }
            }
            else{
                let mut num = numof_fonts;

                while num > 0{
                    for i in 0..self.fonts.len(){
                        formats.push(self.fonts[i].clone());
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
}

#[derive(Clone)]
pub struct ParseFontErr;

impl Display for ParseFontErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The value provided was not in the correct format.")
    }
}

impl FromStr for ImageFont{
    type Err = ParseFontErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match param_value_parser::parse_param_value(s, ParamType::ImageFont){
            Ok((mut fonts, rand)) =>{
                for font in fonts.iter_mut(){
                    if !is_special_font(&font){
                        if !font.ends_with(".ttf"){
                            *font += ".ttf";
                        }
                        if Fonts::get(font).is_none(){
                            return Err(ParseFontErr);
                        }
                    }
                }

                let fonts = fonts.iter().map(|f| {
                    if is_special_font(f){
                        return (f.to_owned(), FontType::Special);
                    }
                    (f.to_owned(), FontType::Normal)
                }).collect::<Vec<(String, FontType)>>();

                Ok(Self { fonts, rand})
            },
            Err(_e) => Err(ParseFontErr)
        } 
    }
}

fn is_special_font(font: &String) -> bool{
    if font == "pokemon" || font == "sesame"{
        return true;
    }
    false
}

pub fn parse_fonts(s: &str) -> Result<ImageFont, String> {
    match s.parse::<ImageFont>(){
        Ok(font) => Ok(font),
        Err(e) => Err(e.to_string())
    }
}