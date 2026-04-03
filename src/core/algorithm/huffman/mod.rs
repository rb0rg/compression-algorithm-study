use std::collections::HashMap;

pub fn generate_frequency_map(file_bytes: &Vec<u8>) -> HashMap<u8, usize> {
    let mut map: HashMap<u8, usize> = HashMap::new();

    for byte in file_bytes.iter() {
        *map.entry(*byte).or_insert(0) += 1;
    }

    map
}
