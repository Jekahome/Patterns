/*
`Pattern Chain Of Responsibilities` используется для достижения слабой связи отправителя запроса с получателем. Делегировать задачу следующему.

Позволяет передавать запросы последовательно по цепочке обработчиков.
Каждый последующий обработчик решает, может ли он обработать запрос сам и стоит ли передавать запрос дальше по цепи.
Механизм цепочки использует рекурсивную композицию, позволяющую связывать неограниченное количество обработчиков.

`Pattern Chain Of Responsibilities` позволяет избежать связывания отправителя запроса с получателем, предоставляя более чем одному объекту возможность обработать запрос.

По сути это набор обработчиков, которые по очереди получают запрос, а затем решают обрабатывать его или нет.
Если запрос не обработан, то он передается дальше по цепочке.
Если же он обработан, то паттерн сам решает передавать его дальше или нет.
middleware

Необходимость эффективной обработки запросов без жесткого связывания отношений и приоритета обработчиков или сопоставлений запросов с обработчиками.

Паттерн предлагает связать объекты обработчиков в одну цепь.
Каждый из них будет иметь ссылку на следующий обработчик в цепи.
Таким образом, при получении запроса обработчик сможет не только сам что-то с ним сделать, но и передать обработку следующему объекту в цепочке.

Не используйте `Pattern Chain Of Responsibilities`, если каждый запрос обрабатывается только одним обработчиком или когда клиентский объект знает,
какой объект службы должен обрабатывать запрос.


**Применимость**

1. Когда программа должна обрабатывать разнообразные запросы несколькими способами, но заранее неизвестно, какие конкретно запросы будут приходить и какие обработчики для них понадобятся.
   С помощью `Pattern Chain Of Responsibilities` вы можете связать потенциальных обработчиков в одну цепь и при получении запроса поочерёдно спрашивать каждого из них, не хочет ли он обработать запрос.

2. Когда важно, чтобы обработчики выполнялись один за другим в строгом порядке.
   `Pattern Chain Of Responsibilities` позволяет запускать обработчиков последовательно один за другим в том порядке, в котором они находятся в цепочке.

3. Когда набор объектов, способных обработать запрос, должен задаваться динамически.
   В любой момент вы можете вмешаться в существующую цепочку и переназначить связи так, чтобы убрать или добавить новое звено.

4. Объектно-ориентированный связанный список с рекурсивным обходом.

**Аналогия**

Банкоматы используют цепочку ответственности в механизме выдачи денег.
Для сдачи подойдут все купюры из которых возможно составить сумму и они есть в наличии.

Эмпирические правила

`Pattern Chain Of Responsibilities` часто используют вместе с Компоновщиком. В этом случае запрос передаётся от дочерних компонентов к их родителям.

Обработчики в `Pattern Chain Of Responsibilities` могут быть выполнены в виде `Pattern Command`. В этом случае множество разных операций может быть выполнено над одним и тем же контекстом, коим является запрос.
Но есть и другой подход, в котором сам запрос является  `Pattern Command`, посланной по цепочке объектов. В этом случае одна и та же операция может быть выполнена над множеством разных контекстов, представленных в виде цепочки.

`Pattern Chain Of Responsibilities` и `Pattern Decorator` имеют очень похожие структуры.
Оба паттерна базируются на принципе рекурсивного выполнения операции через серию связанных объектов. Но `Pattern Decorator` не прерывает ход выполнения.
*/

// обработка пациента через цепочку отделений
// Patient -> Reception -> Doctor -> Medical -> Cashier

use request::Patient;
pub mod request {
    #[derive(Default)]
    pub struct Patient {
        pub name: String,
        pub registration_done: bool,
        pub doctor_check_up_done: bool,
        pub medicine_done: bool,
        pub payment_done: bool,
    }
}

use handlers::*;
pub mod handlers {
    use super::*;

    /// A single role of objects that make up a chain.
    /// A typical trait implementation must have `handle` and `next` methods,
    /// while `execute` is implemented by default and contains a proper chaining
    /// logic.
    pub trait Department {
        fn execute(&mut self, patient: &mut Patient) {
            self.handle(patient);

            if let Some(next) = &mut self.next() {
                next.execute(patient);
            }
        }

        fn handle(&mut self, patient: &mut Patient);
        fn next(&mut self) -> &mut Option<Box<dyn Department>>;
    }

    /// Helps to wrap an object into a boxed type.
    pub(crate) fn into_next(
        department: impl Department + Sized + 'static,
    ) -> Option<Box<dyn Department>> {
        Some(Box::new(department))
    }
}

use cashier::*;
pub mod cashier {
    use super::*;

    #[derive(Default)]
    pub struct Cashier {
        next: Option<Box<dyn Department>>,
    }

    impl Department for Cashier {
        fn handle(&mut self, patient: &mut Patient) {
            if patient.payment_done {
                println!("Payment done");
            } else {
                println!("Cashier getting money from a patient {}", patient.name);
                patient.payment_done = true;
            }
        }

        fn next(&mut self) -> &mut Option<Box<dyn Department>> {
            &mut self.next
        }
    }
}

use doctor::*;
pub mod doctor {
    use super::*;

    pub struct Doctor {
        next: Option<Box<dyn Department>>,
    }

    impl Doctor {
        pub fn new(next: impl Department + 'static) -> Self {
            Self {
                next: into_next(next),
            }
        }
    }

    impl Department for Doctor {
        fn handle(&mut self, patient: &mut Patient) {
            if patient.doctor_check_up_done {
                println!("A doctor checkup is already done");
            } else {
                println!("Doctor checking a patient {}", patient.name);
                patient.doctor_check_up_done = true;
            }
        }

        fn next(&mut self) -> &mut Option<Box<dyn Department>> {
            &mut self.next
        }
    }
}

use medical::*;
pub mod medical {
    use super::*;

    pub struct Medical {
        next: Option<Box<dyn Department>>,
    }

    impl Medical {
        pub fn new(next: impl Department + 'static) -> Self {
            Self {
                next: into_next(next),
            }
        }
    }

    impl Department for Medical {
        fn handle(&mut self, patient: &mut Patient) {
            if patient.medicine_done {
                println!("Medicine is already given to a patient");
            } else {
                println!("Medical giving medicine to a patient {}", patient.name);
                patient.medicine_done = true;
            }
        }

        fn next(&mut self) -> &mut Option<Box<dyn Department>> {
            &mut self.next
        }
    }
}

use reception::*;
pub mod reception {
    use super::*;

    #[derive(Default)]
    pub struct Reception {
        next: Option<Box<dyn Department>>,
    }

    impl Reception {
        pub fn new(next: impl Department + 'static) -> Self {
            Self {
                next: into_next(next),
            }
        }
    }

    impl Department for Reception {
        fn handle(&mut self, patient: &mut Patient) {
            if patient.registration_done {
                println!("Patient registration is already done");
            } else {
                println!("Reception registering a patient {}", patient.name);
                patient.registration_done = true;
            }
        }

        fn next(&mut self) -> &mut Option<Box<dyn Department>> {
            &mut self.next
        }
    }
}

// cargo run --example p_behavior_chain_of_responsibility
fn main() {
    let cashier = Cashier::default();
    let medical = Medical::new(cashier);
    let doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);

    let mut patient = Patient {
        name: "John".into(),
        ..Patient::default()
    };

    // Reception handles a patient passing him to the next link in the chain.
    // Reception -> Doctor -> Medical -> Cashier.
    reception.execute(&mut patient);

    println!("\nThe patient has been already handled:\n");

    reception.execute(&mut patient);
}
