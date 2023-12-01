#![allow(dead_code)]
#![allow(unused_mut)]

// Abstract subject
trait Subject {
    fn request(&self);
}
// Real subject
struct RealSubject;
impl Subject for RealSubject {
    fn request(&self) {
        println!("RealSubject: Handling the request.");
    }
}
// Proxy
struct Proxy {
    real_subject: Option<RealSubject>,
}
impl Proxy {
    pub fn new() -> Self {
        Proxy { real_subject: None }
    }
    // Lazily instantiate the real subject
    fn ensure_real_subject(&mut self) {
        if self.real_subject.is_none() {
            self.real_subject = Some(RealSubject);
        }
    }
}
impl Subject for Proxy {
    fn request(&self) {
        println!("Proxy: Checking preconditions before accessing RealSubject.");
        if let Some(real_subject) = &self.real_subject {
            real_subject.request();
        }
        println!("Proxy: Additional tasks after accessing RealSubject.");
    }
}
// Client code
// cargo run --example proxy_ex1
fn main() {
    let mut proxy = Proxy::new();
    proxy.request();
}
