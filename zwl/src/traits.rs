use std::io::{Read, Write};

pub trait WritableIndex {
    fn do_write<O: Write>(&self, output: &mut O) -> std::io::Result<()>;
}

pub trait ReadableIndex  {
    fn read_from<I: Read>(input: &mut I) -> std::io::Result<(Self, usize)> where Self: Sized;
}

pub trait ToBits{
    fn bits_vec(&self) -> Vec<bool>;
}

pub trait CustomWriteSize{
    fn custom_size() -> usize;
}