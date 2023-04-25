
use image::{RgbImage, Rgb};
use imageproc::{drawing::draw_text_mut};
use rusttype::{Font, Scale, point};
use rgb::RGB8;
use std::{fs::{self, File}, path::Path, io::{Write}};
use crate::{image_instruction::{ImageInstructions, ImageInstruction}, Fonts, misc::{image_fonts::FontType, image_format}};


pub struct ImageGenerator{
    pub image_instructions: ImageInstructions
}

impl ImageGenerator{
    pub fn new(instructions: ImageInstructions) -> Self{
        Self{ image_instructions: instructions}
    }
    pub fn run(&self){
        if Path::new("images").exists(){
            fs::remove_dir_all("images").expect("unable to remove directory for images");
        }

        fs::create_dir_all("images").expect("unable to create directory for images");

        let mut i: usize = 0;

        for instruction in self.image_instructions.instructions.iter(){

            let (font, font_type) = &instruction.font;
            match font_type{
                FontType::Normal =>{
                    Self::create_from_font(instruction, font, &mut i);
                },
                FontType::Special => {
                    Self::create_from_image(instruction, font, &mut i)
                }
            }
        }
    }

    fn create_from_font(instruction: &ImageInstruction, font: &str, i: &mut usize){
        let embeded_font = Fonts::get(font).unwrap();
        let font = Font::try_from_vec(embeded_font.data.to_vec()).expect(&format!("Unable to read font: {}", font));

        let background_r = instruction.image_color[0];
        let background_g = instruction.image_color[1];
        let background_b = instruction.image_color[2];
        //let a = instruction.image_color[3];

        let mut foreground_r : u8 = 0;
        let mut foreground_g : u8 = 0;
        let mut foreground_b : u8 = 0;

        if !instruction.image_text_color.is_empty(){
            foreground_r = instruction.image_text_color[0];
            foreground_g = instruction.image_text_color[1];
            foreground_b = instruction.image_text_color[2];
        }
        else{
            let luminance : f32 = contrast::luminance(RGB8::from([background_r,background_g,background_b]));
        
            if luminance > 0.5{
                foreground_r = 0;
                foreground_g = 0;
                foreground_b = 0;
            }
        }


        let mut image: RgbImage = RgbImage::from_pixel(instruction.image_size.width, instruction.image_size.height, Rgb([background_r, background_g, background_b]));

        let scale = get_scale(instruction.image_size.width as f32, instruction.image_size.height as f32, &instruction.character, &font);
        let (line_width, _) = measure_line(&font, &instruction.character, scale);
        let loc_x = (instruction.image_size.width as f32 - line_width) as i32 / 2;
        let loc_y = 0;
        draw_text_mut(&mut image, Rgb([foreground_r, foreground_g, foreground_b]), loc_x, loc_y, scale, &font, instruction.character.as_str());

        let img_name = generate_filename(instruction, i);
        let path_to_image = format!("images/{}", img_name);
        let mut file = File::create(&path_to_image).unwrap();
        image.write_to(&mut file, image_format::ImageFormat::get_formattype_for_str(&instruction.image_format).unwrap()).unwrap();
        pad_file_size(&mut file, instruction.image_filesize);
        println!("created {}", img_name);

        *i += 1;
    }

