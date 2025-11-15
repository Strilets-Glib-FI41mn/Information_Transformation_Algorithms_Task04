use std::ops::{Index, IndexMut, Sub};
use std::collections::VecDeque;

use crate::traits::RequiredBits;
pub enum FilledBehaviour{
    Clear,
    Freeze
}

pub struct Dictionary<T>{
    pub alphabet:[(u8, Option<T>); 256],
    pub words: Vec<(u8, Option<T>)>,
    pub filled: FilledBehaviour
}

impl<T> Dictionary<T>{
    pub fn len(&self) -> usize{
        self.alphabet.len() + self.words.len()
    }
}
impl<T> RequiredBits for Dictionary<T>{
    fn required_bits(&self) -> usize{
        std::mem::size_of::<usize>() * 8 - self.len().leading_zeros() as usize
    }
}
impl<T: Copy + TryInto<usize, Error: std::fmt::Debug> + min_max_traits::Max> Dictionary<T>{
    pub fn push(&mut self, word: &(u8, T)){
        if self.len() >= (T::MAX).try_into().unwrap(){
            // println!("Reached max!");
            match self.filled{
                FilledBehaviour::Freeze => return,
                FilledBehaviour::Clear => self.words = vec![],
            }
            return;
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
            return None;
        }
        else{
            let mut candidates = vec![];
            for j in (u8::MAX - 1) as usize .. (self.len()){
                if searched[sequence_len - 1] == self[j].0 && let Some(next) = self[j].1{
                    candidates.push((j, next));
                }
            }
            let candidates: Vec<_> = candidates.iter().filter(|candidate|{
                let mut current = searched[sequence_len - 1];
                let mut current_values = vec![current];
                let mut next = Some(candidate.1);
                let mut cand_depth = 1;
                while next.is_some(){
                    (current, next) = self[next.unwrap()];
                    if current != searched[sequence_len - 1 - cand_depth]{
                        return false;
                    }
                    current_values.insert(0, current);
                    if current_values == searched{
                        return next == None;
                    }
                    if cand_depth >= sequence_len{
                        return false;
                    }
                    cand_depth += 1;
                }
                false
            }).map(|c| c.0).collect();
        if candidates.len() == 1{
            match TryInto::<T>::try_into(candidates[0]){
                Ok(result) => {
                    return Some(result);
                },
                Err(_) => {
                    println!("candidate is: {:?}", candidates[0]);
                },
            }
            return Some(TryInto::<T>::try_into(candidates[0]).unwrap());
        }
        else if candidates.len() > 1{
            panic!("search should not have several candidates");
        }
        }
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
        match (index - G::from(u8::MAX) - G::from(1)).try_into(){
            Ok(u) => return self.words.get(u),
            Err(_) => return None
        }
    }
}
impl<T: From<u8> + PartialOrd + Copy + Sub<T, Output = T> + TryInto<usize, Error: std::fmt::Debug> + TryFrom<usize, Error: std::fmt::Debug> + std::fmt::Debug> Dictionary<T>{
    pub fn get_phrase<G>(&self, index: G) -> Option<Vec<u8>>
    where G: From<u8> + PartialOrd + Copy + Sub<G, Output = G> + TryInto<usize, Error: std::fmt::Debug>{
        if index < G::from(u8::MAX){
            match index.try_into(){
                Ok(u) => return Some(vec![self.alphabet.get(u)?.0]),
                Err(_) => return None
            }
        }
        match (index - G::from(u8::MAX) - G::from(1)).try_into(){
            Ok(u) => {
                let phrase_end = self.words.get(u)?;
                let mut output = VecDeque::new();
                output.push_front(phrase_end.0);
                let mut other_index = phrase_end.1;
                while let Some(index) = other_index{
                    match self.get(index){
                        Some(subphrase) => {
                            output.push_front(subphrase.0);
                            //output.push(subphrase.0);
                            other_index = subphrase.1;
                        },
                        None => break,
                    }
                }
                Some(output.into())
            },
            Err(_) => return None
        }
    }
}


impl<T: std::fmt::Debug> Default for Dictionary<T>{
    fn default() -> Self {
        let alphabet: Vec<_> = (0..=u8::MAX).map(|byte| (byte, None)).collect();
        let alphabet = alphabet.try_into().unwrap();
        Self { alphabet, words: vec![], filled: FilledBehaviour::Clear }
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
            &mut self.words[(index - G::from(u8::MAX - 1)).try_into().unwrap()]
        }
    }
}