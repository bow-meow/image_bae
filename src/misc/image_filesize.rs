use std::str::FromStr;
use rand::{self, Rng};
use super::param_value_parser;
use super::param_types_enum::ParamType;

#[derive(Clone)]
pub struct ImageFileSize{
    filesizes: Vec<u32>,
    rand: bool
}
impl ImageFileSize{
    pub fn get_filesizes(&self, numof_sizes: usize) -> Vec<u32>{
        let mut filesize_list: Vec<u32> = Vec::new();
        if self.rand{
            let mut rng = rand::thread_rng();

            if self.filesizes.len() > 0{
                for _ in 0..numof_sizes{
                    let idx = rng.gen_range(0..self.filesizes.len());
                    let filesize = self.filesizes[idx].clone();
                    filesize_list.push(filesize);
                }

            }
            else{
                for _ in 0..numof_sizes{
                    filesize_list.push(rng.gen_range(100..4000));
                }
            }
        }
        else{
            for _ in 0..numof_sizes{
                for file_size in self.filesizes.iter(){
                    filesize_list.push(*file_size);
                }
            }
        }
        filesize_list
    }
}




pub fn parse_filesize(s: &str) -> Result<ImageFileSize, String> {
    match s.parse::<ImageFileSize>(){
        Ok(filesize) => Ok(filesize),
        Err(e) => Err(e.to_string())
    }
}

#[derive(Clone)]
pub struct ParseFileSizeErr;

impl std::fmt::Display for ParseFileSizeErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value was not the correct format")
    }
}

impl FromStr for ImageFileSize{
    type Err = ParseFileSizeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match param_value_parser::parse_param_value(&s, ParamType::ImageFileSize){
            Ok((filesizes, rand)) => {
                let filesizes = filesizes.iter().map(|f| f.parse::<u32>().unwrap()).collect::<Vec<u32>>();

                for filesize in filesizes.iter(){
                    if *filesize > 4000{
                        return Err(ParseFileSizeErr);
                    }
                }

                Ok(Self { filesizes, rand })
            },
            Err(_e) => return Err(ParseFileSizeErr)
        }
    }
}