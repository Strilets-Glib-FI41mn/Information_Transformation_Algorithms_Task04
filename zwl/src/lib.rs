pub mod like_u12;
pub mod bit_encoder;
pub mod bit_decoder;
pub mod traits;
use std::ops::Sub;
use std::io::prelude::*;
pub mod dictionary;
use dictionary::Dictionary;

use crate::dictionary::FilledBehaviour;
use crate::traits::{ReadableIndex, WritableIndex};

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
        let mut buf = [0; 64];
        let mut result = self.input.read(&mut buf);
        while let Ok(s) = result && s > 0{
            for i in 0..s{
            self.current_symbol = Some(buf[i]);
            self.sequence.push(buf[i]);
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
        Self::write_header(&mut output, &self.dictionary.filled)?;
        self.encode_headerless(output)?;
        Ok(())
    }
}


impl<T, I> ZwlEncoder<T, I>
where 
    T: TryInto<usize, Error: std::fmt::Debug> + std::fmt::Debug,// + Sub<T, Output = T>,// + WritableIndex, 
    I: Read
{
    pub fn write_header<O>(output: &mut O, dictionary_filled: &FilledBehaviour) -> std::io::Result<()> where O: Write  {
        let bit_size = Self::header_bit_size();
        output.write_all(&bit_size.to_be_bytes())?;
        match dictionary_filled{
            FilledBehaviour::Clear => output.write_all(&[0])?,
            FilledBehaviour::Freeze => output.write_all(&[1])?,
        }
        Ok(())
    }
    pub fn header_bit_size() -> u8 {
        let bit_size: u8 = size_of::<T>() as u8 * 8;
        bit_size
    }
    pub fn new(input: I, dictionary_filled: FilledBehaviour) -> Self{
        let mut dictionary = Dictionary::default();
        dictionary.filled= dictionary_filled;
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
    pub fn new(input: I, dictionary_filled: FilledBehaviour) -> Self{
        let mut dictionary = Dictionary::default();
        dictionary.filled = dictionary_filled;
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
        let sequence = vec![self.dictionary[index].0];
        output.write(&sequence)?;
        self.old_index = Some(index);
        self.old_sequence = sequence;
        let mut result = T::read_from(&mut self.input);
        while let Ok((index, size)) = result && size > 0{
            let a = self.dictionary.get_phrase(index);
            match a{
                Some(sequence) => {
                    output.write(&sequence)?;
                    self.dictionary.push(&(sequence[0], self.old_index.unwrap()));
                    self.old_index = Some(index);
                    self.old_sequence = sequence;
                },
                None => {
                    let mut sequence = self.old_sequence.clone();
                    sequence.push(self.old_sequence[0]);
                    output.write(&sequence)?;
                    self.dictionary.push(&(self.old_sequence[0], self.old_index.unwrap()));
                },
            }
            result = T::read_from(&mut self.input);
        }
        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    #[test]
    fn encoding_decoding() {
        let s = "abacacacab".to_string();
        let cursor = io::Cursor::new(s.as_bytes());
        let mut encoder: ZwlEncoder<u16, io::Cursor<&[u8]>> = ZwlEncoder::<u16, io::Cursor<&[u8]>>::new(cursor, FilledBehaviour::Clear);
        let mut buffer = vec![0u8; s.len() * 4];  // A buffer with a capacity of 1024 bytes
        let mut buffer_d = [0u8; 10];  // A buffer with a capacity of 1024 bytes
        assert!(encoder.encode(&mut buffer[..]).is_ok());
        let mut decoder = ZwlDecoder::<u16, _>::new(&buffer[1..], FilledBehaviour::Clear);
        assert!(decoder.decode(&mut buffer_d[..]).is_ok());

        assert_eq!(String::from_utf8(buffer_d.to_vec()), Ok(s))
    }
}