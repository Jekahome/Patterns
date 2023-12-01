// Step 1: Abstract products
trait Button {
    fn render(&self);
}
trait Checkbox {
    fn check(&self);
}
// Step 2: Concrete products
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
struct WindowsCheckbox;
impl Checkbox for WindowsCheckbox {
    fn check(&self) {
        println!("Checking a Windows checkbox");
    }
}
struct MacCheckbox;
impl Checkbox for MacCheckbox {
    fn check(&self) {
        println!("Checking a Mac checkbox");
    }
}
// Step 3 & 4: Abstract factory and its concrete implementations
trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}
struct WindowsFactory;
impl GUIFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}
struct MacFactory;
impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}
// Step 5: Client code
fn main() {
    let factory: Box<dyn GUIFactory> = if cfg!(target_os = "windows") {
        Box::new(WindowsFactory)
    } else {
        Box::new(MacFactory)
    };
    let button = factory.create_button();
    button.render();
    let checkbox = factory.create_checkbox();
    checkbox.check();
}
