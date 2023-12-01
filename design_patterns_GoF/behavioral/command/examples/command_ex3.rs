// Command trait representing the command interface
trait Command {
    fn execute(&mut self);
}
// Concrete Command
struct LightOnCommand {
    light: Light,
}
impl LightOnCommand {
    pub fn new(light: Light) -> Self {
        LightOnCommand { light }
    }
}
impl Command for LightOnCommand {
    fn execute(&mut self) {
        self.light.turn_on();
    }
}
struct Light {
    is_on: bool,
}
impl Light {
    pub fn new() -> Self {
        Light { is_on: false }
    }
    pub fn turn_on(&mut self) {
        self.is_on = true;
        println!("Light is ON");
    }
}
// Invoker
struct RemoteControl {
    command: Box<dyn Command>,
}
impl RemoteControl {
    pub fn new(command: Box<dyn Command>) -> Self {
        RemoteControl { command }
    }
    pub fn press_button(&mut self) {
        self.command.execute();
    }
}
// Client code
// cargo run --example command_ex3
fn main() {
    let light = Light::new();
    let light_on = LightOnCommand::new(light);
    let mut remote = RemoteControl::new(Box::new(light_on));
    remote.press_button();
}
