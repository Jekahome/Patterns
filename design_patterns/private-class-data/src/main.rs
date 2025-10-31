#![allow(dead_code)]

/*
`Pattern Private Class Data` - инкапсуляция аттрибутов класса.

**Намерение**

1. Управление доступом на запись к атрибутам класса
2. Отделяйте данные от методов, которые их используют.
3. Инкапсулировать инициализацию данных класса.

Мотивация для этого шаблона проектирования исходит из цели проектирования — защитить состояние класса путем минимизации видимости его атрибутов (данных).

Шаблон проектирования данных частного класса направлен на уменьшение раскрытия атрибутов за счет ограничения их видимости.

Это уменьшает количество атрибутов класса, инкапсулируя их в один объект данных.
Это позволяет разработчику класса лишить права записи атрибутов, которые предназначены для установки только во время создания, даже из методов целевого класса.

P.S.
В Rust'е есть возможность "запечатать" [Sealing](https://github.com/Jekahome/Patterns/tree/main/idioms/6.Sealing) реализацию.
*/

use private_samples::Circle;

mod private_samples {

    use private_class_data::CircleSpecs;
    mod private_class_data {
        pub(in crate::private_samples) struct CircleSpecs {
            radius: f64,
            point: String,
        }
        impl CircleSpecs {
            pub fn new(radius: f64, point: String) -> Self {
                Self { radius, point }
            }
            pub fn radius(&self) -> f64 {
                self.radius
            }
            pub fn point(&self) -> &str {
                self.point.as_str()
            }
        }
    }
    // TODO: Circle не сможет получить доступ к приватным полям CircleSpecs и изменить его состояние после создания
    pub struct Circle {
        circle_specs: CircleSpecs,
    }
    impl Circle {
        pub fn new(radius: f64, point: String) -> Self {
            Self {
                circle_specs: CircleSpecs::new(radius, point),
            }
        }
        pub fn circumference(&self) -> f64 {
            self.circle_specs.radius() * std::f64::consts::PI
        }
        pub fn diameter(&self) -> f64 {
            self.circle_specs.radius() * 2.0
        }
    }
}

// cargo run --bin private-class-data
fn main() {
    let c = Circle::new(4.0, "center".into());
    println!("circumference: {:.2}", c.circumference());
    println!("diameter: {}", c.diameter());
}
