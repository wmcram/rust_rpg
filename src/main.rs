mod map;

fn main() {
    let mut m = map::Map::new(10, 10);
    m.spawn_entity(map::Entity::new("Player".to_string(), 100, 10, 10), 2, 4).unwrap();
    m.render();
}
