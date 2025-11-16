use zwl_gs::bit_decoder::ZwlBitDecoder;
use zwl_gs::bit_encoder::ZwlBitEncoder;
use zwl_gs::dictionary::FilledBehaviour;
use zwl_gs::like_u12::LikeU12;
use zwl_gs::like_u16::LikeU16;

use clap::Parser;
use serde::Serialize;
use dialoguer::{Confirm, Editor};
use zwl_gs::like_u32::LikeU32;
use zwl_gs::like_u64::LikeU64;

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
#[derive(
    clap::ValueEnum, Clone, Default, Serialize
)]
#[serde(rename_all = "kebab-case")]
enum Encoding{
    #[default]
    U12,
    U16,
    U32,
    U64
}


#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(
    clap::ValueEnum, Clone, Default, Serialize
)]
#[serde(rename_all = "kebab-case")]
enum FilledOption{
    #[default]
    Clear,
    Freeze
}

impl From<FilledOption> for FilledBehaviour{
    fn from(val: FilledOption) -> Self{
        match val{
            FilledOption::Clear => Self::Clear,
            FilledOption::Freeze => Self::Freeze,
        }
    }
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
    #[arg(long, short, default_value_t = FilledOption::Clear, value_enum, help = "Filled behavior of dictionary used in encoding mode")]
    filled: FilledOption,
    #[arg(long, short, default_value_t = Encoding::U12, value_enum, help = "Ecnoding used in encoding mode")]
    encoding: Encoding
}

fn main() -> io::Result<()>{
    // TryInto::<Vec<bool>>::try_into(LikeU12::from(10));
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
                    new_extension.push(".zwl");
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
                        if let Some(rv) = Editor::new().edit(out.to_str().unwrap()).unwrap() {
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
            let output = File::create(output_path)?;
            match cli.encoding{
                Encoding::U12 => {
                    let mut encoder = ZwlBitEncoder::<LikeU12, File>::new(input, cli.filled.into());
                    encoder.encode(output)?;
                },
                Encoding::U16 => {
                    let mut encoder = ZwlBitEncoder::<LikeU16, File>::new(input, cli.filled.into());
                    encoder.encode(output)?;
                },
                Encoding::U32 => {
                    let mut encoder = ZwlBitEncoder::<LikeU32, File>::new(input, cli.filled.into());
                    encoder.encode(output)?;
                },
                Encoding::U64 => {
                    let mut encoder = ZwlBitEncoder::<LikeU64, File>::new(input, cli.filled.into());
                    encoder.encode(output)?;
                },
            }
        }
        Mode::Decode => {
            let input = File::open(input_path)?;
            let decoder = get_decoder(input)?;
            let output = File::create(output_path)?;
            match decoder{
                ZwlDecoderE::DU12(mut zwl_decoder) => zwl_decoder.decode(output)?,
                ZwlDecoderE::DU16(mut zwl_decoder) => zwl_decoder.decode(output)?,
                ZwlDecoderE::DU32(mut zwl_decoder) => zwl_decoder.decode(output)?,
                ZwlDecoderE::DU64(mut zwl_decoder) => zwl_decoder.decode(output)?,
            }
        }
    }
    
    Ok(())
}



pub enum ZwlDecoderE<I: Read>{
    DU12(ZwlBitDecoder<LikeU12, I>),
    DU16(ZwlBitDecoder<LikeU16, I>),
    DU32(ZwlBitDecoder<LikeU32, I>),
    DU64(ZwlBitDecoder<LikeU64, I>)
}

impl<I: Read> From::<ZwlBitDecoder<LikeU12, I>> for ZwlDecoderE<I>{
    fn from(value: ZwlBitDecoder<LikeU12, I>) -> Self {
        Self::DU12(value)
    }
}
impl<I: Read> From::<ZwlBitDecoder<LikeU16, I>> for ZwlDecoderE<I>{
    fn from(value: ZwlBitDecoder<LikeU16, I>) -> Self {
        Self::DU16(value)
    }
}

impl<I: Read> From::<ZwlBitDecoder<LikeU32, I>> for ZwlDecoderE<I>{
    fn from(value: ZwlBitDecoder<LikeU32, I>) -> Self {
        Self::DU32(value)
    }
}
impl<I: Read> From::<ZwlBitDecoder<LikeU64, I>> for ZwlDecoderE<I>{
    fn from(value: ZwlBitDecoder<LikeU64, I>) -> Self {
        Self::DU64(value)
    }
}


pub fn get_decoder<I: Read>(mut file: I) -> std::io::Result<ZwlDecoderE<I>> {
    let mut buffer = [0, 0];
    file.read_exact(&mut buffer)?;
    let index_bit_size = buffer[0];
    let filled_behaviour = buffer[1];
    let filled_behaviour = match filled_behaviour{
        0 => FilledBehaviour::Clear,
        1 => FilledBehaviour::Freeze,
        _ => return Err(std::io::Error::other("Header does not say if dictionary should clear or freeze when it is full"))
    };
    match index_bit_size{
        12 => {
            Ok(ZwlDecoderE::from(ZwlBitDecoder::<LikeU12, I>::new(file, filled_behaviour)))
        }
        16 => {
            Ok(ZwlDecoderE::from(ZwlBitDecoder::<LikeU16, I>::new(file, filled_behaviour)))
        }
        32 => {
            Ok(ZwlDecoderE::from(ZwlBitDecoder::<LikeU32, I>::new(file, filled_behaviour)))
        }
        64 => {
            Ok(ZwlDecoderE::from(ZwlBitDecoder::<LikeU64, I>::new(file, filled_behaviour)))
        }
        _ =>{
            Err(std::io::Error::other("Only LikeU12, u16, u32 and u64 indexes were implemented"))
        }
    }
}