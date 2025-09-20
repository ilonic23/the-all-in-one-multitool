use std::collections::HashMap;

pub struct List {
    pub map: HashMap<String, ListItem>
}

impl List {
    pub fn new() -> List {
        List {
            map: HashMap::new()
        }
    }

    pub fn add(&mut self, item: ListItem) {
        self.map.insert(item.title.clone(), item);
    }
    
    pub fn remove(&mut self, item: String) {
        self.map.remove(&item);
    }
    
    pub fn get(&self, item: &String) -> Option<&ListItem> {
        self.map.get(item)
    }

    pub fn mark(&mut self, key: &String) -> Option<&mut ListItem> {
        match self.map.get_mut(key) {
            Some(item) => {
                item.completed = true;
                Some(item)
            }
            None => None,
        }
    }
}

pub struct ListItem {
    pub title: String,
    pub completed: bool
}