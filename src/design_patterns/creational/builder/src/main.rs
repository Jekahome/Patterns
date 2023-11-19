/*
Создайте объект с помощью вызовов помощника-строителя.
`Pattern Builder` — это порождающий паттерн проектирования, который позволяет создавать сложные объекты пошагово.
`Pattern Builder` даёт возможность использовать один и тот же код строительства для получения разных представлений объектов.
`Pattern Builder` особенно подходит, когда при построении T есть побочные эффекты, такие как создание потока или запуск процесса.
Полезно, когда в противном случае вам потребовалось бы много конструкторов или когда конструкция имеет побочные эффекты.

Преимущества
Отделяет методы построения от других методов.
Предотвращает распространение конструкторов.
Может использоваться для однострочной инициализации, а также для более сложной конструкции.

Этот шаблон чаще встречается в Rust (и для более простых объектов), чем во многих других языках, поскольку в Rust отсутствует перегрузка.
Поскольку у вас может быть только один метод с заданным именем, иметь несколько конструкторов в Rust менее удобно, чем в C++, Java или других.

Этот шаблон часто используется там, где объект-строитель полезен сам по себе, а не просто является строителем.
Например, см [std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html)
```
    Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")

```
*/

#[derive(Debug, PartialEq)]
pub struct TargetObject {
    // Много сложных полей.
    a: i32,
    b: i32,
    c: Option<i32>,
    d: Option<(bool, usize)>,
    sum: usize,
}

impl TargetObject {
    // Этот метод поможет пользователям найти builder
    pub fn builder() -> Builder {
        Builder::default()
    }
}

pub struct Builder {
    // Вероятно, много необязательных полей.
    a: i32,
    b: i32,
    c: Option<i32>,
    d: Option<(bool, usize)>,
}
impl Default for Builder {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: None,
            d: None,
        }
    }
}

impl Builder {
    pub fn new(a: i32, b: i32) -> Builder {
        // Установите минимально необходимые поля TargetObject.
        Builder {
            a,
            b,
            c: None,
            d: None,
        }
    }

    pub fn c(&mut self, c: i32) -> &mut Builder {
        // Задайте имя и верните построитель по значению.
        self.c = Some(c);
        self
    }
    pub fn d(&mut self, left: bool, right: usize) -> &mut Builder {
        // Задайте имя и верните построитель по значению.
        self.d = Some((left, right));
        self
    }
    // Создание TargetObject применив все настройки Builder
    pub fn build(&self) -> Result<TargetObject, String> {
        let mut sum: usize = (self.a + self.b) as usize;
        if let Some(c) = self.c {
            sum += c as usize;
        }
        if let Some(d) = self.d {
            sum = sum + d.1;
        }
        Ok(TargetObject {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            sum,
        })
    }
}

// cargo run --example p_creational_builder
fn main() {
    // без построителя
    let target = {
        let (a, b, c, d) = (1, 2, 2, 1);
        TargetObject {
            a: a,
            b: b,
            c: Some(c),
            d: Some((true, d as usize)),
            sum: (a + b + c + d) as usize,
        }
    };

    // с построителем
    let target_2: TargetObject = Builder::new(1, 2).c(2).d(true, 1).build().unwrap();
    assert_eq!(target, target_2);
}
