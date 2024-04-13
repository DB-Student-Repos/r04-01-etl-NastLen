use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
   let mut new_map = BTreeMap::new();

   for (&score, letters) in h { // score -> key; letters -> value; h is a reference to each key-value pair in the BTreeMap
       for letter in letters { // the loop is iterating over each letter in the letters vector
           new_map.insert(letter.to_lowercase().next().unwrap(), score); // the key is the lowercase letter and the value is the score

       }
   }
   new_map
}

