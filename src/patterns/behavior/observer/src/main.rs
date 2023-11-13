/*
`Pattern Observer` - полезен, когда вас интересует состояние объекта и вы хотите получать уведомления о любых изменениях.
Создаёт механизм подписки, позволяющий одним объектам следить и реагировать на события, происходящие в других объектах.

В `Pattern Observer` объект, который наблюдает за состоянием другого объекта, называется Observer,
а объект, за которым ведется наблюдение, называется Subject.

Определите зависимость «один ко многим» между объектами, чтобы при изменении состояния одного объекта
все его зависимые объекты автоматически уведомлялись и обновлялись.

Когда после изменения состояния одного объекта требуется что-то сделать в других, но вы не знаете наперёд, какие именно объекты должны отреагировать.

**Проблема**

Представьте, что вы имеете два объекта: Покупатель и Магазин. В магазин вот-вот должны завезти новый товар, который интересен покупателю.
Покупатель может каждый день ходить в магазин, чтобы проверить наличие товара. Но при этом он будет злиться, без толку тратя своё драгоценное время.
С другой стороны, магазин может разослать спам каждому своему покупателю. Многих это расстроит, так как товар специфический, и не всем он нужен.
Получается конфликт: либо покупатель тратит время на периодические проверки, либо магазин тратит ресурсы на бесполезные оповещения.


Паттерн Наблюдатель предлагает хранить внутри объекта издателя `Observer` список ссылок на объекты подписчиков `Subject`,
причём издатель не должен вести список подписки самостоятельно.
Он предоставит методы, с помощью которых подписчики могли бы добавлять или убирать себя из списка.
Когда в издателе будет происходить важное событие, он будет проходиться по списку подписчиков и
оповещать их об этом, вызывая определённый метод объектов-подписчиков.
Издателю безразлично, какой класс будет иметь тот или иной подписчик, так как все они должны следовать общему интерфейсу и иметь единый метод оповещения.
*/

// Subject manages all Events
// Observer is called from Subject when Event occurs

// Event
trait Event {
    fn get_title(&self) -> &str;
}

#[derive(Debug, Clone)]
struct EventObject {
    title: String,
}

impl EventObject {
    fn new(title: String) -> EventObject {
        EventObject { title }
    }
}

impl Event for EventObject {
    fn get_title(&self) -> &str {
        &self.title
    }
}

// Subject
trait Subject<T: Clone> {
    fn notify_observers(&self, event: &T);
    fn register_observer(&mut self, observer: Box<dyn Observer<T>>) -> usize;
    fn unregister_observer(&mut self, index: usize);
}

struct ConcreteSubject {
    observers: Vec<(bool, Box<dyn Observer<EventObject>>)>,
}

impl ConcreteSubject {
    fn new() -> ConcreteSubject {
        ConcreteSubject {
            observers: Vec::new(),
        }
    }
}

impl Subject<EventObject> for ConcreteSubject {
    fn notify_observers(&self, event: &EventObject) {
        for observer in &self.observers {
            if observer.0 {
                observer.1.on_notify(event);
            }
        }
    }

    fn register_observer(&mut self, observer: Box<dyn Observer<EventObject>>) -> usize {
        self.observers.push((true, observer));
        self.observers.len() - 1
    }

    fn unregister_observer(&mut self, index: usize) {
        self.observers[index].0 = false;
    }
}

// Observer
trait Observer<T: Clone> {
    fn on_notify(&self, event: &T);
}

struct ConcreteObserver(usize);

impl Observer<EventObject> for ConcreteObserver {
    fn on_notify(&self, event: &EventObject) {
        println!(
            "ConcreteObserver {} gets event: [{}]",
            self.0,
            event.get_title()
        );
    }
}

fn main() {
    let mut subject = ConcreteSubject::new();
    subject.register_observer(Box::new(ConcreteObserver(1)));
    subject.register_observer(Box::new(ConcreteObserver(2)));
    subject.register_observer(Box::new(ConcreteObserver(3)));

    let event1 = EventObject::new(String::from("first event"));
    let event2 = EventObject::new(String::from("second event"));

    subject.notify_observers(&event1);
    subject.notify_observers(&event2);
}