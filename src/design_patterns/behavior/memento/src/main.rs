/*
`Pattern Memento` (`Token`) позволяет сохранять и восстанавливать прошлые состояния объектов, не раскрывая подробностей их реализации.
По мере разработки вашего приложения вы можете захотеть сохранить контрольные точки в своем приложении и позже вернуться к этим контрольным точкам.
Предоставить возможность выполнить действие отмены, чтобы восстановить объект в предыдущее состояние.

**Аналония**

Отмена действия, возврат назад или сочетание клавиш Ctrl+Z — одна из наиболее часто используемых операций в редакторе.
Для реализации операции отмены используется `Pattern Memento`. Это делается путем сохранения текущего состояния объекта по мере его изменения.

Одним из важных моментов, которые следует избегать при реализации `Pattern Memento`, является то, что инкапсуляция объекта не должна подвергаться риску.
Паттерн Снимок поручает создание копии состояния объекта самому объекту.
Паттерн предлагает держать копию состояния в специальном объекте-снимке с ограниченным интерфейсом, позволяющим, например, узнать дату изготовления или название снимка.
Но, с другой стороны, снимок должен быть открыт для своего создателя, позволяя прочесть и восстановить его внутреннее состояние.
Такая схема позволяет создателям производить снимки и отдавать их для хранения другим объектам, называемым опекунами.
Опекунам будет доступен только ограниченный интерфейс снимка, поэтому они никак не смогут повлиять на «внутренности» самого снимка.
В нужный момент опекун может попросить создателя восстановить своё состояние, передав ему соответствующий снимок.

В некоторых языках (например, PHP, Python, JavaScript) сложно гарантировать, чтобы только исходный объект имел доступ к состоянию снимка.

**Эмпирические правила**

`Pattern Command` и `Pattern Memento` можно использовать сообща для реализации отмены операций.
В этом случае объекты команд будут отвечать за выполнение действия над объектом, а снимки будут хранить резервную копию состояния этого объекта,
сделанную перед самым запуском команды.

`Pattern Memento` иногда можно заменить Прототипом, если объект, состояние которого требуется сохранять в истории, довольно простой,
не имеет активных ссылок на внешние ресурсы либо их можно легко восстановить.

**Участники:**

1. Originator (Создатель) может производить снимки своего состояния, а также воспроизводить прошлое состояние, если подать в него готовый снимок.

2. Memento (Снимок) — это простой объект данных, содержащий состояние создателя. Надёжнее всего сделать объекты снимков неизменяемыми, передавая в них состояние только через конструктор.

3. Caretaker (Смотритель/Опекун/Хранитель) должен знать, когда делать снимок создателя и когда его нужно восстанавливать.
  Опекун может хранить историю прошлых состояний создателя в виде стека из снимков.
  Когда понадобится отменить выполненную операцию, он возьмёт «верхний» снимок из стека и передаст его создателю для восстановления.

*/

use originator::{Memento, Originator, Point};
pub mod originator {
    use std::marker::PhantomData;
    pub trait Originator {
        fn generate_memento(&self) -> Box<dyn Memento>;
        fn restore_from_memento(&mut self, _: &dyn Memento);
    }

    // Concrete Originator
    #[derive(Debug)]
    pub struct Point {
        x: i32,
        y: i32,
        step: i32,
    }
    impl Point {
        pub fn new(x: i32, y: i32, step: u8) -> Self {
            Self {
                x,
                y,
                step: step as i32,
            }
        }
        pub fn up(&mut self) {
            self.y += self.step;
        }
        pub fn down(&mut self) {
            self.y -= self.step;
        }
        pub fn left(&mut self) {
            self.x -= self.step;
        }
        pub fn right(&mut self) {
            self.x += self.step;
        }
    }
    impl Originator for Point {
        fn generate_memento(&self) -> Box<dyn Memento> {
            Box::new(PointMemento(self.x, self.y, self.step))
        }
        fn restore_from_memento(&mut self, m: &dyn Memento) {
            let (x, y, step) = m.get_value(&Lock(PhantomData));
            self.x = x;
            self.y = y;
            self.step = step;
        }
    }

    // TODO: доступ к данным имееет только originator
    pub struct Lock(PhantomData<Self>);

    pub trait Memento {
        fn get_value(&self, _: &Lock) -> (i32, i32, i32);
    }

    // Concrete Memento
    struct PointMemento(i32, i32, i32);
    impl Memento for PointMemento {
        fn get_value(&self, _: &Lock) -> (i32, i32, i32) {
            (self.0, self.1, self.2)
        }
    }
}

// Смотритель/Опекун/Хранитель
use caretaker::ConcreteCaretaker;
pub mod caretaker {
    use super::originator::Memento;

    pub struct ConcreteCaretaker {
        history: Vec<Box<dyn Memento>>,
    }
    impl ConcreteCaretaker {
        pub fn new() -> ConcreteCaretaker {
            ConcreteCaretaker {
                history: Vec::new(),
            }
        }
        pub fn add_memento(&mut self, m: Box<dyn Memento>) {
            self.history.push(m)
        }
        pub fn get_memento(&mut self, index: usize) -> &dyn Memento {
            &*self.history[index]
        }
    }
}

pub fn main() {
    let mut caretaker = ConcreteCaretaker::new();

    {
        let mut originator = Point::new(10, 0, 1);
        println!("{:?}", &originator);
        caretaker.add_memento(originator.generate_memento());

        originator.up();
        println!("{:?}", &originator);
        caretaker.add_memento(originator.generate_memento());

        originator.right();
        println!("{:?}", &originator);
        caretaker.add_memento(originator.generate_memento());
    }

    println!("\n");

    let mut originator = Point::new(0, 0, 0);
    for indx in 0..3 {
        let m: &dyn Memento = caretaker.get_memento(indx);
        originator.restore_from_memento(&*m);
        println!("{:?}", originator);
    }
}
