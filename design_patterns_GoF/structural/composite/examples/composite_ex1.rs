// Step 1: Define the component interface
pub trait Graphic {
    fn draw(&self);
}
// Step 2: Leaf object
pub struct Circle;
impl Graphic for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}
pub struct Square;
impl Graphic for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}
// Step 3: Composite object
pub struct GraphicGroup {
    graphics: Vec<Box<dyn Graphic>>,
}
impl GraphicGroup {
    pub fn new() -> Self {
        GraphicGroup {
            graphics: Vec::new(),
        }
    }
    pub fn add(&mut self, graphic: Box<dyn Graphic>) {
        self.graphics.push(graphic);
    }
}
impl Graphic for GraphicGroup {
    fn draw(&self) {
        for graphic in &self.graphics {
            graphic.draw();
        }
    }
}
// Client code
// cargo run --example composite_ex1
fn main() {
    let circle1 = Circle;
    let square1 = Square;
    let mut group1 = GraphicGroup::new();
    group1.add(Box::new(circle1));
    group1.add(Box::new(square1));
    let circle2 = Circle;
    let mut group2 = GraphicGroup::new();
    group2.add(Box::new(circle2));
    group2.add(Box::new(group1));
    group2.draw();
}
