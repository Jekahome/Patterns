///Вариант основан на преобразованим From
///
/// Базовая структура
/// Содержит текущее состояние
struct StateMachine<S> {
    state: S,
}

/// Состояния
/// Следующие состояния могут быть 'S' в StateMachine<S>
struct New;
struct Unmoderated;
struct Published;
struct Deleted;

/// New -- Unmoderated
impl From<StateMachine<New>> for StateMachine<Unmoderated> {
    fn from(_val: StateMachine<New>) -> StateMachine<Unmoderated> {
        println!("New -- Unmoderated");
        StateMachine {
            state: Unmoderated {},
        }
    }
}

/// Unmoderated -- Published
impl From<StateMachine<Unmoderated>> for StateMachine<Published> {
    fn from(_val: StateMachine<Unmoderated>) -> StateMachine<Published> {
        println!("Unmoderated -- Published");
        StateMachine {
            state: Published {},
        }
    }
}

/// Unmoderated -- Deleted
impl From<StateMachine<Unmoderated>> for StateMachine<Deleted> {
    fn from(_val: StateMachine<Unmoderated>) -> StateMachine<Deleted> {
        println!("Unmoderated -- Deleted");
        StateMachine { state: Deleted {} }
    }
}

/// Published -- Deleted
impl From<StateMachine<Published>> for StateMachine<Deleted> {
    fn from(_val: StateMachine<Published>) -> StateMachine<Deleted> {
        println!("Published -- Deleted");
        StateMachine { state: Deleted {} }
    }
}

/// Вариант основан на методах структуры
/// Передача владения в методах обеспечивает продвижение состояния

/// начальное состояние
impl StateMachine<New> {
    fn new() -> Self {
        StateMachine { state: New {} }
    }
    fn publish(self) -> StateMachine<Unmoderated> {
        println!("New -- \"publish()\" --> Unmoderated");
        StateMachine {
            state: Unmoderated {},
        }
    }
}

impl StateMachine<Unmoderated> {
    fn allow(self) -> StateMachine<Published> {
        println!("Unmoderated -- \"allow()\" --> Published");
        StateMachine {
            state: Published {},
        }
    }
    fn deny(self) -> StateMachine<Deleted> {
        println!("Unmoderated -- \"deny()\" --> Deleted");
        StateMachine { state: Deleted {} }
    }
}

impl StateMachine<Published> {
    fn delete(self) -> StateMachine<Deleted> {
        println!("Published -- \"delete()\" --> Deleted");
        StateMachine { state: Deleted {} }
    }
    //Вариант для реализованного типажа From
    fn _delete(val: StateMachine<Published>) -> StateMachine<Deleted> {
        println!("Published -- \"delete()\" --> Deleted");
        val.into()
    }
}

fn main() {
    println!("Через реализацию типажа From:\n");
    //let _in_delete = StateMachine::<Published>::delete_(in_published);

    let in_new: StateMachine<New> = StateMachine::<New>::new();
    let in_unmoderated: StateMachine<Unmoderated> = StateMachine::<Unmoderated>::from(in_new);
    let in_published: StateMachine<Published> = StateMachine::<Published>::from(in_unmoderated);
    //let in_deleted:StateMachine<Deleted> = StateMachine::<Deleted>::from(in_unmoderated);
    let _in_deleted: StateMachine<Deleted> = StateMachine::<Deleted>::from(in_published);

    println!("\nЧерез методы StateMachine<S>:\n");
    let in_new: StateMachine<New> = StateMachine::<New>::new();
    let in_unmoderated: StateMachine<Unmoderated> = in_new.publish();
    let in_published: StateMachine<Published> = in_unmoderated.allow();
    //let _in_delete = in_unmoderated.deny();
    let _in_delete: StateMachine<Deleted> = in_published.delete();

    println!("\nЦепочка:\n");
    StateMachine::<New>::new().publish().allow().delete();

    println!("\nЦепочка:\n");
    StateMachine::<New>::new().publish().deny();
}
