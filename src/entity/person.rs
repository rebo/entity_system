use super::*;

use slotmap::{Key, SlotMap};

pub struct Person {
    pub name: String,

    pub key: Key,
    pub book_keys: Vec<Key>,
    pub friend_keys: Vec<Key>,
}

impl Person {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            key: Key::null(),
            book_keys: vec![],
            friend_keys: vec![],
        }
    }

    pub fn books<'a>(&self, b_sm: &'a SlotMap<Book>) -> Vec<&'a Book> {
        self.book_keys
            .iter()
            .filter_map(|b| b_sm.get(*b))
            .collect::<Vec<&Book>>()
    }

    pub fn friends<'a>(&self, sm: &'a SlotMap<Person>) -> Vec<&'a Person> {
        self.book_keys
            .iter()
            .filter_map(|f| sm.get(*f))
            .collect::<Vec<&Person>>()
    }
}

impl OverloadedChildKeys<Book> for Person {
    fn overloaded_child_keys<'a>(&'a mut self) -> &'a mut Vec<Key> {
        &mut self.book_keys
    }
}

impl OverloadedChildKeys<Person> for Person {
    fn overloaded_child_keys<'a>(&'a mut self) -> &'a mut Vec<Key> {
        &mut self.friend_keys
    }
}
