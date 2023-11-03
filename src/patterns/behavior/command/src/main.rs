/*
Команда — это поведенческий паттерн проектирования, который превращает запросы в объекты, 
позволяя передавать их как аргументы при вызове методов, ставить запросы в очередь, логировать их, 
а также поддерживать отмену операций.

Когда использовать паттерн команды:
- Очередь. Когда запросы необходимо обрабатывать в определенные моменты времени и в соответствии с различными триггерными ситуациями.
- Слои. Когда необходимо разделить клиента и поставщика услуг (инкапсуляция получателя, для вызывающего не важно какая команда будет послана)
- Когда возникает необходимость в функции отката для определенных операций
- Когда необходима история запросов
- Когда есть необходимость добавить новые команды
- При необходимости параметризации объектов по действию

Пример:
Существует графический интерфейс с кнопками сохранения своих данных. 
Проблема возникает если мы в слое пользовательского интерфейса будем реализовывать зависящий код от слоя бизнес-логики.
Тогда при изменении бизнес-логики придется переписывать слой пользовательского интерфейса, при чем во всех местах вызова.
Таким образом, команды станут гибкой прослойкой между пользовательским интерфейсом и бизнес-логикой. 
И это лишь малая доля пользы, которую может принести паттерн Команда!

Решение:
Всю логику вынести в бизнес слой и унифицировать все вызова к интерфейсу команды.
*/

use std::env;
use business_layer::Command;
pub mod business_layer {
    pub use base::MacroCommand;
    pub use base::Command;
    pub use command_add_str::CAddStr;
    pub use command_read::CRead;

    mod base {
        pub trait Command {
            fn execute(&self);
        }
        
        pub struct MacroCommand {
            stack: Vec<Box<dyn Command>>,
        }
        impl MacroCommand {
            pub fn new() -> MacroCommand {
                MacroCommand {
                    stack: Vec::new(),
                }
            }
            pub fn push(&mut self, cmd: Box<dyn Command>) {
                self.stack.push(cmd);
            }
            pub fn undo(&mut self) {
                self.stack.pop();
            }
            pub fn clear(&mut self) {
                self.stack.clear();
            }
            pub fn size(&self) -> usize{
                self.stack.len()
            }
        }
        impl Command for MacroCommand {
            fn execute(&self) {
                for command in &self.stack {
                    command.execute();
                }
            }
        }
  }

  mod command_read{
    use std::io::{Write,Read};
    use std::fs::OpenOptions;
    use std::fmt::Display;
    use std::path::Path;

    use super::*;

    pub struct CRead<P>{
        pub file:P
    }
    impl<P: AsRef<Path> + Display> base::Command for CRead<P>{
        fn execute(&self){
            let _ = self.read();
        }
    }
    impl<P: AsRef<Path> + Display> CRead<P>{
        pub fn new(file: P) -> Self{
            Self{file}
        }
        pub fn read(&self) -> std::io::Result<()> {
            println!("Read");
            let display = format!("src/patterns/behavior/command/src/{}",self.file.as_ref().display());
            let path = Path::new(&display);
            let mut f = OpenOptions::new()
                .read(true)
                .open(path)?;
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            let _ = std::io::stdout().write(contents.as_bytes());
            Ok(())
        }
    }
  }

  mod command_add_str{
    use std::io::Write;
    use std::fs::OpenOptions;
    use std::fmt::Display;
    use std::path::Path;
    
    use super::*;

    pub struct CAddStr<M,P>{
        pub file:P,
        pub msg:M
    }
 
    impl<M: AsRef<str>, P: AsRef<Path> + Display> base::Command for CAddStr<M, P>{
        fn execute(&self){
            let _ = self.add_str();
        }
    }

    impl<M: AsRef<str>, P: AsRef<Path> + Display> CAddStr<M, P>{
        pub fn new(file: P, msg: M) -> Self{
            Self{file,msg}
        }
        pub fn add_str(&self) -> std::io::Result<()> {
            println!("Add");
            let display = format!("src/patterns/behavior/command/src/{}",self.file.as_ref().display());
            let path = Path::new(&display);
            let mut f = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)?;
            f.write_fmt(format_args!("{}\n",self.msg.as_ref()))?; 
            Ok(())
        }
    }
  }
 

}
 

// cargo run --example p_behavior_command add 'msg of args' data.txt
// cargo run --example p_behavior_command read data.txt
fn args_user_layer(history:&mut business_layer::MacroCommand, args:Vec<String>) -> bool{
    if args.len() > 3{
        let action = &args[1];
        match action.as_str() {
            "add" => {
                let msg = args[2].to_owned();
                let file = args[3].to_owned();
              
                let cmd = Box::new(business_layer::CAddStr::new(file,msg));
                history.push(cmd);
                return true;
            },
            _ => {return false;}
        }
    } else if args.len() > 2 {
        let action = &args[1];
        match action.as_str() {
            "read" => {
                let file = args[2].to_owned();
                let cmd = Box::new(business_layer::CRead::new(file));
                history.push(cmd);
                return true;
            },
            _ => {return false;}
        }
    }
    false
}

// ENV_VAR_ACTION="add" ENV_VAR_VALUE='msg of env' ENV_VAR_FILE='data.txt' cargo run --example p_behavior_command
// ENV_VAR_ACTION="read" ENV_VAR_FILE='data.txt' cargo run --example p_behavior_command
fn env_user_layer(history:&mut business_layer::MacroCommand) -> bool{
    if let Ok(action) = env::var("ENV_VAR_ACTION"){
        match action.as_str() {
            "read" => {
                if let Ok(file) = env::var("ENV_VAR_FILE"){
                    let cmd = Box::new(business_layer::CRead::new(file));
                    history.push(cmd);
                    return true;
                } 
            },
            "add" => {
                if let Ok(msg) = env::var("ENV_VAR_VALUE"){
                    if let Ok(file) = env::var("ENV_VAR_FILE"){
                        let cmd = Box::new(business_layer::CAddStr::new(file,msg));
                        history.push(cmd);
                        return true;
                    }
                } 
            },
            _ => {return false;}
        }
    }
    false
}

fn main(){
    let mut history = business_layer::MacroCommand::new();

    env_user_layer(&mut history);

    let args: Vec<String> = env::args().collect();
    args_user_layer(&mut history,args);
          
    // + Своя логика обработки накопившихся команд      
    if history.size() > 0{
        history.execute();
        history.clear();
    }
} 