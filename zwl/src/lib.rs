use std::{fs::File, path::Path};
use std::io::prelude::*;
pub struct ZwlEncoder<T: TryInto<usize>>{
    input_file: File,
    alphabet: Vec<(u8, Option<T>)>,
    sequence: Vec<u8>,
    current_symbol: Option<u8>,
    index: Option<T>,
}

impl<T: TryInto<usize>> ZwlEncoder<T>{
    pub fn write_header<P>(path: P) -> std::io::Result<()> where P: AsRef<Path>  {
        let mut file = File::create(path)?;
        let bit_size = Self::header_bit_size();
        file.write_all(&bit_size.to_be_bytes())?;
        Ok(())
    }
    pub fn header_bit_size() -> u64 {
        let bit_size: u64 = size_of::<T>() as u64 * 8;
        bit_size
    }
    pub fn new(input_file: File) -> Self{
        let alphabet = (0..u8::MAX).map(|byte| (byte, None)).collect();
        Self{
            input_file,
            alphabet,
            sequence: vec![],
            current_symbol: None,
            index: None
        }
    }
    pub fn encode<O: Write>(&mut self, mut output: O){
        let mut buf = 0;
        let result = self.input_file.read(&mut [buf]);
        while let Ok(s) = result{
            if s > 0{
                self.current_symbol = Some(buf);
                self.sequence.push(buf);
            }
        }
    }
}

pub struct ZwlDecoder<T: TryInto<usize>>{
    input_file: File,
    alphabet: Vec<(u8, Option<T>)>
}

impl<T: TryInto<usize>> ZwlDecoder<T>{
    pub fn write_header<P>(path: P) -> std::io::Result<()> where P: AsRef<Path>  {
        let mut file = File::create(path)?;
        let size = Self::header_size();
        file.write_all(&size.to_be_bytes())?;
        Ok(())
    }
    pub fn header_size() -> u64 {
        let size: u64 = size_of::<T>() as u64 * 8;
        size
    }
    pub fn new(input_file: File) -> Self{
        let alphabet = (0..u8::MAX).map(|byte| (byte, None)).collect();
        Self{
            input_file,
            alphabet
        }
    }
}
