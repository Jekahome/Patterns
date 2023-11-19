/*
`Pattern Mediator` (посредник) позволяет уменьшить связанность множества классов между собой (многие ко многим), благодаря перемещению этих связей в один класс-посредник.
Устраняет зависимости между компонентами, позволяя повторно их использовать. Упрощает взаимодействие между компонентами. Централизует управление в одном месте.

Паттерн Посредник заставляет объекты общаться не напрямую друг с другом, а через отдельный объект-посредник, который знает,
кому нужно перенаправить тот или иной запрос.
Благодаря этому, компоненты системы будут зависеть только от посредника, а не от десятков других компонентов.

Объектам больше нет нужды вызывать друг друга напрямую.
Это хорошая альтернатива `Pattern Observer`, если у вас есть “центр интеллекта” вроде контроллера (но не в смысле MVC)
Все компоненты (называемые «Коллеги») объединяются в интерфейс MediatorInterface.
Подписчики или объединенные компоненты делегируют управление медиатору.

Таким образом, посредник скрывает в себе все сложные связи и зависимости между классами отдельных компонентов программы.
А чем меньше связей имеют классы, тем проще их изменять, расширять и повторно использовать.

**Аналогия**

Пилоты садящихся или улетающих самолётов не общаются напрямую с другими пилотами.
Вместо этого они связываются с диспетчером, который координирует действия нескольких самолётов одновременно.
Без диспетчера пилотам приходилось бы все время быть начеку и следить за всеми окружающими самолётами самостоятельно, а это приводило бы к частым катастрофам в небе.
Важно понимать, что диспетчер не нужен во время всего полёта. Он задействован только в зоне аэропорта, когда нужно координировать взаимодействие многих самолётов.

**Применимость**

1. Когда вам сложно менять некоторые классы из-за того, что они имеют множество хаотичных связей с другими классами.
Посредник позволяет поместить все эти связи в один класс, после чего вам будет легче их отрефакторить, сделать более понятными и гибкими.

2. Когда вы не можете повторно использовать класс, поскольку он зависит от уймы других классов.
После применения паттерна компоненты теряют прежние связи с другими компонентами, а всё их общение происходит косвенно, через объект-посредник.

3.Когда вам приходится создавать множество подклассов компонентов, чтобы использовать одни и те же компоненты в разных контекстах.
Если раньше изменение отношений в одном компоненте могли повлечь за собой лавину изменений во всех остальных компонентах,
то теперь вам достаточно создать подкласс посредника и поменять в нём связи между компонентами.

**Эмпирические правила**

Разница между `Pattern Mediator` и `Pattern Observer` не всегда очевидна. Чаще всего они выступают как конкуренты, но иногда могут работать вместе.

Цель `Pattern Mediator` — убрать обоюдные зависимости между компонентами системы. Вместо этого они становятся зависимыми от самого посредника.
С другой стороны, цель `Pattern Observer` — обеспечить динамическую одностороннюю связь, в которой одни объекты косвенно зависят от других.
*/

/*
Типичная реализация Mediator на других языках — это классический антишаблон в Rust: многие объекты содержат изменяемые перекрестные ссылки друг на друга, пытаясь изменить друг друга,
что в Rust является смертным грехом — компилятор не пропустит ваш первый наивный код. реализация, если она не упрощена.
По определению, Медиатор ограничивает прямую связь между объектами и заставляет их сотрудничать только через объект-посредник.
Это также означает контроллер в шаблоне MVC.

Владение сверху вниз
Ключевым моментом является мышление с точки зрения СОБСТВЕННОСТИ.

1.Посредник берет на себя ответственность за все компоненты.
2.Компонент не сохраняет ссылку на посредник. Вместо этого он получает ссылку через вызов метода.
3.Поток управления начинается с main(), где посредник получает внешние события/команды.
4.Свойство медиатора для взаимодействия между компонентами — это не то же самое, что его внешний API для получения внешних событий (команд из основного цикла).
Trait Mediator взаимодействия между компонентами ( notify_about_arrival, notify_about_departure) не совпадает с его внешним API для получения внешних событий ( accept​​, depart команд из основного цикла).

https://github.com/fadeevab/mediator-pattern-rust/blob/main/README.md
*/
// A train gets a mediator object by reference.
pub trait Train {
    fn name(&self) -> &String;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}

