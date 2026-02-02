use godot::classes::class_macros::private::virtuals::Os::Vector2i;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy)]
struct Vector2iKey(Vector2i);

impl PartialEq for Vector2iKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

impl Eq for Vector2iKey {}

impl Hash for Vector2iKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.x.hash(state);
        self.0.y.hash(state);
    }
}

pub fn create_resolution_map() {
    let mut resolution_map: HashMap<_, _> = HashMap::new();

    resolution_map.insert(
        Vector2iKey(Vector2i { x: 800, y: 600 }),
        Vector2i { x: 213, y: 300 },
    );
}
