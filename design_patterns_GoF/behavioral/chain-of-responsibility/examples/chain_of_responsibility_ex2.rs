// Abstract handler
trait Handler {
    fn set_next(&mut self, next: Box<dyn Handler>);
    fn handle(&self, request: &str);
}
// Concrete handler 1
struct Handler1 {
    next: Option<Box<dyn Handler>>,
}
impl Handler1 {
    pub fn new() -> Self {
        Handler1 { next: None }
    }
}
impl Handler for Handler1 {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }
    fn handle(&self, request: &str) {
        if request == "Request1" {
            println!("Handler1: Handling {}", request);
        } else if let Some(next) = &self.next {
            next.handle(request);
        }
    }
}
// Concrete handler 2
struct Handler2 {
    next: Option<Box<dyn Handler>>,
}
impl Handler2 {
    pub fn new() -> Self {
        Handler2 { next: None }
    }
}
impl Handler for Handler2 {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }
    fn handle(&self, request: &str) {
        if request == "Request2" {
            println!("Handler2: Handling {}", request);
        } else if let Some(next) = &self.next {
            next.handle(request);
        }
    }
}
// Client code
// cargo run --example chain_of_responsibility_ex2
fn main() {
    let mut handler1 = Handler1::new();
    let handler2 = Handler2::new();
    handler1.set_next(Box::new(handler2));
    handler1.handle("Request1");
    handler1.handle("Request2");
}
