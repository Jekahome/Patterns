// Step 1 & 2: Define the abstraction and the implementor interface
// Implementor
pub trait Renderer {
    fn render_circle(&self, radius: f32);
}
// Abstraction
pub struct Circle {
    radius: f32,
    renderer: Box<dyn Renderer>,
}
impl Circle {
    pub fn new(radius: f32, renderer: Box<dyn Renderer>) -> Self {
        Circle { radius, renderer }
    }
    pub fn draw(&self) {
        self.renderer.render_circle(self.radius);
    }
}
// Step 3: Concrete implementations
pub struct OpenGLRenderer;
impl Renderer for OpenGLRenderer {
    fn render_circle(&self, radius: f32) {
        println!("OpenGL: Drawing a circle of radius {}", radius);
    }
}
pub struct DirectXRenderer;
impl Renderer for DirectXRenderer {
    fn render_circle(&self, radius: f32) {
        println!("DirectX: Drawing a circle of radius {}", radius);
    }
}
// Client code
// cargo run --example bridge_ex2
fn main() {
    let opengl = OpenGLRenderer;
    let directx = DirectXRenderer;
    let circle_opengl = Circle::new(5.0, Box::new(opengl));
    let circle_directx = Circle::new(10.0, Box::new(directx));
    circle_opengl.draw();
    circle_directx.draw();
}
