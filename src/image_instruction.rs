use crate::{misc::{image_size::Size, image_text::TextType, image_fonts::FontType}, args::args::Cli};

pub struct ImageInstructions{
    pub instructions : Vec<ImageInstruction>,
    pub repeat: u32
}

impl ImageInstructions{
    pub fn build(cli: Cli) -> Self{
        let mut instructions : Vec<ImageInstruction> = Vec::new();
        let image_size_collection = cli.image_size.as_ref().unwrap();
        let image_font = cli.image_font.as_ref().unwrap();
        let image_color = cli.image_color.as_ref().unwrap();
        let image_text_color = cli.image_text_color.as_ref().unwrap();
        let image_format = cli.image_format.as_ref().unwrap();
        let image_filesize = cli.image_filesize.as_ref().unwrap();

        let repeat = cli.repeat.to_owned().unwrap();

        //TODO this could be done in the parsing method
        let mut text_len: usize = 0;
        cli.image_text.text.iter().for_each(|(text, text_type)| {
            match text_type{
                TextType::Char => text_len+=text.len(),
                _ => text_len+=1
            }
        } );

        text_len *= repeat as usize;

        let fonts = image_font.get_fonts(text_len);
        let colors = image_color.get_colors(text_len);
        let text_colors = image_text_color.get_colors(text_len);
        let formats = image_format.get_formats(text_len);
        let sizes = image_size_collection.get_sizes(text_len);
        let filesizes = image_filesize.get_filesizes(text_len);

        let mut i: usize = 0;
        let mut repeater = repeat;

        loop{
            for (text, text_type) in cli.image_text.text.iter(){
                match text_type{
                    TextType::Char =>{
                        for c in text.chars(){
                            let character = c.to_string();
                            let size = sizes[i].to_owned(); //not good
                            let font = fonts[i].to_owned();
                            let format = formats[i].to_owned();
                            let color = colors[i].to_owned();
                            let text_color = text_colors[i].to_owned();
                            let filesize = filesizes[i].to_owned();
                            instructions.push(ImageInstruction{ character, image_size: size, font: font, image_format: format, image_color: color, image_text_color: text_color, image_filesize: filesize });
                            i+=1;
                        }
                    },
                    _ =>{
                        let size = sizes[i].to_owned(); //not good
                        let font = fonts[i].to_owned();
                        let format = formats[i].to_owned();
                        let color = colors[i].to_owned();
                        let text_color = text_colors[i].to_owned();
                        let filesize = filesizes[i].to_owned();
                        i+=1;
                        instructions.push(ImageInstruction{ character: text.to_string(), image_size: size, font: font, image_format: format, image_color: color, image_text_color: text_color, image_filesize: filesize });
                    }
                }
            }
            repeater -= 1;
            if repeater == 0{
                break;
            }
        }
        Self{ instructions, repeat }
    }
}

pub struct ImageInstruction{
    pub character: String,
    pub image_size: Size,
    pub font: (String, FontType),// need some font + special font type here
    pub image_format: String,
    pub image_color: [u8; 4],
    pub image_text_color: [u8; 4],
    pub image_filesize: u32
}