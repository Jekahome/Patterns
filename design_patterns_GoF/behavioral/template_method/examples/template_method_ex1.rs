// Base class defining the template method
trait Algorithm {
    // Template method
    fn execute(&self) {
        self.step_one();
        self.step_two();
        self.step_three();
    }
    fn step_one(&self) {
        println!("Step 1: Common Initialization");
    }
    // Placeholder methods to be overridden in subclasses
    fn step_two(&self);
    fn step_three(&self) {
        println!("Step 3: Common Cleanup");
    }
}

// Concrete implementation
struct ConcreteAlgorithmA;
impl Algorithm for ConcreteAlgorithmA {
    fn step_two(&self) {
        println!("Step 2A: Specific Processing for A");
    }
}
struct ConcreteAlgorithmB;
impl Algorithm for ConcreteAlgorithmB {
    fn step_two(&self) {
        println!("Step 2B: Specific Processing for B");
    }
}
// Client code
// cargo run --example template_method_ex1
fn main() {
    let algo_a = ConcreteAlgorithmA;
    let algo_b = ConcreteAlgorithmB;
    println!("Executing Algorithm A:");
    algo_a.execute();
    println!("\nExecuting Algorithm B:");
    algo_b.execute();
}
