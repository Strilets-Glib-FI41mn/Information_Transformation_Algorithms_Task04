use std::{io::{Read, Write}, ops::Sub};

use bit_writer_reader::bit_writter::BitWriter;

use crate::{dictionary::{Dictionary, FilledBehaviour}, traits::{TrailingOnesR, LeadingZerosR, RequiredBits, ToBits}};

pub struct ZwlBitEncoder<T: TryInto<usize>, I: Read>{
    input: I,
    pub dictionary: Dictionary<T>,
    sequence: Vec<u8>,
    current_symbol: Option<u8>,
    index: Option<T>,
}



impl<T, I> ZwlBitEncoder<T, I>
where 
    T: TryInto<usize, Error: std::fmt::Debug> + TryFrom<usize, Error: std::fmt::Debug> + From<u8> + std::fmt::Debug + PartialOrd + Copy + Sub<T, Output = T> + min_max_traits::Max + ToBits + crate::traits::CustomWriteSize
    + for<'a> TryFrom<&'a [bool], Error: std::fmt::Debug>
    + LeadingZerosR + TrailingOnesR + RequiredBits
    , //+ Add<T, Output = T> 
    I: Read{
pub fn encode_headerless<O: Write>(&mut self, mut output: O) -> std::io::Result<()> {
        let mut writtable = BitWriter::new(&mut output);
        let mut buf = [0; 64];
        let mut result = self.input.read(&mut buf);
        let mut size_req = 9;
        while let Ok(s) = result && s > 0{
            for i in 0..s{
            self.current_symbol = Some(buf[i]);
            self.sequence.push(buf[i]);
                let found = self.dictionary.find(&self.sequence);
                match found{
                    Some(found) => {
                        self.index = Some(found);
                    },
                    None => {
                        if let Some(t) = self.index{
                            // println!("INDEX: {t:?}, {:?}, {:?}, req_t {}", t.bits_vec(), TryInto::<T>::try_into(t.bits_vec().as_slice()), t.required_bits());
                            let mut target = t.bits_vec();
                            if target.len() < size_req{
                                // println!("len:{} ->{size_req}", target.len());
                                let mut summary = vec![false; size_req - target.len()];
                                target.append(&mut summary);
                            }
                            writtable.write_bits(&target)?;
                        }
                        self.dictionary.push(&(self.current_symbol.unwrap(), self.index.unwrap()));
                        let new_required_bits = self.dictionary.required_bits();
                        if size_req != new_required_bits{
                            // println!("{size_req} -> {new_required_bits}");
                            let output = (0..size_req).into_iter().map(|_| true).collect::<Vec<_>>();
                            writtable.write_bits(&output)?;
                            // let transf= T::try_from(output.as_slice()).unwrap();
                            // println!("11111...: {transf:?}, 0s: {}, 1s: {}, {}", transf.leading_zeros(), transf.trailing_ones(), transf.trailing_ones() - size_req);
                            // println!("{size_req} -> {new_required_bits}");
                            size_req = new_required_bits;
                        }
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
        writtable.output.flush()?;
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