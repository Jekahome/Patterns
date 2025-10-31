#[derive(Clone, Copy)]
enum PizzaType {
    CHEESE,
    PEPPERONI,
}

// Классы создатели --------------------------------------------------------------------------------
trait PizzaFactory {
    fn order_pizza(&mut self, types: PizzaType) -> Box<dyn Pizza> {
        let mut pizza: Box<dyn Pizza> = self.create_pizza(types);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.boxes();
        pizza
    }
    fn create_pizza(&self, types: PizzaType) -> Box<dyn Pizza>;
}

struct ChicagoPizzaConcreteFactory;
impl PizzaFactory for ChicagoPizzaConcreteFactory {
    fn create_pizza(&self, types: PizzaType) -> Box<dyn Pizza> {
        match types {
            PizzaType::CHEESE => Box::new(ChicagoStyleSheesePizza::default()),
            PizzaType::PEPPERONI => Box::new(ChicagoStylePepperoniPizza::default()),
        }
    }
}

struct NYPizzaConcreteFactory;
impl PizzaFactory for NYPizzaConcreteFactory {
    fn create_pizza(&self, types: PizzaType) -> Box<dyn Pizza> {
        match types {
            PizzaType::CHEESE => Box::new(NYStyleSheesePizza::default()),
            PizzaType::PEPPERONI => Box::new(NYStylePepperoniPizza::default()),
        }
    }
}

// Классы продукты ---------------------------------------------------------------------------------
trait Pizza {
    fn prepare(&mut self) {
        println!("prepare {}", self.get_name());
    }
    fn bake(&mut self) {
        println!("bake {}", self.get_name());
    }
    fn cut(&mut self) {
        println!("cut {}", self.get_name());
    }
    fn boxes(&mut self) {
        println!("box {}", self.get_name());
    }
    fn get_name(&self) -> String;
}

struct ChicagoStyleSheesePizza {
    name: String,
}
impl Default for ChicagoStyleSheesePizza {
    fn default() -> Self {
        Self {
            name: String::from("ChicagoStyleSheesePizza"),
        }
    }
}
impl Pizza for ChicagoStyleSheesePizza {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct ChicagoStylePepperoniPizza {
    name: String,
}
impl Default for ChicagoStylePepperoniPizza {
    fn default() -> Self {
        Self {
            name: String::from("ChicagoStylePepperoniPizza"),
        }
    }
}
impl Pizza for ChicagoStylePepperoniPizza {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct NYStyleSheesePizza {
    name: String,
}
impl Default for NYStyleSheesePizza {
    fn default() -> Self {
        Self {
            name: String::from("NYStyleSheesePizza"),
        }
    }
}
impl Pizza for NYStyleSheesePizza {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct NYStylePepperoniPizza {
    name: String,
}
impl Default for NYStylePepperoniPizza {
    fn default() -> Self {
        Self {
            name: String::from("NYStylePepperoniPizza"),
        }
    }
}
impl Pizza for NYStylePepperoniPizza {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn new_pizza(creator: &mut Box<dyn PizzaFactory>, types: PizzaType) -> Box<dyn Pizza> {
    creator.order_pizza(types)
}

// cargo run --example fabric_method_ex1
fn main() {
    let mut pizza = new_pizza(
        &mut (Box::new(NYPizzaConcreteFactory) as Box<PizzaFactory>),
        PizzaType::CHEESE,
    );
    pizza = new_pizza(
        &mut (Box::new(NYPizzaConcreteFactory) as Box<PizzaFactory>),
        PizzaType::PEPPERONI,
    );

    pizza = new_pizza(
        &mut (Box::new(ChicagoPizzaConcreteFactory) as Box<PizzaFactory>),
        PizzaType::CHEESE,
    );
    pizza = new_pizza(
        &mut (Box::new(ChicagoPizzaConcreteFactory) as Box<PizzaFactory>),
        PizzaType::PEPPERONI,
    );
}
