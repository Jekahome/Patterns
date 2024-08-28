
use std::io::{self, BufWriter, Write};
use std::cell::RefCell;

pub trait Writer {
    fn to_stdout(&self, output: &[u8]) -> Result<(), String>;

    fn to_stderr(&self, output: &[u8]) -> Result<(), String>;
}

pub struct UserWriter<W: Writer> {
    writer: W,
}

// Для возможности применения Default необходим конкретный тип (ImplWriter)
// но основные методы реализованы в обобщенном виде для любого типа Writer
impl UserWriter<ImplWriter> {
    pub fn new() -> Self {
        Self {
            writer: ImplWriter,
        }
    }
}

impl Default for UserWriter<ImplWriter> {
    fn default() -> Self {
        Self::new()
    }
}

impl<W: Writer + Default> UserWriter<W> {
    pub fn new_with(writer: W) -> Self {
        Self {
            writer,
        }
    }
    pub fn call(&self, msg: &str) -> Result<(), String>{
        self.writer.to_stdout(msg.as_bytes())
    }
}

#[derive(Default)]
pub struct ImplWriter;
impl Writer for ImplWriter {
    fn to_stdout(&self, output: &[u8]) -> Result<(), String> {
        if !output.is_empty() {
            let stdout = io::stdout();
            let handle = stdout.lock();
            let mut writer = BufWriter::new(handle);
            writer.write_all(output).map_err(|e|e.to_string())?;
            writer.flush().map_err(|e|e.to_string())?;
        }
        Ok(())
    }

    fn to_stderr(&self, output: &[u8]) -> Result<(), String> {
        if !output.is_empty() {
            let stderr = io::stderr();
            let handle = stderr.lock();
            let mut writer = BufWriter::new(handle);
            writer.write_all(output).map_err(|e|e.to_string())?;
            writer.write_all(b"\n").map_err(|e|e.to_string())?;
            writer.flush().map_err(|e|e.to_string())?;
        }
        Ok(())
    }
}


// Вариант использования DI в функциях
pub fn other_some_di(writer: &impl Writer, msg: &str) -> Result<(), String>{
    writer.to_stdout(msg.as_bytes())
}

pub fn other_some_di_2<W: Writer>(writer: &W, msg: &str) -> Result<(), String>{
    W::to_stdout(writer, msg.as_bytes())
}

// Вариант использования DI в методах
struct SomeEntity;

impl SomeEntity{
    fn call(&self, writer: &impl Writer, msg: &str) -> Result<(), String>{
        writer.to_stdout(msg.as_bytes())
    }
    fn call_dyn(&self, writer: &dyn Writer, msg: &str) -> Result<(), String>{
        writer.to_stdout(msg.as_bytes())
    }
    fn call_box(&self, writer: Box<dyn Writer>, msg: &str) -> Result<(), String>{
        writer.to_stdout(msg.as_bytes())
    }
}

// Ленивая инициализация с помощью внутренней изменчивости
struct LazyDI<W: Writer + Default>(RefCell<Option<W>>); 
    
impl<W: Writer + Default> LazyDI<W>{
    fn call(&self, msg: &str)-> Result<(), String>{
        if self.0.borrow().is_none(){
            *self.0.borrow_mut() = Some(W::default());
        }
         
        if let Some(writer) = self.0.borrow().as_ref(){
            writer.to_stdout(msg.as_bytes());
        }
        
        Ok(()) 
    }
}

// Ленивая инициализация с помощью замыкания
// unimplemented!

fn main(){
    let mut user_writer = UserWriter::default();
    user_writer.call("hello 1\n");


    let w = ImplWriter;
    other_some_di(&w,"hello 2\n");
    other_some_di_2(&w,"hello 3\n");

    let s = SomeEntity;
    s.call(&w, "hello 4\n");
    s.call_dyn(&w, "hello 5\n");
    s.call_box(Box::new(w), "hello 6\n");

    let l: LazyDI<ImplWriter> = LazyDI(RefCell::new(None));
    l.call("hello 7\n");
}

#[cfg(test)]
mod test_writer{
    use std::{
        cell::RefCell,
        path::{Path, PathBuf},
        rc::Rc,
    };

    use super::{Writer, UserWriter};

    pub struct TestImplWriter {
        pub stdout: Rc<RefCell<Vec<u8>>>,
        pub stderr: Rc<RefCell<Vec<u8>>>,
    }
    impl Default for TestImplWriter {
        fn default() -> Self {
            Self {
                stdout: Rc::new(RefCell::new(Vec::new())),
                stderr: Rc::new(RefCell::new(Vec::new())),
            }
        }
    }
    impl TestImplWriter {
        pub fn set(
            &mut self,
            stdout: Rc<RefCell<Vec<u8>>>,
            stderr: Rc<RefCell<Vec<u8>>>,
        ) {
            self.stdout = stdout;
            self.stderr = stderr;
        }
    }
    impl Writer for TestImplWriter {
        fn to_stdout(
            &self,
            output: &[u8],
        ) -> Result<(), String> {
            if !output.is_empty() {
                self.stdout.borrow_mut().extend_from_slice(output);
            }
            Ok(())
        }

        fn to_stderr(
            &self,
            output: &[u8],
        ) -> Result<(), String> {
            if !output.is_empty() {
                self.stderr.borrow_mut().extend_from_slice(output);
                self.stderr.borrow_mut().extend_from_slice(b"\n");
            }
            Ok(())
        }
    }

    #[test]
    fn test(){
        let result_stdout = Rc::new(RefCell::new(Vec::new()));
        let result_stderr = Rc::new(RefCell::new(Vec::new()));

        let mut writer = TestImplWriter::default();
        writer.set(result_stdout.clone(), result_stderr.clone());
        let mock_user_writer = UserWriter::new_with(
            writer,
        );
 
        mock_user_writer.call("hello\n");

        let guard = result_stdout.borrow();
        let output_str = String::from_utf8_lossy(&guard);
        assert_eq!(output_str, "hello\n");

        let guard = result_stderr.borrow();
        let output_str = String::from_utf8_lossy(&guard);
        assert_eq!(output_str, "");
    }
}