// Nodes
use freight_train::FreightTrain;
pub mod freight_train {
    use super::Mediator;
    use super::Train;

    pub struct FreightTrain {
        name: String,
    }

    impl FreightTrain {
        pub fn new(name: &'static str) -> Self {
            Self { name: name.into() }
        }
    }

    impl Train for FreightTrain {
        fn name(&self) -> &String {
            &self.name
        }
        fn arrive(&mut self, mediator: &mut dyn Mediator) {
            if !mediator.notify_about_arrival(&self.name) {
                println!("Freight train {}: Arrival blocked, waiting", self.name);
                return;
            }
            println!("Freight train {}: Arrived", self.name);
        }
        fn depart(&mut self, mediator: &mut dyn Mediator) {
            println!("Freight train {}: Leaving", self.name);
            mediator.notify_about_departure(&self.name);
        }
    }
}
// Nodes
use passenger_train::PassengerTrain;
pub mod passenger_train {
    use super::*;

    pub struct PassengerTrain {
        name: String,
    }

    impl PassengerTrain {
        pub fn new(name: &'static str) -> Self {
            Self { name: name.into() }
        }
    }

    impl Train for PassengerTrain {
        fn name(&self) -> &String {
            &self.name
        }

        fn arrive(&mut self, mediator: &mut dyn Mediator) {
            if !mediator.notify_about_arrival(&self.name) {
                println!("Passenger train {}: Arrival blocked, waiting", self.name);
                return;
            }

            println!("Passenger train {}: Arrived", self.name);
        }

        fn depart(&mut self, mediator: &mut dyn Mediator) {
            println!("Passenger train {}: Leaving", self.name);
            mediator.notify_about_departure(&self.name);
        }
    }
}

// Mediator
use train_station::{Mediator, TrainStation};
pub mod train_station {
    use std::collections::{HashMap, VecDeque};

    use super::Train;

    // Mediator has notification methods.
    pub trait Mediator {
        fn notify_about_arrival(&mut self, train_name: &str) -> bool;
        fn notify_about_departure(&mut self, train_name: &str);
    }

    #[derive(Default)]
    pub struct TrainStation {
        trains: HashMap<String, Box<dyn Train>>,
        train_queue: VecDeque<String>,
        train_on_platform: Option<String>,
    }

    impl Mediator for TrainStation {
        fn notify_about_arrival(&mut self, train_name: &str) -> bool {
            if self.train_on_platform.is_some() {
                self.train_queue.push_back(train_name.into());
                return false;
            } else {
                self.train_on_platform.replace(train_name.into());
                return true;
            }
        }

        fn notify_about_departure(&mut self, train_name: &str) {
            if Some(train_name.into()) == self.train_on_platform {
                self.train_on_platform = None;

                if let Some(next_train_name) = self.train_queue.pop_front() {
                    let mut next_train = self.trains.remove(&next_train_name).unwrap();
                    next_train.arrive(self);
                    self.trains.insert(next_train_name.clone(), next_train);

                    self.train_on_platform = Some(next_train_name);
                }
            }
        }
    }

    impl TrainStation {
        pub fn accept(&mut self, mut train: impl Train + 'static) {
            if self.trains.contains_key(train.name()) {
                println!("{} has already arrived", train.name());
                return;
            }

            train.arrive(self);
            self.trains.insert(train.name().clone(), Box::new(train));
        }

        pub fn depart(&mut self, name: &'static str) {
            let train = self.trains.remove(name.into());
            if let Some(mut train) = train {
                train.depart(self);
            } else {
                println!("'{}' is not on the station!", name);
            }
        }
    }
}

// cargo run --example p_behavior_mediator
fn main() {
    let train1 = PassengerTrain::new("Train 1"); // Пассажирский поезд
    let train2 = FreightTrain::new("Train 2"); // Грузовой поезд

    // Station has `accept` and `depart` methods,
    // but it also implements `Mediator`.
    let mut station = TrainStation::default();

    // Станция берет на себя управление поездами.
    station.accept(train1);
    station.accept(train2);

    // `train1` and `train2` have been moved inside,
    // but we can use train names to depart them.
    station.depart("Train 1");
    station.depart("Train 2");
    station.depart("Train 3");
}
