// Step 1: Define the target interface
pub trait JsonLogger {
    fn log(&self, data: &str);
}
// The existing class with an incompatible interface
pub struct XmlLogger;
impl XmlLogger {
    pub fn log_xml(&self, xml_data: &str) {
        println!("Logging XML: {}", xml_data);
    }
}
// Step 2: Create the adapter class
pub struct LoggerAdapter {
    xml_logger: XmlLogger,
}
impl LoggerAdapter {
    pub fn new(xml_logger: XmlLogger) -> Self {
        LoggerAdapter { xml_logger }
    }
}
// Step 3 & 4: Implement the target interface using the adapted object's methods
impl JsonLogger for LoggerAdapter {
    fn log(&self, data: &str) {
        // Here, we're simply passing the JSON data to the XML logger
        // In a real-world scenario, you would convert the JSON data to XML format
        self.xml_logger.log_xml(data);
    }
}
// Client code
// cargo run --example adapter_ex1
fn main() {
    let xml_logger = XmlLogger;
    let logger = LoggerAdapter::new(xml_logger);
    logger.log("{\"message\": \"This is a log.\"}");
}
