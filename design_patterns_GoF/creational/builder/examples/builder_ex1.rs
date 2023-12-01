// Step 1: Define the product
pub struct Meal {
    starter: Option<String>,
    main_course: Option<String>,
    dessert: Option<String>,
}
impl Meal {
    pub fn new() -> Self {
        Meal {
            starter: None,
            main_course: None,
            dessert: None,
        }
    }
}
// Step 2: Builder interface
pub trait MealBuilder {
    fn add_starter(&mut self, starter: String);
    fn add_main_course(&mut self, main_course: String);
    fn add_dessert(&mut self, dessert: String);
    fn build(&self) -> Meal;
}
// Step 3: Concrete builder
pub struct FullMealBuilder {
    meal: Meal,
}
impl FullMealBuilder {
    pub fn new() -> Self {
        FullMealBuilder { meal: Meal::new() }
    }
}
impl MealBuilder for FullMealBuilder {
    fn add_starter(&mut self, starter: String) {
        self.meal.starter = Some(starter);
    }
    fn add_main_course(&mut self, main_course: String) {
        self.meal.main_course = Some(main_course);
    }
    fn add_dessert(&mut self, dessert: String) {
        self.meal.dessert = Some(dessert);
    }
    fn build(&self) -> Meal {
        Meal {
            starter: self.meal.starter.clone(),
            main_course: self.meal.main_course.clone(),
            dessert: self.meal.dessert.clone(),
        }
    }
}
// Step 4: Director
pub fn construct_meal(builder: &mut dyn MealBuilder) -> Meal {
    builder.add_starter("Soup".to_string());
    builder.add_main_course("Steak".to_string());
    builder.add_dessert("Ice cream".to_string());
    builder.build()
}
// Step 5: Client code
// cargo run --example builder_ex1
fn main() {
    let mut builder = FullMealBuilder::new();
    let meal = construct_meal(&mut builder);
    println!("Starter: {:?}", meal.starter);
    println!("Main Course: {:?}", meal.main_course);
    println!("Dessert: {:?}", meal.dessert);
}
