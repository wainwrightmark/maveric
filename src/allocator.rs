use bevy::{prelude::*, utils::HashMap};

use crate::child_key::ChildKey;

#[derive(Debug, Default)]
pub(crate) struct Allocator {
    pub ordered_children: Slab<Vec<Entity>>,
    pub ordered_indices: Slab<Vec<Option<usize>>>,

    pub ordered_entities: Slab<HashMap<ChildKey, (usize, Entity)>>,
    pub unordered_entities: Slab<HashMap<ChildKey, Entity>>,
}

// impl Allocator {
//     pub fn print_info(&self){
//         println!("Allocator");
//         self.ordered_children.print_info();
//         self.ordered_indices.print_info();
//         self.ordered_entities.print_info();
//         self.unordered_entities.print_info();
//     }
// }



pub(crate) trait Clear {
    fn clear(&mut self);
}

impl<T> Clear for Vec<T> {
    fn clear(&mut self) {
        self.clear();
    }
}

impl<K, V> Clear for HashMap<K, V> {
    fn clear(&mut self) {
        self.clear();
    }
}

#[derive(Debug, Default)]
pub(crate) struct Slab<T: Default + Clear> {
    elements: Vec<T>,
}

impl<T: Default + Clear> Slab<T> {
    pub fn claim(&mut self) -> T {
        if let Some(x) = self.elements.pop() {
            x
        } else {
            T::default()
        }
    }

    pub fn reclaim(&mut self, mut element: T) {
        element.clear();
        self.elements.push(element)
    }

    // pub fn print_info(&self){

    //     println!("{} {}", std::any::type_name::<T>(), self.elements.len())
    // }
}
