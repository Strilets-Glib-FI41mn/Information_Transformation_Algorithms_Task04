use std::ops::{Index, IndexMut, Sub};


pub struct Dictionary<T>{
    pub alphabet:[(u8, Option<T>); 256],
    pub words: Vec<(u8, Option<T>)>
}

impl<T> Dictionary<T>{
    pub fn len(&self) -> usize{
        self.alphabet.len() + self.words.len()
    }
}
impl<T: Copy + TryInto<usize, Error: std::fmt::Debug> + min_max_traits::Max> Dictionary<T>{
    pub fn push(&mut self, word: &(u8, T)){
        if self.len() >= (T::MAX).try_into().unwrap(){
            self.words = vec![];
        }
        self.words.push((word.0, Some(word.1)));
    }
}
impl<T: From<u8> + PartialOrd + Copy + Sub<T, Output = T> + TryInto<usize, Error: std::fmt::Debug> + TryFrom<usize, Error: std::fmt::Debug> + std::fmt::Debug> Dictionary<T>{
    pub fn find(&self, searched: &[u8]) -> Option<T>{
        let sequence_len = searched.len();
        if sequence_len == 1{
            if searched[0] == self[searched[0]].0{
                return Some(searched[0].into());
            }
            println!("weird");
        }
        else{
            //println!("Searching through the words: {:?}", self.words);
            let mut depth = 0;
            let mut candidates = vec![];
            for j in u8::MAX as usize .. (self.len()){
                if searched[sequence_len - depth - 1] == self[j].0{
                    candidates.push(j);
                }
            }
            depth += 1;
            //println!("depth: {}", depth);
            while candidates.len() > 0 && depth < sequence_len{
                let mut remained = vec![];
                for candidate in &candidates{
                    if let Some(candidate_point) = &self[*candidate].1{
                        if searched[sequence_len - depth - 1] == self[*candidate_point].0{
                            remained.push(*candidate);
                        }
                    }
                }
                candidates = remained;
                depth += 1;
                // println!("depth: {}", depth);
            }

        if candidates.len() > 0{
            // println!("c: {:?}, conv: {:?}", candidates, TryInto::<T>::try_into(candidates[0]));
            //return Some(TryInto::<T>::try_into(candidates[0]).unwrap() + T::from(u8::MAX));
            return Some(TryInto::<T>::try_into(candidates[0]).unwrap());
        }
        }
        // println!("None found");
        None
    }
}


impl<T> Dictionary<T>{
    pub fn get<G>(&self, index: G) -> Option<&(u8, Option<T>)>
    where G: From<u8> + PartialOrd + Copy + Sub<G, Output = G> + TryInto<usize, Error: std::fmt::Debug>{
        if index <= G::from(u8::MAX){
            match index.try_into(){
                Ok(u) => return self.alphabet.get(u),
                Err(_) => return None
            }
        }
        match (index - G::from(u8::MAX)).try_into(){
            Ok(u) => return self.words.get(u),
            Err(_) => return None
        }
    }
}


impl<T: std::fmt::Debug> Default for Dictionary<T>{
    fn default() -> Self {
        let alphabet: Vec<_> = (0..=u8::MAX).map(|byte| (byte, None)).collect();
        let alphabet = alphabet.try_into().unwrap();
        Self { alphabet, words: vec![] }
    }
}


impl<T, G: From<u8> + PartialOrd + Copy + Sub<G, Output = G> + TryInto<usize, Error: std::fmt::Debug>> Index<G> for Dictionary<T>
{
    type Output = (u8, Option<T>);

    fn index(&self, index: G) -> &Self::Output {
        if index <= G::from(u8::MAX){
            TryInto::<usize>::try_into(index).unwrap();
            return &self.alphabet[index.try_into().unwrap()]
        }else{
            &self.words[(index - G::from(u8::MAX)).try_into().unwrap()  - 1]
        }
    }
}

impl<T, G: From<u8> + PartialOrd + Copy + Sub<G, Output = G> + TryInto<usize, Error: std::fmt::Debug>> IndexMut<G> for Dictionary<T>
{
    fn index_mut(&mut self, index: G) -> &mut Self::Output {
        if index < G::from(u8::MAX){
            TryInto::<usize>::try_into(index).unwrap();
            return &mut self.alphabet[index.try_into().unwrap()]
        }else{
            &mut self.words[(index - G::from(u8::MAX)).try_into().unwrap()]
        }
    }
}