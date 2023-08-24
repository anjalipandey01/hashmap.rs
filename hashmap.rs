use std::collections::HashMap;

// Define the SortByKey trait
trait SortByKey<K, V> {
    fn sort_by_key(&mut self);
}

// Implement the SortByKey trait for HashMap
impl<K: Ord, V> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&mut self) {
        let mut sorted_pairs: Vec<_> = self.drain().collect();
        sorted_pairs.sort_by_key(|(key, _)| key.clone());

        for (key, value) in sorted_pairs {
            self.insert(key,value);
        }
    }
}

fn main() {
    // Create a new HashMap instance
    let mut my_map: HashMap<i32, &str> = HashMap::new();

    // Add key-value pairs
    my_map.insert(3, "three");
    my_map.insert(1, "one");
    my_map.insert(2, "two");

    // Print the original map
    println!("Original map: {:?}",my_map);

    // Sort the map by key
    my_map.sort_by_key();

    // Print the sorted map
    println!("Sorted map: {:?}",my_map);
}