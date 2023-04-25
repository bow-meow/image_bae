use clap::Parser;
use image_instruction::ImageInstructions;
use rust_embed::RustEmbed;
use args::args::{Cli};
use image_generator::ImageGenerator;
mod image_generator;
mod args;
mod misc;
mod image_instruction;

#[derive(RustEmbed)]
#[folder = "fonts"]
pub struct Fonts;

fn main() {
    ImageGenerator::new(ImageInstructions::build(Cli::parse())).run();
}


