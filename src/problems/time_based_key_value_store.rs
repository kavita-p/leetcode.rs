use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, HashMap<i32, String>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let value_map = self.map.entry(key).or_insert_with(HashMap::new);
        value_map.insert(timestamp, value);
    }

    fn get(&mut self, key: String, timestamp: i32) -> String {
        if let Some(val) = &self.map.get(&key) {
            if let Some(stamped_val) = val.get(&timestamp) {
                stamped_val.to_string()
            } else {
                let stamps = val.keys();
                if let Some(most_recent) = stamps.filter(|&t| *t < timestamp).max() {
                    if let Some(stamped_val) = val.get(most_recent) {
                        stamped_val.to_string()
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                }
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
