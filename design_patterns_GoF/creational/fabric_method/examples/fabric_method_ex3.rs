// Step 1 & 3: Product interface and its concrete implementation
trait Button {
    fn render(&self);
}
struct WindowsButton;
impl Button for WindowsButton {
    fn render(&self) {
        println!("Rendering a Windows button");
    }
}
struct MacButton;
impl Button for MacButton {
    fn render(&self) {
        println!("Rendering a Mac button");
    }
}
// Step 2: Creator interface and its concrete implementations
trait ButtonFactory {
    fn create_button(&self) -> Box<dyn Button>;
}
struct WindowsButtonFactory;
impl ButtonFactory for WindowsButtonFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}
struct MacButtonFactory;
impl ButtonFactory for MacButtonFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }
}
// Step 5: Using the factory method
// cargo run --example fabric_method_ex3
fn main() {
    let factory: Box<dyn ButtonFactory> = if cfg!(target_os = "windows") {
        Box::new(WindowsButtonFactory)
    } else {
        Box::new(MacButtonFactory)
    };
    let button = factory.create_button();
    button.render();
}
