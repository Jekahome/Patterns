## Generic in type out

Абстрагируйтесь от конкретного типа входных данных строки, что бы увеличить гибкость API, принимая как можно больше различных типов.
  
Типаж AsRef используется в обобщённом коде для преобразования некоторого значения в ссылку на внутренние данные структуры.

```
fn take_a_str(some: impl AsRef<str>) { // тоже самое `fn take_a_str<T:AsRef<str>>(some: T) {...`
    let some = some.as_ref();
    println!("{some}");
}

use core::fmt::Debug;
fn take_a_str_into(some: impl Into<String>+Debug) {
    println!("{:?}",some);
}

fn main() {
    take_a_str("str");
    take_a_str("String".to_string());
    
    // also `&String` is supported:
    let string_ref = "StringRef".to_string();
    take_a_str(&string_ref);

    take_a_str_into("str");
    take_a_str_into("String".to_string());
    let string_ref = "StringRef".to_string();
    take_a_str_into(&string_ref);
}

```

Типаж Borrow когда цель абстрагироваться типа владения, нам нужно владеть значением или ссылаться ссылкой, нам это неважно, любой тип `T/&T/&mut T` нам подойдут

```
fn validate(email:&str)->bool{
    true
}

pub fn new<S: Into<String> + AsRef<str>>(email: S) -> Option<String> {
        if  validate(email.as_ref()) {
             Some(email.into())
        } else {
             None
        }
}

fn main() {
  new("email");
  new("email".to_string());
}
```

```
use std::borrow::Borrow;
fn check_borrow<T: Borrow<str>+std::fmt::Display>(s: T) {
    validate(s.borrow());
    print!("{}",s);
}
 
fn validate(value:&str)->bool{
    true
}

fn main() {
    check_borrow("Hello".to_string());
    check_borrow("Hello");
}
```

Ф-ция возвращает String или &str  
```
use std::borrow::Cow;
// Если входные данные будут мутироваться то вернем String иначе возвращать входную строку &str
// Т.е. клонирование при мутации Cow, отложите выделение памяти как можно дольше.

// вариант с AsRef т.е. можно String присылать
fn remove_spaces<'a>(input: &'a(impl AsRef<str> + ?Sized)) -> Cow<'a, str> { // <'a> что бы можно было вернуть  Cow<'a...
    let input = input.as_ref();
    if input.contains(' ') {
        input
        .chars()
        .filter(|&x| x != ' ')
        .collect::<std::string::String>()
        .into()
    } else {
        // input.into() // Into<Cow<'a, str>> 
        // или полный синтаксис
        Into::<Cow<'_, str>>::into(input)  
    }
} 

fn main(){
    let s = remove_spaces("Herman"); // Cow::Borrowed  
    let len = s.len(); // impl Deref
    let owned: String = s.into_owned(); // memory is allocated for a new string

    let binding = "Herman Radtke".to_string();
    let s = remove_spaces(&binding); // Cow::Owned 
    let len = s.len(); // impl Deref
    let owned: String = s.into_owned(); // no new memory allocated as we already had a String
}
```

Уменьшение раздутия кода, оптимизация
Обратной стороной этой идиомы является то, что компилятор генерирует больше кода из-за мономорфизации, что потенциально приводит к раздуванию кода.

Статическая диспетчеризация с параметрами типа имеет недостаток: генерируется довольно много кода (для каждого типа), увеличивается размер двоичного файла и потенциально пессимизируется использование кэша выполнения. Однако часто дженерики на самом деле нужны не для скорости, а для эргономики.

Каноническое решение этой проблемы — исключить внутренний метод, содержащий весь код за вычетом универсальных преобразований, и оставить внешний метод в качестве оболочки.

```
#[inline]
pub fn this<I: Into<String>>(i: I) -> usize {
    _this_inner(i.into())
}
fn _this_inner(i: String) -> usize {
    // same code as above without the conversion
}
```

[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_4_generic_in_type_out)