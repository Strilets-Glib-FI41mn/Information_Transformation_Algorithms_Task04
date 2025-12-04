use std::{fmt::Debug, io::{Read, Write}, ops::Sub};

use bit_writer_reader::bit_reader::BitReader;

use crate::{dictionary::{Dictionary, FilledBehaviour}, traits::{CustomWriteSize, TrailingOnesR, RequiredBits}};

pub struct ZwlBitDecoder<T: TryInto<usize>, I: Read>{
    input: I,
    pub dictionary: Dictionary<T>,
    // sequence: Vec<u8>,
    //current_symbol: Option<u8>,
    // index: Option<T>,
    old_sequence: Vec<u8>,
    // old_symbol: Option<u8>,
    old_index: Option<T>,
}

impl<T, I> ZwlBitDecoder<T, I>
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

impl<T, I> ZwlBitDecoder<T, I>
where 
    T: TryInto<usize, Error: std::fmt::Debug> + std::fmt::Debug + for<'a> TryFrom<&'a [bool], Error: Debug> + Default + From<u8> + PartialOrd + Copy + Sub<Output = T> 
        + TryFrom<usize, Error: std::fmt::Debug> + min_max_traits::Max + CustomWriteSize 
        //+ LeadingZerosR 
        + TrailingOnesR,
    I: Read{
    pub fn decode<O: Write>(&mut self, mut output: O) -> std::io::Result<()> {
        let mut readable = BitReader::new(&mut self.input);
        //let size_req = T::custom_size();
        let mut size_req = self.dictionary.required_bits();

        let binding = readable.read_bits(size_req)?;
        let index_v: &[bool] = binding.as_slice();
        let index = T::try_from(index_v).unwrap();
        // println!("TRAILING ONES: {}, index: {index:?}, size_req: {size_req}", index.trailing_ones());
        let sequence = vec![self.dictionary[index].0];

        output.write(&sequence)?;
        self.old_index = Some(index);
        self.old_sequence = sequence;
        let mut result = readable.read_bits(size_req);
        while let Ok(index_v) = result{
            let index = T::try_from(index_v.as_slice()).unwrap();
            if index.trailing_ones() == size_req{
                if T::custom_size() == size_req{
                    size_req = 9;
                }
                else{
                    size_req += 1;
                }
                result = readable.read_bits(size_req);
                continue;
            }
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
                    self.old_index = Some(T::try_from(self.dictionary.len() - 1).unwrap());
                    self.old_sequence = sequence;
                },
            }
            result = readable.read_bits(size_req);
        }
        Ok(())
    }
}