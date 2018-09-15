use crate::entity::*;
use crate::store::Store;
use slotmap::Key;

pub trait RegisterComponents {
    fn domain_register_components(&mut self);
}

impl RegisterComponents for Store {
    fn domain_register_components(&mut self) {
        self.register_component::<Person>();
        self.register_component::<Book>();
    }
}

pub trait DomainSpecificStuff {
    fn link_keys_book_to_person(&self, p_key: Key, b_key: Key) -> (Key, Key);
    fn link_keys_friends(&self, person_a_key: Key, person_b_key: Key) -> (Key, Key);
}
impl DomainSpecificStuff for Store {
    fn link_keys_book_to_person(&self, p_key: Key, b_key: Key) -> (Key, Key) {
        let b_exists = self.entity_exists::<Book>(b_key);
        let p_exists = self.entity_exists::<Person>(p_key);

        if b_exists && p_exists {
            self.store_parent_child_keys::<Person, Book>(p_key, b_key)
        } else {
            (Key::null(), Key::null())
        }
    }

    fn link_keys_friends(&self, person_a_key: Key, person_b_key: Key) -> (Key, Key) {
        let p_a_exists = self.entity_exists::<Person>(person_a_key);
        let p_b_exists = self.entity_exists::<Person>(person_b_key);

        if p_a_exists && p_b_exists {
            if let Ok(mut p_sm) = self.write_to::<Person>() {
                {
                    let person_a = p_sm.get_mut(person_a_key).expect("person does not exist");
                    person_a.friend_keys.push(person_b_key);
                }
                {
                    let person_b = p_sm.get_mut(person_b_key).expect("person does not exist");
                    person_b.friend_keys.push(person_a_key);
                }
                (person_a_key, person_b_key)
            } else {
                (Key::null(), Key::null())
            }
        } else {
            (Key::null(), Key::null())
        }
    }
}
