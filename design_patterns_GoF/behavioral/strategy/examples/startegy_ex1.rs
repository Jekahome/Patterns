// Strategy trait
trait CompressionStrategy {
    fn compress(&self, data: &str) -> String;
}
// Concrete Strategies
struct ZipCompression;
impl CompressionStrategy for ZipCompression {
    fn compress(&self, data: &str) -> String {
        format!("{} (ZIP compressed)", data)
    }
}
struct RarCompression;
impl CompressionStrategy for RarCompression {
    fn compress(&self, data: &str) -> String {
        format!("{} (RAR compressed)", data)
    }
}
// Context struct that uses a compression strategy
struct Compressor {
    strategy: Box<dyn CompressionStrategy>,
}
impl Compressor {
    fn new(strategy: Box<dyn CompressionStrategy>) -> Self {
        Compressor { strategy }
    }
    fn compress_data(&self, data: &str) -> String {
        self.strategy.compress(data)
    }
}
// Client Code
// cargo run --example startegy_ex1
fn main() {
    let zip = ZipCompression;
    let rar = RarCompression;
    let mut compressor = Compressor::new(Box::new(zip));
    println!("{}", compressor.compress_data("MyData"));
    compressor = Compressor::new(Box::new(rar));
    println!("{}", compressor.compress_data("MyData"));
}
