use zwl_gs::{self, ZwlDecoder};

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
    println!("u8 size: {}", zwl_gs::ZwlEncoder::<u8>::header_bit_size());
    println!("u16 size: {}", zwl_gs::ZwlEncoder::<u16>::header_bit_size());
    println!("u32 size: {}", zwl_gs::ZwlEncoder::<u32>::header_bit_size());
    println!("u64 size: {}", zwl_gs::ZwlEncoder::<u64>::header_bit_size());
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
            todo!("encoding");
            //encode(&input_path, &output_path, !cli.frequencyless)?;
        }
        Mode::Decode => {
            todo!("decoding");
            //decode(&input_path, &output_path)?;
        }
    }
    
    Ok(())
}



pub enum ZwlDecoderE{
    DU8(ZwlDecoder<u8>),
    DU16(ZwlDecoder<u16>),
    DU32(ZwlDecoder<u32>),
    DU64(ZwlDecoder<u64>)
}
impl From::<ZwlDecoder<u8>> for ZwlDecoderE{
    fn from(value: ZwlDecoder<u8>) -> Self {
        Self::DU8(value)
    }
}

impl From::<ZwlDecoder<u16>> for ZwlDecoderE{
    fn from(value: ZwlDecoder<u16>) -> Self {
        Self::DU16(value)
    }
}

impl From::<ZwlDecoder<u32>> for ZwlDecoderE{
    fn from(value: ZwlDecoder<u32>) -> Self {
        Self::DU32(value)
    }
}
impl From::<ZwlDecoder<u64>> for ZwlDecoderE{
    fn from(value: ZwlDecoder<u64>) -> Self {
        Self::DU64(value)
    }
}


pub fn get_decoder(mut file: File) -> Option<ZwlDecoderE> {
    let mut buff = [4; 8];
    file.read_exact(&mut buff);
    let bit_size = u64::from_be_bytes(buff);
    match bit_size{
        8 =>{
            Some(ZwlDecoderE::from(zwl_gs::ZwlDecoder::<u8>::new(file)))
        }
        16 => {
            Some(ZwlDecoderE::from(zwl_gs::ZwlDecoder::<u16>::new(file)))
        }
        32 => {
            Some(ZwlDecoderE::from(zwl_gs::ZwlDecoder::<u32>::new(file)))
        }
        64 => {
            Some(ZwlDecoderE::from(zwl_gs::ZwlDecoder::<u64>::new(file)))
        }
        _ =>{
            todo!("Only u8, u16, u32 and u64 indexes were implemented")
        }
    }
}