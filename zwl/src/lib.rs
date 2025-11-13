use std::ops::Sub;
use std::{fs::File, path::Path};
use std::io::prelude::*;
pub mod dictionary;
use dictionary::Dictionary;

pub struct ZwlEncoder<T: TryInto<usize>, I: Read>{
    input: I,
    pub dictionary: Dictionary<T>,
    sequence: Vec<u8>,
    current_symbol: Option<u8>,
    index: Option<T>,
}

impl<T, I> ZwlEncoder<T, I>
where 
    T: TryInto<usize, Error: std::fmt::Debug> + TryFrom<usize, Error: std::fmt::Debug> + From<u8> + std::fmt::Debug + PartialOrd + Copy + Sub<T, Output = T> + WritableIndex + min_max_traits::Max, //+ Add<T, Output = T> 
    I: Read{
pub fn encode_headerless<O: Write>(&mut self, mut output: O) -> std::io::Result<()> {
        let mut buf = [0];
        let mut result = self.input.read(&mut buf);
        while let Ok(s) = result && s > 0{
            self.current_symbol = Some(buf[0]);
            // println!("cs: {}", buf[0]);
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
                        //let found_prev = self.dictionary.find(&self.sequence[0..self.sequence.len() - 1]).unwrap();
                        self.dictionary.push(&(self.current_symbol.unwrap(), self.index.unwrap()));
                        self.sequence = vec![self.current_symbol.unwrap()];
                        self.index = self.dictionary.find(&[self.current_symbol.unwrap()]);
                    },
                }
            }
            result = self.input.read(&mut buf);
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
    pub fn header_bit_size() -> u8 {
        let bit_size: u8 = size_of::<T>() as u8 * 8;
        bit_size
    }
    pub fn new(input: I) -> Self{
        let dictionary = Dictionary::default();
        Self{
            input,
            dictionary,
            sequence: vec![],
            current_symbol: None,
            index: None
        }
    }
}


pub struct ZwlDecoder<T: TryInto<usize>, I: Read>{
    input: I,
    pub dictionary: Dictionary<T>,
    // sequence: Vec<u8>,
    //current_symbol: Option<u8>,
    // index: Option<T>,
    old_sequence: Vec<u8>,
    // old_symbol: Option<u8>,
    old_index: Option<T>,
}

impl<T, I> ZwlDecoder<T, I>
where 
    T: TryInto<usize, Error: std::fmt::Debug> + std::fmt::Debug,
    I: Read{
    pub fn write_header<P>(path: P) -> std::io::Result<()> where P: AsRef<Path>  {
        let mut file = File::create(path)?;
        let size = Self::header_size();
        file.write_all(&size.to_be_bytes())?;
        Ok(())
    }
    pub fn header_size() -> u8 {
        let size: u8 = size_of::<T>() as u8 * 8;
        size
    }
    pub fn new(input: I) -> Self{
        let dictionary = Dictionary::default();
        Self{
            input,
            dictionary,
            // sequence: vec![],
            // current_symbol: None,
            // index: None,
            old_sequence: vec![],
            old_index: None,
            // old_symbol: None,
        }
    }
}

impl<T, I> ZwlDecoder<T, I>
where 
    T: TryInto<usize, Error: std::fmt::Debug> + std::fmt::Debug + ReadableIndex + Default + From<u8> + PartialOrd + Copy + Sub<Output = T> + TryFrom<usize, Error: std::fmt::Debug> + min_max_traits::Max,
    I: Read{
    pub fn decode<O: Write>(&mut self, mut output: O) -> std::io::Result<()> {
        // self.index = Some(T::read_from(&mut self.input)?);
        let (index, size) = T::read_from(&mut self.input)?;
        if size == 0{
            return Ok(());
        }
        // println!("first index: {:?}", &index);
        let sequence = vec![self.dictionary[index].0];
        println!("found sequence: {:?}", sequence);
        output.write(&sequence)?;
        self.old_index = Some(index);
        self.old_sequence = sequence;
        let mut result = T::read_from(&mut self.input);
        while let Ok((index, size)) = result && size > 0{
            let a = self.dictionary.get_phrase(index);
            match a{
                Some(sequence) => {
                    println!("found sequence: {:?}", sequence);
                    output.write(&sequence)?;
                    self.dictionary.push(&(sequence[0], self.old_index.unwrap()));
                    println!("{:?}",self.dictionary.words.last());
                    self.old_index = Some(index);
                    self.old_sequence = sequence;
                },
                None => {
                    let mut sequence = self.old_sequence.clone();
                    sequence.push(self.old_sequence[0]);
                    println!("sequence: {:?}", sequence);
                    output.write(&sequence)?;
                    self.dictionary.push(&(self.old_sequence[0], self.old_index.unwrap()));
                    println!("{:?}",self.dictionary.words.last());
                },
            }
            result = T::read_from(&mut self.input);
            // println!("{:?}", result);
        }
        Ok(())
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

pub trait ReadableIndex  {
    fn read_from<I: Read>(input: &mut I) -> std::io::Result<(Self, usize)> where Self: Sized;
}


impl ReadableIndex for u16{
    fn read_from<I: Read>(input: &mut I) -> std::io::Result<(Self, usize)>{
        let mut buff = [0; 2];
        let size = input.read(&mut buff)?;
        Ok((Self::from_be_bytes(buff), size))
    }
}

impl ReadableIndex for u32{
    fn read_from<I: Read>(input: &mut I) -> std::io::Result<(Self, usize)>{
    let mut buff = [0; 4];
        let size = input.read(&mut buff)?;
        Ok((Self::from_be_bytes(buff),size))
    }
}

impl ReadableIndex for u64{
    fn read_from<I: Read>(input: &mut I) -> std::io::Result<(Self, usize)>{
        let mut buff = [0; 8];
        let size = input.read(&mut buff)?;
        Ok((Self::from_be_bytes(buff),size))
    }
}