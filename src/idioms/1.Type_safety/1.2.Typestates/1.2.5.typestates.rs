 
/// Продвижение состояния обеспечивает enum

/// Текущее состояние
#[derive(Debug)]
struct StateMachine<S> {
    shared_value: bool,
    state: S
}

/// Состояния
#[derive(Debug)]
struct New;
#[derive(Debug)]
struct Unmoderated;
#[derive(Debug)]
struct Published;
#[derive(Debug)]
struct Deleted;

/// Родитель обеспечивает единственное состояние на текущий момент
#[derive(Debug)]
enum StateMachineWrapper {
    New(StateMachine<New>),
    Unmoderated(StateMachine<Unmoderated>),
    Published(StateMachine<Published>),
    Deleted(StateMachine<Deleted>),
}

#[derive(Debug)]
struct Factory {
    machine: StateMachineWrapper,
}

impl Factory {
    fn new() -> Self {
        Factory {
            machine: StateMachineWrapper::New(StateMachine::new(true)),
        }
    }
    fn get_state(&self) -> &StateMachineWrapper{
        &self.machine
    }
    fn get_state_str(&self) -> &str{
        match &self.machine {
            StateMachineWrapper::New(_) =>  "New" ,
            StateMachineWrapper::Unmoderated(_) => "Unmoderated" ,
            StateMachineWrapper::Published(_) => "Published" ,
            StateMachineWrapper::Deleted(_) => "Deleted" ,
        }
    }
    fn is_end(&self) -> bool {
        match &self.machine {
            StateMachineWrapper::Deleted(_val) => true,
            _ => false
        }
    }
}

/// Переходы состояния
impl StateMachineWrapper {
    fn step(mut self) -> Self {
        self = match self {
            StateMachineWrapper::New(val) => StateMachineWrapper::Unmoderated(val.into()),
            StateMachineWrapper::Unmoderated(val) => {
                if val.shared_value == true {
                    StateMachineWrapper::Published(val.into())
                }else{
                    StateMachineWrapper::Deleted(val.into())
                }
            },
            StateMachineWrapper::Published(val) => StateMachineWrapper::Deleted(val.into()),
            StateMachineWrapper::Deleted(val) => StateMachineWrapper::Deleted(val.into())
        };
        self
    }
    /// Изменить shared_value
    fn set_shared(&mut self,shared_value: bool){
        match self {
            StateMachineWrapper::New(val) =>{val.shared_value=shared_value; } ,
            StateMachineWrapper::Unmoderated(val) =>{val.shared_value=shared_value; } ,
            StateMachineWrapper::Published(val) => {val.shared_value=shared_value; } ,
            StateMachineWrapper::Deleted(val)  => {val.shared_value=shared_value; }
        };
    }
}

// New
impl StateMachine<New> {
    fn new(shared_value: bool) -> Self {
        StateMachine {
            shared_value: shared_value,
            state: New{}
        }
    }
}
// New -- Unmoderated
impl From<StateMachine<New>> for StateMachine<Unmoderated> {
    fn from(_val: StateMachine<New>) -> StateMachine<Unmoderated> {
        StateMachine {
            shared_value: _val.shared_value,
            state:Unmoderated{}
        }
    }
}
// Unmoderated -- Published
impl From<StateMachine<Unmoderated>> for StateMachine<Published> {
    fn from(_val: StateMachine<Unmoderated>) -> StateMachine<Published> {
        StateMachine {
            shared_value: _val.shared_value,
            state:Published{}
        }
    }
}
// Unmoderated -- Deleted
impl From<StateMachine<Unmoderated>> for StateMachine<Deleted> {
    fn from(_val: StateMachine<Unmoderated>) -> StateMachine<Deleted> {
        StateMachine {
            shared_value: _val.shared_value,
            state:Deleted{}
        }
    }
}
// Published -- Published
impl From<StateMachine<Published>> for StateMachine<Deleted> {
    fn from(_val: StateMachine<Published>) -> StateMachine<Deleted> {
        StateMachine {
            shared_value: _val.shared_value,
            state:Deleted{}
        }
    }
}

fn main(){
    let mut the_factory = Factory::new();

    println!("\nПроход №1\n");
    println!("state:{}\n",the_factory.get_state_str());// state:New
    the_factory.machine = the_factory.machine.step();
    println!("state:{}\n",the_factory.get_state_str());// state:Unmoderated
    // the_factory.machine.set_shared(false);
    the_factory.machine = the_factory.machine.step();
    println!("state:{}\n",the_factory.get_state_str());//state:Published
    the_factory.machine = the_factory.machine.step();
    println!("state:{}\n",the_factory.get_state_str());// state:Deleted
    the_factory.machine = the_factory.machine.step();
    println!("state:{}\n",the_factory.get_state_str());// state:Deleted

    println!("\nПроход №2\n");
    let mut the_factory = Factory::new();
    while the_factory.is_end() == false {
        println!("state:{}\n",the_factory.get_state_str());
        the_factory.machine = the_factory.machine.step();
    }
    println!("Step:{}\n","Deleted");
}