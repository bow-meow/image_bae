pub mod args{
    use clap::{Parser};
    use crate::misc::image_filesize::{parse_filesize, ImageFileSize};
    use crate::misc::image_size::{parse_size, ImageSizeCollection};
    use crate::misc::image_fonts::{ImageFont, parse_fonts};
    use crate::misc::image_format::{ImageFormat, parse_formats};
    use crate::misc::image_color::{ImageColor, parse_colors};
    use crate::misc::image_text::{ImageText, parse_text};

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub struct Cli {
        /// Characters to add to images
        #[arg(short = 't', long = "text", value_name = "TEXT", default_value = "[1..100]",  value_parser = parse_text)]
        pub image_text : ImageText,

        /// Optional size of images to generate.
        #[arg(short = 's', long = "size", value_name = "SIZE", default_value = "500", value_parser = parse_size)]
        pub image_size: Option<ImageSizeCollection>,

        /// Optional font/s to use.
        #[arg(long = "font", value_name = "FONT", default_value = "comicsans.ttf", value_parser = parse_fonts)]
        pub image_font: Option<ImageFont>,

        /// Optional color of image.
        #[arg(long = "color", value_name = "COLOR", default_value = "white", value_parser = parse_colors)]
        pub image_color: Option<ImageColor>,

        /// Optional text color.
        #[arg(long = "textc", value_name = "TEXT-COLOR", default_value = "none", value_parser = parse_colors)]
        pub image_text_color: Option<ImageColor>,

        /// Optional format/s of images. currently supported: .tif, .jpg, .png
        #[arg(short = 'f', long = "format", value_name = "FORMAT", default_value = ".jpg", value_parser = parse_formats)]
        pub image_format: Option<ImageFormat>,
        /// Optional repeater. Allows text to be repeated n times
        #[arg(short = 'r', long = "repeat", value_name = "REPEAT", default_value = "0", value_parser = parse_repeater)]
        pub repeat: Option<u32>,
        /// Optional filesize. Makes sure all files are at least the size specified in MB
        #[arg(long = "filesize", value_name = "FILESIZE", default_value = "0", value_parser = parse_filesize)]
        pub image_filesize: Option<ImageFileSize>
    }

    fn parse_repeater(s: &str) -> Result<u32, String>{
        match s.parse::<u32>(){
            Ok(num) => Ok(num + 1),
            Err(_e) => Err("Value was not the correct format".to_owned())
        }
    }
}