/*
`Template Brige` - отделение интерфейса (группа поведений) от реализации (конкретное поведение из группы)

`Template Brige` используется, чтобы избежать увеличения количества подклассов, к которому в конечном итоге могут привести механизмы наследования.
Итак, если у вас есть, скажем, 2 ортогональные обязанности, то вместо создания 2**2 подклассов вы используете композицию для объединения этих обязанностей.

Учитывая, что любое изменение, внесенное в абстракцию, повлияет на все классы, которые ее реализуют,
`Template Brige` предлагает добавить **новый уровень абстракции** между обоими элементами, который позволяет разрабатывать каждый из них независимо.


**Отношения с другими паттернами**

Основное различие между `Template Brige` и `Template Adapter` заключается в том, что `Template Adapter` используется
для унификации уже существующих интерфейсов, а `Template Brige` используется, когда есть подозрение, что реализация интерфейса со временем изменится.
Т.е. своевременное использование `Template Brige` избавит нас от необходимости внедрять `Template Adapter`
*/

/*
Проблема:
Пример, система устройств в которой используется кнопки (это и есть абстракция) для включения/выключения.
Кнопки могут быть (это первая реализация) на пружине, сенсорные, bluetooth,
также (это вторая реализация) они могут иметь не только два состояния вкл/выкл но еще нейтральное или промежуточное,
также (это третья реализация) они могут иметь применение в помещения или снаружи или в космосе, под водой ...
И мы как следсвие создаем под каждый новый тип свою абстракцию т.е. на пружине, сенсорные, bluetooth это все абстракции и про состояния и применение.
Вообшем количество расширений может расти и количество создаваемых вариантов растет в геометрической прогрессии.
Но вместо создания конкретных абстракций сдедует их выделить в абстракную группу

Кнопка может иметь различные виды поведения:
- Принцип действия: пружина, сенсор, bluetooth,...
- Состояния: вкл/выкл, нейтральное,...
- Применение: в помещения, снаружи,...
- ...
*/
#[derive(Clone, Copy)]
struct User;
impl User {
    fn verif_sensor(&self, sensor: &str) -> bool {
        true
    }
}

use btn::{Button1, Button2};
mod btn {
    use super::User;
    pub struct Button1 {
        // Кнопка на пружине с двумя состояниями
        status: bool,
        spring: bool,
    }
    impl Button1 {
        pub fn new() -> Self {
            Self {
                status: false,
                spring: false,
            }
        }
        pub fn turn_on(&mut self, _user: Option<User>) -> bool {
            if !self.spring_state() {
                self.spring_on();
                self.status = true;
                println!("Кнопка нажата");
                return true;
            }
            false
        }
        pub fn turn_off(&mut self, _user: Option<User>) -> bool {
            if self.spring_state() {
                self.spring_off();
                self.status = false;
                println!("Кнопка отжата");
                return true;
            }
            false
        }
        fn spring_state(&self) -> bool {
            if self.spring {
                return true;
            }
            false
        }
        fn spring_on(&mut self) {
            self.spring = true;
            println!("Пружина сжата");
        }
        fn spring_off(&mut self) {
            self.spring = false;
            println!("Пружина расжата");
        }
    }

