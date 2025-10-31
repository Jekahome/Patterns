// Element trait
trait Element {
    fn accept(&self, visitor: &dyn Visitor);
}
// Concrete elements
struct ElementA;
impl Element for ElementA {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_element_a(self);
    }
}
struct ElementB;
impl Element for ElementB {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_element_b(self);
    }
}
// Visitor trait
trait Visitor {
    fn visit_element_a(&self, element: &ElementA);
    fn visit_element_b(&self, element: &ElementB);
}
// Concrete visitor
struct ConcreteVisitor;
impl Visitor for ConcreteVisitor {
    fn visit_element_a(&self, _: &ElementA) {
        println!("Visited ElementA with ConcreteVisitor");
    }
    fn visit_element_b(&self, _: &ElementB) {
        println!("Visited ElementB with ConcreteVisitor");
    }
}
// Client code
// cargo run --example visitor_ex2
fn main() {
    let elements: Vec<Box<dyn Element>> = vec![Box::new(ElementA), Box::new(ElementB)];
    let visitor = ConcreteVisitor;
    for element in &elements {
        element.accept(&visitor);
    }
}
