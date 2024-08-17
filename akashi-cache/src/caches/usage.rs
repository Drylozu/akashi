use dashmap::DashMap;

pub struct UsageCache {
    /// command name and it's number of usages
    pub cache: DashMap<String, i32>,
}

impl UsageCache {
    pub fn new() -> Self {
        Self {
            cache: DashMap::new(),
        }
    }

    pub fn increment(&self, command: String) {
        let mut count = self.cache.entry(command).or_insert(0);
        *count += 1;
    }

    pub fn get(&self, command: String) -> Option<i32> {
        self.cache.get(&command).map(|v| v.value().clone())
    }

    pub fn ordered_entries(&self) -> Vec<(String, i32)> {
        let mut entries: Vec<(String, i32)> = self
            .cache
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1));

        entries
    }

    pub fn clear(&self) {
        self.cache.clear();
    }
}