    pub struct Button2 {
        // Кнопка на сенсоре с двумя состояниями
        status: bool,
        sensor: String,
    }
    impl Button2 {
        fn new(sensor: String) -> Self {
            Self {
                status: false,
                sensor,
            }
        }
        pub fn turn_on(&mut self, user: Option<User>) -> bool {
            if user.unwrap().verif_sensor(&self.sensor) {
                if !self.status {
                    self.status = true;
                    println!("Кнопка нажата");
                    return true;
                }
            }
            false
        }
        pub fn turn_off(&mut self, user: Option<User>) -> bool {
            if user.unwrap().verif_sensor(&self.sensor) {
                if self.status {
                    self.status = false;
                    println!("Кнопка отжата");
                    return true;
                }
            }
            false
        }
    }
}
// Что будет, если добавить еще один тип состония?
// Придется создавать еще две структуры для пружины и сенсора.
// И чем это мешает? Много структур?
// Да! при добавлении нового типа поведения придется реализовывать каждую комбинацию поведений
use client::Client;
mod client {
    use super::*;
    pub struct Client {
        btn: Button1,
    }
    impl Client {
        pub fn new() -> Self {
            Client {
                btn: Button1::new(),
            }
        }
        pub fn turn_on(&mut self) {
            if self.btn.turn_on(Some(User)) {
                println!("Включено\n");
            }
        }
        pub fn turn_off(&mut self) {
            if self.btn.turn_off(Some(User)) {
                println!("Выключено\n");
            }
        }
    }
}
//------------------------------------------------------------------------------------------------
// Решение - выделить возможные новые поведения в группы, эти группы будут абстракции, а их реализации - новые поведения.
// Такмим образом, группа вариантов отвечающих за внутреннее устройство - пружина или сенсор или еще что-то теперь в группе `trait OperatingPrinciple`
// а все виды поведения по стостояния кнопки теперь в группе `trait ModeState`
// И все новые их варианты не будут увеличивать количество реализаций
// И мы сможем разделить код для независимой разработки групп поведения
use btn2::*;
mod btn2 {
    use super::*;
    pub trait Button {
        fn turn_on(&mut self, user: Option<User>) -> bool;
        fn turn_off(&mut self, user: Option<User>) -> bool;
    }
    pub trait ModeState {
        fn on(&mut self);
        fn off(&mut self);
        fn state(&self) -> Option<bool>;
    }
    pub struct StateOnOff {
        state: bool,
    }
    pub enum VariantState {
        On,
        Off,
        Zero,
    }
    pub struct StateOnZeroOff {
        state: VariantState,
    }
    impl StateOnZeroOff {
        pub fn new() -> Self {
            Self {
                state: VariantState::Off,
            }
        }
    }
    impl ModeState for StateOnOff {
        fn on(&mut self) {}
        fn off(&mut self) {}
        fn state(&self) -> Option<bool> {
            Some(self.state)
        }
    }
    impl ModeState for StateOnZeroOff {
        fn on(&mut self) {}
        fn off(&mut self) {}
        fn state(&self) -> Option<bool> {
            match self.state {
                VariantState::On => Some(true),
                VariantState::Off => Some(false),
                VariantState::Zero => None,
            }
        }
    }

    pub trait OperatingPrinciple {
        fn check(&self, user: Option<&User>) -> bool;
        fn on(&mut self);
        fn off(&mut self);
    }
    pub struct Spring {
        spring: bool,
    }
    pub struct Sensor;
    impl OperatingPrinciple for Spring {
        fn check(&self, user: Option<&User>) -> bool {
            self.spring_state()
        }
        fn on(&mut self) {
            self.spring = false;
        }
        fn off(&mut self) {
            self.spring = true;
        }
    }
    impl Spring {
        fn spring_state(&self) -> bool {
            if self.spring {
                return true;
            }
            false
        }
    }
    impl OperatingPrinciple for Sensor {
        fn check(&self, user: Option<&User>) -> bool {
            if let Some(u) = user {
                return u.verif_sensor("");
            }
            false
        }
        fn on(&mut self) {}
        fn off(&mut self) {}
    }
    pub struct ButtonV2 {
        status: Box<dyn ModeState>,
        core: Box<dyn OperatingPrinciple>,
    }
    impl ButtonV2 {
        pub fn new(status: Box<dyn ModeState>, core: Box<dyn OperatingPrinciple>) -> Self {
            Self { status, core }
        }
    }

    impl Button for ButtonV2 {
        fn turn_on(&mut self, user: Option<User>) -> bool {
            if self.core.check(user.as_ref()) {
                self.core.on();
                self.status.on();
                println!("Кнопка нажата");
                return true;
            }
            false
        }
        fn turn_off(&mut self, user: Option<User>) -> bool {
            if self.core.check(user.as_ref()) {
                self.core.off();
                self.status.off();
                println!("Кнопка отжата");
                return true;
            }
            false
        }
    }
}

use client2::Client2;
mod client2 {
    use super::*;
    pub struct Client2 {
        btn: Box<dyn Button>,
    }
    impl Client2 {
        pub fn new(btn: Box<dyn Button>) -> Self {
            Self { btn }
        }
        pub fn turn_on(&mut self) {
            if self.btn.turn_on(Some(User)) {
                println!("Включено\n");
            }
        }
        pub fn turn_off(&mut self) {
            if self.btn.turn_off(Some(User)) {
                println!("Выключено\n");
            }
        }
    }
}

fn main() {
    let mut client = Client::new();
    client.turn_on();
    client.turn_off();

    println!("-----------------------");

    let mut client = Client2::new(Box::new(ButtonV2::new(
        Box::new(StateOnZeroOff::new()),
        Box::new(Sensor),
    )));
    client.turn_on();
    client.turn_off();
}
