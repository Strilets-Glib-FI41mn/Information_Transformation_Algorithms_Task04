pub mod like_u12;
pub mod like_u16;
pub mod like_u32;
pub mod like_u64;
pub mod bit_encoder;
pub mod bit_decoder;
pub mod traits;
pub mod dictionary;
#[cfg(test)]
mod tests {
    use crate::{bit_decoder::ZwlBitDecoder, bit_encoder::ZwlBitEncoder, like_u12::LikeU12, like_u16::LikeU16, like_u32::LikeU32, like_u64::LikeU64, dictionary::FilledBehaviour};

    use super::*;
    use std::io;
    
    const PREAMBLE: &str =  "The Project Gutenberg eBook of The Ethics of Aristotle
    
This ebook is for the use of anyone anywhere in the United States and
most other parts of the world at no cost and with almost no restrictions
whatsoever. You may copy it, give it away or re-use it under the terms
of the Project Gutenberg License included with this ebook or online
at www.gutenberg.org. If you are not located in the United States,
you will have to check the laws of the country where you are located
before using this eBook.";
    #[test]
    fn encoding_decoding_l_u12() {
        let cursor = io::Cursor::new(PREAMBLE.as_bytes());
        let mut encoder = ZwlBitEncoder::<LikeU12, _>::new(cursor, FilledBehaviour::Clear);
        
        
        let mut buffer = vec![0u8; PREAMBLE.len() * 4];
        let mut buffer_d = vec![0u8; PREAMBLE.len()];
        assert!(encoder.encode(&mut buffer[..]).is_ok());
        println!("{:?}", buffer);
        let mut decoder = ZwlBitDecoder::<LikeU12, _>::new(&buffer[2..], FilledBehaviour::Clear);
        assert!(decoder.decode(&mut buffer_d[..]).is_ok());

        println!("-----");
        println!("encoder dict: {:?}", encoder.dictionary.words);
        println!("-----");

        println!("-----");
        println!("decoder dict: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()]);
        println!("-----");
        println!("lost: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()] == &encoder.dictionary.words[0..]);

        assert_eq!(str::from_utf8(&buffer_d.to_vec()), Ok(PREAMBLE))
    }

    #[test]
    fn encoding_decoding_l_u12_headless() {
        let cursor = io::Cursor::new(PREAMBLE.as_bytes());
        let mut encoder = ZwlBitEncoder::<LikeU12, _>::new(cursor, FilledBehaviour::Clear);
        
        
        // let mut buffer = vec![0u8; PREAMBLE.len() * 4];
        // let mut buffer_d = vec![0u8; PREAMBLE.len()];
        let mut buffer = vec![];
        let mut buffer_d = vec![];
        let mut cursor_writer = std::io::Cursor::new(&mut buffer);
        assert!(encoder.encode_headerless(&mut cursor_writer).is_ok());
        println!("{:?}", buffer);
        let mut cursor = std::io::Cursor::new(&buffer);
        let mut decoder = ZwlBitDecoder::<LikeU12, _>::new(&mut cursor, FilledBehaviour::Clear);
        let mut cursor_writer = std::io::Cursor::new(&mut buffer_d);
        assert!(decoder.decode(&mut cursor_writer).is_ok());

        println!("-----");
        println!("encoder dict: {:?}", encoder.dictionary.words);
        println!("-----");

        println!("-----");
        // println!("decoder dict: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()]);
        println!("-----");
        // println!("lost: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()] == &encoder.dictionary.words[0..]);

        assert_eq!(PREAMBLE.as_bytes(), &buffer_d.to_vec());
        assert_eq!(Ok(PREAMBLE), str::from_utf8(&buffer_d.to_vec()));
    }
    #[test]
    fn longer_text(){
        let cursor = io::Cursor::new(PREAMBLE.as_bytes());
        let mut encoder: ZwlBitEncoder<LikeU16, io::Cursor<&[u8]>> = ZwlBitEncoder::<LikeU16, io::Cursor<&[u8]>>::new(cursor, FilledBehaviour::Clear);
        let mut buffer = vec![0u8; PREAMBLE.len() * 4];
        let mut buffer_d = vec![0u8; PREAMBLE.len()];
        assert!(encoder.encode(&mut buffer[..]).is_ok());
        println!("{:?}", buffer);
        let mut decoder = ZwlBitDecoder::<LikeU16, _>::new(&buffer[2..], FilledBehaviour::Clear);
        assert!(decoder.decode(&mut buffer_d[..]).is_ok());

        assert_eq!(Ok(PREAMBLE), str::from_utf8(&buffer_d.to_vec()))
    }

    #[test]
    fn encoding_decoding_l_u16() {
        let cursor = io::Cursor::new(PREAMBLE.as_bytes());
        let mut encoder = ZwlBitEncoder::<LikeU16, _>::new(cursor, FilledBehaviour::Clear);
        
        
        let mut buffer = vec![0u8; PREAMBLE.len() * 4];
        let mut buffer_d = vec![0u8; PREAMBLE.len()];
        assert!(encoder.encode(&mut buffer[..]).is_ok());
        println!("{:?}", buffer);
        let mut decoder = ZwlBitDecoder::<LikeU16, _>::new(&buffer[2..], FilledBehaviour::Clear);
        assert!(decoder.decode(&mut buffer_d[..]).is_ok());

        println!("-----");
        println!("encoder dict: {:?}", encoder.dictionary.words);
        println!("-----");

        println!("-----");
        // println!("decoder dict: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()]);
        println!("-----");
        // println!("lost: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()] == &encoder.dictionary.words[0..]);

        assert_eq!(str::from_utf8(&buffer_d.to_vec()), Ok(PREAMBLE))
    }
    #[test]
    fn encoding_decoding_l_u32() {
        let cursor = io::Cursor::new(PREAMBLE.as_bytes());
        let mut encoder = ZwlBitEncoder::<LikeU32, _>::new(cursor, FilledBehaviour::Clear);
        
        
        let mut buffer = vec![0u8; PREAMBLE.len() * 4];
        let mut buffer_d = vec![0u8; PREAMBLE.len()];
        assert!(encoder.encode(&mut buffer[..]).is_ok());
        println!("{:?}", buffer);
        let mut decoder = ZwlBitDecoder::<LikeU32, _>::new(&buffer[2..], FilledBehaviour::Clear);
        assert!(decoder.decode(&mut buffer_d[..]).is_ok());

        println!("-----");
        println!("encoder dict: {:?}", encoder.dictionary.words);
        println!("-----");

        println!("-----");
        println!("decoder dict: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()]);
        println!("-----");
        println!("lost: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()] == &encoder.dictionary.words[0..]);

        assert_eq!(str::from_utf8(&buffer_d.to_vec()), Ok(PREAMBLE))
    }
    #[test]
    fn encoding_decoding_l_u64() {
        let cursor = io::Cursor::new(PREAMBLE.as_bytes());
        let mut encoder = ZwlBitEncoder::<LikeU64, _>::new(cursor, FilledBehaviour::Clear);
        
        
        let mut buffer = vec![0u8; PREAMBLE.len() * 4];
        let mut buffer_d = vec![0u8; PREAMBLE.len()];
        assert!(encoder.encode(&mut buffer[..]).is_ok());
        println!("{:?}", buffer);
        let mut decoder = ZwlBitDecoder::<LikeU64, _>::new(&buffer[2..], FilledBehaviour::Clear);
        assert!(decoder.decode(&mut buffer_d[..]).is_ok());

        println!("-----");
        println!("encoder dict: {:?}", encoder.dictionary.words);
        println!("-----");

        println!("-----");
        println!("decoder dict: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()]);
        println!("-----");
        println!("lost: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()] == &encoder.dictionary.words[0..]);

        assert_eq!(str::from_utf8(&buffer_d.to_vec()), Ok(PREAMBLE))
    }
}