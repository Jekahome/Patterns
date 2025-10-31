#![allow(dead_code)]
// The Context keeps track of information that the interpreter needs
struct Context {
    input: String,
    output: i32,
}
// The Abstract Expression trait
trait Expression {
    fn interpret(&self, context: &mut Context);
}
// Terminal Expression
struct NumberExpression {
    number: i32,
}
impl NumberExpression {
    fn new(number: i32) -> Self {
        NumberExpression { number }
    }
}
impl Expression for NumberExpression {
    fn interpret(&self, context: &mut Context) {
        context.output = self.number;
    }
}
// Client Code
// cargo run --example interpreter_ex1
fn main() {
    let context = &mut Context {
        input: "Number: 5".to_string(),
        output: 0,
    };
    let number = NumberExpression::new(5);
    number.interpret(context);
    println!("Interpreted number: {}", context.output);
}