    fn create_from_image(instruction: &ImageInstruction, font: &str, i: &mut usize){
        if instruction.character.chars().all(|c| !c.is_alphabetic() ){
            Self::create_from_font(instruction, "comicsans.ttf", i);
            return;
        }
        for c in instruction.character.chars(){
    
            if !c.is_alphabetic(){
                Self::create_from_font(instruction, "comicsans.ttf", i);
                continue;
            }
    
            let img_data = Fonts::get(&format!("{}/{}.jpg", font, c.to_ascii_uppercase())).unwrap();
    
            let img = image::load_from_memory(&img_data.data).unwrap();
            let resized_img = img.resize_exact(instruction.image_size.width, instruction.image_size.height, image::imageops::FilterType::Nearest);
            let mut img = resized_img.to_rgb8();
    
            let background_r = instruction.image_color[0];
            let background_g = instruction.image_color[1];
            let background_b = instruction.image_color[2];
            //let a = instruction.image_color[3];
    
            let mut foreground_r : u8 = 0;
            let mut foreground_g : u8 = 0;
            let mut foreground_b : u8 = 0;
    
            if !instruction.image_text_color.is_empty(){
                foreground_r = instruction.image_text_color[0];
                foreground_g= instruction.image_text_color[1];
                foreground_b = instruction.image_text_color[2];
            }
            else{
                let luminance : f32 = contrast::luminance(RGB8::from([background_r,background_g,background_b]));
            
                if luminance > 0.5{
                    foreground_r = 0;
                    foreground_g = 0;
                    foreground_b = 0;
                }
            }
    
            for pixel in img.pixels_mut(){
                if pixel.0[0] > 200 && pixel.0[1] > 200 && pixel.0[2] > 200{
                    pixel.0[0] = instruction.image_color[0];
                    pixel.0[1] = instruction.image_color[1];
                    pixel.0[2] = instruction.image_color[2];
                }
                else if pixel.0[0] < 50 && pixel.0[1] < 50 && pixel.0[2] < 50{
                    pixel.0[0] = foreground_r;
                    pixel.0[1] = foreground_g;
                    pixel.0[2] = foreground_b;
                }
            }
    
            // img-index-character-width_height-color-font.format
            let img_name = generate_filename(instruction, i);
            let path_to_image = format!("images/{}", img_name);
            let mut file = File::create(&path_to_image).unwrap();
            img.write_to(&mut file, image_format::ImageFormat::get_formattype_for_str(&instruction.image_format).unwrap()).unwrap();
            pad_file_size(&mut file, instruction.image_filesize);
            println!("created {}", img_name);

            *i += 1;
        }
    }
}

fn pad_file_size(img_file: &mut File, target_filesize_mb: u32){

    if target_filesize_mb == 0{
        return;
    }

    let current_size_kb = img_file.metadata().unwrap().len() as u32 / 1024;

    let padding_size_kb = target_filesize_mb * 1024 - current_size_kb;

    println!("{}", padding_size_kb);

    let padding_bytes = vec![0u8; (padding_size_kb * 1024) as usize];
    img_file.write_all(&padding_bytes).unwrap();
}

fn measure_line(font: &Font, text: &str, scale: Scale) -> (f32, f32) {
    let width = font
        .layout(text, scale, point(0.0, 0.0))
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.0);

    let v_metrics = font.v_metrics(scale);
    let height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

    (width, height)
}

fn get_scale(x: f32, y: f32, txt: &str, font: &Font) -> Scale{
    let scale = Scale{ 
        x,
        y
    };

    let (w,_) = measure_line(font, txt, scale);

    if w > scale.x{
        let one = (w - x) as f32;
        let two = one / 2.0 as f32;
        let three = x - two;
        let four = three - 50.0;
        return Scale{
            x: four,
            y: y
        }
    }
    scale
}

fn generate_filename(instruction: &ImageInstruction, image_index: &usize) -> String{
    let character_string = instruction.character.chars().map(|c|{
        if c.is_alphanumeric(){
            c
        }
        else{
            '$'
        }
        
    }).collect::<String>();

    let font = match instruction.font.0.find('.'){
        Some(dot_index) => instruction.font.0[0..dot_index].to_string(),
        None => instruction.font.0.to_string()
    };
    let img_name = format!("img{}_{}_{}x{}_{}_{}{}", image_index, character_string, instruction.image_size.width, instruction.image_size.height,instruction.image_color.map(|num| num.to_string()).join("-"),font, instruction.image_format);

    return img_name;
}

