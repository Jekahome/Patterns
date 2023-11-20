/*
Код с проблемами... 
Пользовательский слой (интерфейс) жестко зависит от бизнес слоя, напрямую используя его API (add_str,read_str). 
При изменении бизнес логики потребуется все вызовы в интерфейсе менять.
Так же, нет возможности фильтровать запросы или выполнять с задержкой, смотреть историю и воспроизводить последовательность дейсвий.
Вообщем, нет сохраненной истории запросов.
*/
use std::fmt::Display;
use std::env;
 
pub mod business_layer {
    use std::io::{Write,Read};
    use std::fs::OpenOptions;
    use std::path::Path;
    use super::*;

    pub fn add_str<M: AsRef<str>, P: AsRef<Path> + Display>(msg: M, file: P) -> std::io::Result<()> {
        let display = format!("{}",file.as_ref().display());
        let path = Path::new(&display);
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        f.write_fmt(format_args!("{}\n",msg.as_ref()))?; 
        Ok(())
    }
    pub fn read_str<P: AsRef<Path>>(file: P) -> std::io::Result<()> {
        let display = format!("{}",file.as_ref().display());
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

// cargo run --bin command add 'msg of args' data.txt -- --bin main_problem  
// cargo run --bin command read data.txt -- --bin main_problem  
fn args_user_layer(args:Vec<String>) -> bool{
    if args.len() > 3{
        let action = &args[1];
        match action.as_str() {
            "add" => {
                let value = &args[2];
                let file = &args[3];
                let _ = business_layer::add_str(value,file);
                return true;
            },
            _ => {return false;}
        }
    } else if args.len() > 2 {
        let action = &args[1];
        match action.as_str() {
            "read" => {
                let file = &args[2];
                let _ = business_layer::read_str(file);
                return true;
            },
            _ => {return false;}
        }
    }
    false
}

// ENV_VAR_ACTION="add" ENV_VAR_VALUE='msg of env' ENV_VAR_FILE='data.txt' cargo run --bin command -- --bin main_problem 
// ENV_VAR_ACTION="read" ENV_VAR_FILE='data.txt' cargo run --bin command -- --bin main_problem
fn env_user_layer() -> bool{
    if let Ok(action) = env::var("ENV_VAR_ACTION"){
        match action.as_str() {
            "read" => {
                if let Ok(file) = env::var("ENV_VAR_FILE"){
                    let _ = business_layer::read_str(file);
                    return true;
                } 
            },
            "add" => {
                if let Ok(value) = env::var("ENV_VAR_VALUE"){
                    if let Ok(file) = env::var("ENV_VAR_FILE"){
                        let _ = business_layer::add_str(value,file);
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
    env_user_layer();

    let args: Vec<String> = env::args().collect();
    args_user_layer(args);
}