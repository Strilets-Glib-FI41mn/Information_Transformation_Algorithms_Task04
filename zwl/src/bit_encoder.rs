use std::{io::{Read, Write}, ops::Sub};

use bit_writer_reader::{bit_reader::BitReader, bit_writter::BitWriter};

use crate::{dictionary::{Dictionary, FilledBehaviour}, traits::ToBits};

pub struct ZwlBitEncoder<T: TryInto<usize>, I: Read>{
    input: I,
    pub dictionary: Dictionary<T>,
    sequence: Vec<u8>,
    current_symbol: Option<u8>,
    index: Option<T>,
}



impl<T, I> ZwlBitEncoder<T, I>
where 
    T: TryInto<usize, Error: std::fmt::Debug> + TryFrom<usize, Error: std::fmt::Debug> + From<u8> + std::fmt::Debug + PartialOrd + Copy + Sub<T, Output = T> + min_max_traits::Max + ToBits + crate::traits::CustomWriteSize, //+ Add<T, Output = T> 
    I: Read{
pub fn encode_headerless<O: Write>(&mut self, mut output: O) -> std::io::Result<()> {
        let mut writtable = BitWriter::new(&mut output);
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
                            writtable.write_bits(&(t.bits_vec()))?;
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
            writtable.write_bits(&(t.bits_vec()))?;
        }
        Ok(())
    }
    pub fn encode<O: Write>(&mut self, mut output: O) -> std::io::Result<()> {
        Self::write_header(&mut output, &self.dictionary.filled)?;
        self.encode_headerless(output)?;
        Ok(())
    }

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
        let bit_size: u8 = (T::custom_size() as usize).try_into().unwrap();
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