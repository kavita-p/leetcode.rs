// url: https://leetcode.com/problems/time-based-key-value-store
// date: 6 oct. 2022
// thanks to forum solutions for the idea to use a vec and binary-search it
// previous HashMap<String, HashMap<i32, String>> worked
// but was too slow

use std::collections::HashMap;

#[derive(Default)]
struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Default::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(val) = self.map.get(&key) {
            match val.binary_search_by_key(&timestamp, |&(t, _)| t) {
                Ok(i) => val[i].1.clone(),
                Err(i) if i > 0 => val[i - 1].1.clone(),
                _ => "".to_string(),
            }
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_e1() {
        let mut obj = TimeMap::new();
        obj.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(obj.get("foo".to_string(), 1), "bar".to_string());

        assert_eq!(obj.get("foo".to_string(), 3), "bar".to_string());

        obj.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(obj.get("foo".to_string(), 4), "bar2");
        assert_eq!(obj.get("foo".to_string(), 5), "bar2");
    }
}
