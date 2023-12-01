// Intrinsic state
struct TreeType {
    name: String,
    texture: String,
}
impl TreeType {
    pub fn new(name: &str, texture: &str) -> Self {
        TreeType {
            name: name.to_string(),
            texture: texture.to_string(),
        }
    }
    pub fn draw(&self, x: i32, y: i32) {
        println!(
            "Drawing {} at ({}, {}) with texture {}",
            self.name, x, y, self.texture
        );
    }
}
// Flyweight factory
struct TreeFactory {
    tree_types: Vec<TreeType>,
}
impl TreeFactory {
    pub fn new() -> Self {
        TreeFactory {
            tree_types: Vec::new(),
        }
    }
    pub fn get_tree_type(&mut self, name: &str, texture: &str) -> &TreeType {
        if self
            .tree_types
            .iter()
            .any(|t| t.name == name && t.texture == texture)
        {
            self.tree_types
                .iter()
                .find(|t| t.name == name && t.texture == texture)
                .unwrap()
        } else {
            let tree_type = TreeType::new(name, texture);
            self.tree_types.push(tree_type);

            return self.tree_types.last().unwrap();
        }
    }
}
// Client code
// cargo run --example flyweight_ex1
fn main() {
    let mut factory = TreeFactory::new();
    let tree_type1: &TreeType = factory.get_tree_type("Pine", "PineTexture");
    tree_type1.draw(10, 20);
    tree_type1.draw(50, 60);

    let tree_type2: &TreeType = factory.get_tree_type("Oak", "OakTexture");
    tree_type2.draw(30, 40);
}
