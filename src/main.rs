use zwl_gs::{self, ZwlDecoder, ZwlEncoder};

use clap::Parser;
use serde::Serialize;
use dialoguer::{Confirm, Editor};

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(
    clap::ValueEnum, Clone, Default, Serialize
)]
#[serde(rename_all = "kebab-case")]
enum Mode{
    #[default]
    Encode,
    Decode
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    input_file: PathBuf,
    output_file: Option<PathBuf>,
    #[arg(long, short, default_value_t = Mode::Encode, value_enum)]
    mode: Mode,
    #[arg(long, short, default_value_t = false)]
    overwrite: bool,
    #[arg(long, short, default_value_t = false)]
    frequencyless: bool
}

fn main() -> io::Result<()>{
    // println!("u8 size: {}", zwl_gs::ZwlEncoder::<u8, File>::header_bit_size());
    // println!("u16 size: {}", zwl_gs::ZwlEncoder::<u16, File>::header_bit_size());
    // println!("u32 size: {}", zwl_gs::ZwlEncoder::<u32, File>::header_bit_size());
    // println!("u64 size: {}", zwl_gs::ZwlEncoder::<u64, File>::header_bit_size());
    //let s = "This is a test string for encoding for the sake of checking it works".to_string();
    // let s = "tested word just in case... ...".to_string();

    let cli = Cli::parse();
    #[cfg(debug_assertions)]
    println!("{:?}", cli);
    let input_path = cli.input_file.clone();

    let output_path = match cli.output_file{
        Some(output) =>{
            output
        }
        None =>{
            let mut out = input_path.clone();
            match cli.mode{
                Mode::Encode => {
                    let mut new_extension = out.extension().map(|e| e.to_os_string()).unwrap_or_default();
                    new_extension.push(".huffman");
                    out.set_extension(new_extension);
                    out
                }
                Mode::Decode => {
                    out.set_extension("");

                    let confirmation = Confirm::new()
                        .with_prompt(format!("Should the name of new file be {:?}", &out))
                        .interact()
                        .unwrap();
                    if !confirmation{
                        if let Some(rv) = Editor::new().edit(&format!("{}", &out.to_str().unwrap()) ).unwrap() {
                            println!("The file will become:");
                            println!("{}", rv);
                            out = rv.into();
                        } else {
                            println!("No name for the output file found! Exiting");
                            return Ok(());
                        }
                    }
                    out
                }
            }
        }
    };
    if output_path.exists() && !cli.overwrite{
        let confirmation = Confirm::new()
                .with_prompt(format!("File {:?} already exists. Do you want to replace it?", output_path))
                .interact().unwrap();
        if !confirmation{
            println!("Canceled writting into existing file");
            return Ok(());
        }
    }
    match cli.mode{
        Mode::Encode => {
            let input = File::open(input_path)?;
            let mut encoder = ZwlEncoder::<u16, File>::new(input);
            let output = File::create(output_path)?;
            encoder.encode(output)?;
            //encode(&input_path, &output_path, !cli.frequencyless)?;
        }
        Mode::Decode => {
            let input = File::open(input_path)?;
            let decoder = get_decoder(input)?;
            let output = File::create(output_path)?;
            match decoder{
                ZwlDecoderE::DU16(mut zwl_decoder) => zwl_decoder.decode(output)?,
                ZwlDecoderE::DU32(mut zwl_decoder) => zwl_decoder.decode(output)?,
                ZwlDecoderE::DU64(mut zwl_decoder) => zwl_decoder.decode(output)?,
            }
        }
    }
    
    Ok(())
}



pub enum ZwlDecoderE<I: Read>{
    DU16(ZwlDecoder<u16, I>),
    DU32(ZwlDecoder<u32, I>),
    DU64(ZwlDecoder<u64, I>)
}

impl<I: Read> From::<ZwlDecoder<u16, I>> for ZwlDecoderE<I>{
    fn from(value: ZwlDecoder<u16, I>) -> Self {
        Self::DU16(value)
    }
}

impl<I: Read> From::<ZwlDecoder<u32, I>> for ZwlDecoderE<I>{
    fn from(value: ZwlDecoder<u32, I>) -> Self {
        Self::DU32(value)
    }
}
impl<I: Read> From::<ZwlDecoder<u64, I>> for ZwlDecoderE<I>{
    fn from(value: ZwlDecoder<u64, I>) -> Self {
        Self::DU64(value)
    }
}


pub fn get_decoder<I: Read>(mut file: I) -> std::io::Result<ZwlDecoderE<I>> {
    let mut buff = [0];
    file.read_exact(&mut buff)?;
    match buff[0]{
        16 => {
            Ok(ZwlDecoderE::from(zwl_gs::ZwlDecoder::<u16, I>::new(file)))
        }
        32 => {
            Ok(ZwlDecoderE::from(zwl_gs::ZwlDecoder::<u32, I>::new(file)))
        }
        64 => {
            Ok(ZwlDecoderE::from(zwl_gs::ZwlDecoder::<u64, I>::new(file)))
        }
        _ =>{
            Err(std::io::Error::other("Only u8, u16, u32 and u64 indexes were implemented"))
        }
    }
}