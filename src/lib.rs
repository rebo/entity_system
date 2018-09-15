#![feature(uniform_paths)]

#[macro_use]
extern crate lazy_static;

pub mod domain;
pub mod entity;
pub mod store;

pub use domain::*;
pub use store::Store;
pub fn libmain() {}

lazy_static! {
    pub static ref STORE: Store = { Store::new() };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::*;
    #[test]
    fn create_person() {
        let person = Person::new("The Hulk");

        let p_key = STORE.store_entity(person);
        if let Ok(person_slotmap) = STORE.read_from::<Person>() {
            let person = person_slotmap.get(p_key).expect("person doesnt exist!");

            assert_eq!(person.name, "The Hulk");
        }
    }
    #[test]
    fn create_book() {
        let book = Book::new("Moby Dick");

        let b_key = STORE.store_entity(book);
        if let Ok(book_slotmap) = STORE.read_from::<Book>() {
            let book = book_slotmap.get(b_key).expect("book doesnt exist!");

            assert_eq!(book.title, "Moby Dick");
        }
    }

    #[test]
    fn create_books_and_assign_to_person() {
        let person = Person::new("The Hulk");
        let p_key = STORE.store_entity(person);

        let moby_dick = Book::new("Moby Dick");
        let lotr = Book::new("Lord of the Rings");

        let moby_dick_key = STORE.store_entity(moby_dick);
        let lotr_key = STORE.store_entity(lotr);

        let (p_key, moby_dick_key) = STORE.link_keys_book_to_person(p_key, moby_dick_key);

        let (p_key, lotr_key) = STORE.link_keys_book_to_person(p_key, lotr_key);

        if let Ok(book_slotmap) = STORE.read_from::<Book>() {
            let moby_dick_read = book_slotmap.get(moby_dick_key).expect("book doesnt exist!");
            let lotr_read = book_slotmap.get(lotr_key).expect("book doesnt exist!");

            assert_eq!(moby_dick_read.title, "Moby Dick");
            assert_eq!(lotr_read.title, "Lord of the Rings");
        }

        if let (Ok(person_slotmap), Ok(book_slotmap)) = STORE.read_from2::<Person, Book>() {
            let person = person_slotmap.get(p_key).expect("person doesnt exist!");
            assert_eq!(person.book_keys.len(), 2);

            if let Some(book) = person.books(&book_slotmap).first() {
                assert_eq!(book.title, "Moby Dick");
            }
        }
    }

    #[test]
    fn create_friends() {
        let hulk = Person::new("The Hulk");
        let thor = Person::new("Thor");
        let hulk_key = STORE.store_entity(hulk);
        let thor_key = STORE.store_entity(thor);

        let (hulk_key, thor_key) = STORE.link_keys_friends(hulk_key, thor_key);

        if let Ok(person_slotmap) = STORE.read_from::<Person>() {
            let hulk = person_slotmap.get(hulk_key).expect("person doesnt exist!");

            assert_eq!(hulk.friend_keys.len(), 1);

            if let Some(friend) = hulk.friends(&person_slotmap).first() {
                assert_eq!(friend.name, "Thor");
            }

            let thor = person_slotmap.get(thor_key).expect("person doesnt exist!");

            assert_eq!(thor.friend_keys.len(), 1);

            if let Some(friend) = thor.friends(&person_slotmap).first() {
                assert_eq!(friend.name, "Hulk");
            }
        }
    }

}
