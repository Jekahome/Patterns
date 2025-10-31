// Step 1: Common interface for core and decorators
pub trait Beverage {
    fn cost(&self) -> f32;
    fn description(&self) -> String;
}
// Step 2: Concrete core object
pub struct Coffee;
impl Beverage for Coffee {
    fn cost(&self) -> f32 {
        2.0
    }
    fn description(&self) -> String {
        "Coffee".to_string()
    }
}
// Step 3: Decorator classes
pub struct Milk<'a> {
    beverage: &'a dyn Beverage,
}
impl<'a> Milk<'a> {
    pub fn new(beverage: &'a dyn Beverage) -> Self {
        Milk { beverage }
    }
}
impl<'a> Beverage for Milk<'a> {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.5
    }
    fn description(&self) -> String {
        format!("{} with Milk", self.beverage.description())
    }
}
pub struct Sugar<'a> {
    beverage: &'a dyn Beverage,
}
impl<'a> Sugar<'a> {
    pub fn new(beverage: &'a dyn Beverage) -> Self {
        Sugar { beverage }
    }
}
impl<'a> Beverage for Sugar<'a> {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.25
    }
    fn description(&self) -> String {
        format!("{} with Sugar", self.beverage.description())
    }
}
// Client code
// cargo run --example decorator_ex2
fn main() {
    let my_coffee = Coffee;
    let my_coffee_with_milk = Milk::new(&my_coffee);
    let my_coffee_with_milk_and_sugar = Sugar::new(&my_coffee_with_milk);
    println!("{}", my_coffee_with_milk_and_sugar.description());
    println!("Cost: ${}", my_coffee_with_milk_and_sugar.cost());
}
