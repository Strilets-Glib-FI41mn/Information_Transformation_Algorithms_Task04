pub mod like_u12;
pub mod like_u16;
pub mod bit_encoder;
pub mod bit_decoder;
pub mod naive_encoder_decoder;
pub mod traits;
pub mod dictionary;
#[cfg(test)]
mod tests {
    use crate::{bit_decoder::ZwlBitDecoder, bit_encoder::ZwlBitEncoder, like_u12::LikeU12, like_u16::LikeU16, naive_encoder_decoder::{ZwlBitDecoder, ZwlBitEncoder}};

    use super::*;
    use std::io;
    #[test]
    fn encoding_decoding() {
        let s = "abacacacab".to_string();
        let cursor = io::Cursor::new(s.as_bytes());
        let mut encoder: ZwlEncoder<u16, io::Cursor<&[u8]>> = ZwlEncoder::<u16, io::Cursor<&[u8]>>::new(cursor, FilledBehaviour::Clear);
        let mut buffer = vec![0u8; s.len() * 10];
        let mut buffer_d = vec![0u8; s.len()];
        assert!(encoder.encode(&mut buffer[..]).is_ok());
        println!("{:?}", buffer);

        println!("-----");
        println!("encoder dict: {:?}", encoder.dictionary.words);
        println!("-----");
        let mut decoder = ZwlDecoder::<u16, _>::new(&buffer[2..], FilledBehaviour::Clear);
        assert!(decoder.decode(&mut buffer_d[..]).is_ok());
        println!("-----");
        println!("decoder dict: {:?}", decoder.dictionary.words);
        println!("-----");
        assert_eq!(String::from_utf8(buffer_d.to_vec()), Ok(s))
    }


    #[test]
    fn encoding_decoding_l_u12() {
        let input = 0b011101111011_u16;
        let double_conv = TryInto::<LikeU12>::try_into
        (crate::traits::ToBits::bits_vec(&LikeU12(input)).as_slice()
        ).unwrap();
        {
            use crate::traits::ToBits;
            use crate::traits::RequiredBits;
            println!("INDEX: {double_conv:?}, {:?}, {:?}, req_t {}", double_conv.bits_vec(), TryInto::<LikeU12>::try_into(double_conv.bits_vec().as_slice()), double_conv.required_bits());
            let input = LikeU12(12);
            println!("INDEX: {input:?}, {:?}, {:?}, req_t {}", input.bits_vec(), TryInto::<LikeU12>::try_into(input.bits_vec().as_slice()), input.required_bits());

            let input = LikeU12(12);
            println!("INDEX: {input:?}, {:?}, {:?}, req_t {}", input.bits_vec(), TryInto::<LikeU12>::try_into(input.bits_vec().as_slice()), input.required_bits());
        }

        println!("from {}, double conv:{double_conv:?}", input);
        println!("LU12");
        let s = "abacacacab".to_string();
        let cursor = io::Cursor::new(s.as_bytes());
        let mut encoder = ZwlBitEncoder::<LikeU12, _>::new(cursor, FilledBehaviour::Clear);
        let mut buffer = vec![0u8; s.len() * 4];
        let mut buffer_d = [0u8; 10];
        assert!(encoder.encode(&mut buffer[..]).is_ok());

        println!("-----");
        println!("encoder dict: {:?}", encoder.dictionary.words);
        println!("-----");

        println!("{:?}", buffer);
        let mut decoder = ZwlBitDecoder::<LikeU12, _>::new(&buffer[2..], FilledBehaviour::Clear);
        assert!(decoder.decode(&mut buffer_d[..]).is_ok());

        println!("-----");
        println!("decoder dict: {:?}", decoder.dictionary.words);
        println!("-----");



        assert_eq!(String::from_utf8(buffer_d.to_vec()), Ok(s));
    }
    
    #[test]
    fn encoding_decoding_l_u12_longer() {
        let s = "The Project Gutenberg eBook of The Ethics of Aristotle
    
This ebook is for the use of anyone anywhere in the United States and
most other parts of the world at no cost and with almost no restrictions
whatsoever. You may copy it, give it away or re-use it under the terms
of the Project Gutenberg License included with this ebook or online
at www.gutenberg.org. If you are not located in the United States,
you will have to check the laws of the country where you are located
before using this eBook.".to_string();
        let cursor = io::Cursor::new(s.as_bytes());
        let mut encoder = ZwlBitEncoder::<LikeU12, _>::new(cursor, FilledBehaviour::Clear);
        
        
        let mut buffer = vec![0u8; s.len() * 4];
        let mut buffer_d = vec![0u8; s.len()];
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

        assert_eq!(String::from_utf8(buffer_d.to_vec()), Ok(s))
    }
    #[test]
    fn longer_text(){
        let s = "The Project Gutenberg eBook of The Ethics of Aristotle
    
This ebook is for the use of anyone anywhere in the United States and
most other parts of the world at no cost and with almost no restrictions
whatsoever. You may copy it, give it away or re-use it under the terms
of the Project Gutenberg License included with this ebook or online
at www.gutenberg.org. If you are not located in the United States,
you will have to check the laws of the country where you are located
before using this eBook.".to_string();


        let cursor = io::Cursor::new(s.as_bytes());
        let mut encoder: ZwlEncoder<u16, io::Cursor<&[u8]>> = ZwlEncoder::<u16, io::Cursor<&[u8]>>::new(cursor, FilledBehaviour::Clear);
        let mut buffer = vec![0u8; s.len() * 4];
        let mut buffer_d = vec![0u8; s.len()];
        assert!(encoder.encode(&mut buffer[..]).is_ok());
        println!("{:?}", buffer);
        let mut decoder = ZwlDecoder::<u16, _>::new(&buffer[2..], FilledBehaviour::Clear);
        assert!(decoder.decode(&mut buffer_d[..]).is_ok());

        assert_eq!(String::from_utf8(buffer_d.to_vec()), Ok(s))
    }

    #[test]
    fn encoding_decoding_l_u16_longer() {
        let s = "The Project Gutenberg eBook of The Ethics of Aristotle
    
This ebook is for the use of anyone anywhere in the United States and
most other parts of the world at no cost and with almost no restrictions
whatsoever. You may copy it, give it away or re-use it under the terms
of the Project Gutenberg License included with this ebook or online
at www.gutenberg.org. If you are not located in the United States,
you will have to check the laws of the country where you are located
before using this eBook.".to_string();
        let cursor = io::Cursor::new(s.as_bytes());
        let mut encoder = ZwlBitEncoder::<LikeU16, _>::new(cursor, FilledBehaviour::Clear);
        
        
        let mut buffer = vec![0u8; s.len() * 4];
        let mut buffer_d = vec![0u8; s.len()];
        assert!(encoder.encode(&mut buffer[..]).is_ok());
        println!("{:?}", buffer);
        let mut decoder = ZwlBitDecoder::<LikeU16, _>::new(&buffer[2..], FilledBehaviour::Clear);
        assert!(decoder.decode(&mut buffer_d[..]).is_ok());

        println!("-----");
        println!("encoder dict: {:?}", encoder.dictionary.words);
        println!("-----");

        println!("-----");
        println!("decoder dict: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()]);
        println!("-----");
        println!("lost: {:?}", &decoder.dictionary.words[0..encoder.dictionary.words.len()] == &encoder.dictionary.words[0..]);

        assert_eq!(String::from_utf8(buffer_d.to_vec()), Ok(s))
    }
}