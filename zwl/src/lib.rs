use std::ops::{Add, Sub};
use std::process::Output;
use std::{fs::File, path::Path};
use std::io::prelude::*;
pub mod dictionary;
use dictionary::Dictionary;




impl<T: TryInto<usize> + From<u8>> Dictionary<T>{

}
pub struct ZwlEncoder<T: TryInto<usize>, I: Read>{
    input_file: I,
    pub dictionary: Dictionary<T>,
    sequence: Vec<u8>,
    current_symbol: Option<u8>,
    index: Option<T>,
}

impl<T, I> ZwlEncoder<T, I>
where 
    T: TryInto<usize, Error: std::fmt::Debug> + TryFrom<usize, Error: std::fmt::Debug> + From<u8> + std::fmt::Debug + PartialOrd + Copy + Sub<T, Output = T> + WritableIndex, //+ Add<T, Output = T> 
    I: Read{
pub fn encode_headerless<O: Write>(&mut self, mut output: O) -> std::io::Result<()> {
        let mut buf = [0];
        let mut result = self.input_file.read(&mut buf);
        while let Ok(s) = result && s > 0{
            self.current_symbol = Some(buf[0]);
            println!("cs: {}", buf[0]);
            self.sequence.push(buf[0]);
            if self.sequence.len() == 1{
                self.index = self.dictionary.find(&self.sequence);
                self.index.unwrap();
                //T::from(buf[0]).do_write(& mut output)?;
            }
            else{
                let found = self.dictionary.find(&self.sequence);
                match found{
                    Some(found) => self.index = Some(found),
                    None => {
                        if let Some(t) = self.index{
                            t.do_write(&mut output)?;
                        }
                        let last = self.sequence[self.sequence.len() - 1];
                        //let found_prev = self.dictionary.find(&self.sequence[0..self.sequence.len() - 1]).unwrap();
                        self.dictionary.push(&(last, self.index.unwrap()));
                        self.sequence = vec![last];
                        self.index = self.dictionary.find(&[last]);
                        if let Some(t) = self.index{
                            t.do_write(&mut output)?;
                        }
                    },
                }
            }
            result = self.input_file.read(&mut buf);
        }
        if let Some(t) = self.index{
            t.do_write(&mut output)?;
        }
        Ok(())
    }
    pub fn encode<O: Write>(&mut self, mut output: O) -> std::io::Result<()> {
        Self::write_header(&mut output)?;
        self.encode_headerless(output)?;
        Ok(())
    }
}


impl<T, I> ZwlEncoder<T, I>
where 
    T: TryInto<usize, Error: std::fmt::Debug> + std::fmt::Debug,// + Sub<T, Output = T>,// + WritableIndex, 
    I: Read
{
    pub fn write_header<O>(output: &mut O) -> std::io::Result<()> where O: Write  {
        let bit_size = Self::header_bit_size();
        output.write_all(&bit_size.to_be_bytes())?;
        Ok(())
    }
    pub fn header_bit_size() -> u64 {
        let bit_size: u64 = size_of::<T>() as u64 * 8;
        bit_size
    }
    pub fn new(input_file: I) -> Self{
        let dictionary = Dictionary::default();
        Self{
            input_file,
            dictionary,
            sequence: vec![],
            current_symbol: None,
            index: None
        }
    }
}

pub struct ZwlDecoder<T: TryInto<usize>>{
    input_file: File,
    dictionary: Vec<(u8, Option<T>)>
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
        let dictionary = (0..u8::MAX).map(|byte| (byte, None)).collect();
        Self{
            input_file,
            dictionary
        }
    }
}

pub trait WritableIndex {
    fn do_write<O: Write>(&self, output: &mut O) -> std::io::Result<()>;
}

impl WritableIndex for u16{
    fn do_write<O: Write>(&self, output: &mut O) -> std::io::Result<()> {
        output.write(&self.to_be_bytes())?;
        Ok(())
    }
}

impl WritableIndex for u32{
    fn do_write<O: Write>(&self, output: &mut O) -> std::io::Result<()> {
        output.write(&self.to_be_bytes())?;
        Ok(())
    }
}

impl WritableIndex for u64{
    fn do_write<O: Write>(&self, output: &mut O) -> std::io::Result<()> {
        output.write(&self.to_be_bytes())?;
        Ok(())
    }
}