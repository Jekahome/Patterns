##  mem::replace

Поскольку Rust подразумевает семантику перемещения по умолчанию и довольно строгие правила заимствования , часто возникают ситуации (особенно с большими structs и enums), когда изменение значения на месте или замена значений не могут быть разрешены средством проверки заимствований, что довольно сбивает с толку и приводит к к созданию ненужных клонов (что приводит к избыточным затратам на производительность).

### Завладеть данными не используя clone

Средство проверки заимствований не позволит нам извлечь из name перечисления (потому что что- то должно быть там). 
Мы, конечно, могли бы .clone() назвать и поместить клон в наш MyEnum::B, но это будет экземпляр антипаттерна «Клон для проверки заимствований». 
В любом случае, мы можем избежать лишнего распределения, изменив `e` только изменяемое заимствование.

```
use std::fmt::Debug;
use std::default::Default;
use std::mem;

#[derive(Debug,PartialEq)]
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String }
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A{ name, x:_x @ 3...7 } = e {
        // это извлекает наше имя и вставляет вместо него пустую строку 
        // (обратите внимание, что пустые строки не выделяются). 
        // Затем создаем новый вариант перечисления (который будет 
        // присваивается `* e`)
        
        *e = MyEnum::B{ name: mem::take(name) }
        //*e = MyEnum::B{ name: mem::replace(name,Default::default()) }
    }
}

fn main() {
    let mut e = MyEnum::A{name:"foo".to_owned(),x:4};
    a_to_b(&mut e);
    assert_eq!(e,MyEnum::B{name:"foo".to_owned()});
    println!("{:?}",e);
}
```

### Сохранение принадлежащих значений в измененных перечислениях

При работе с перечислениями нам может потребоваться изменить значение перечисления на месте, возможно, на другой вариант.

[Rust Design Patterns: mem::replace to keep owned values in changed enums](https://github.com/rust-unofficial/patterns/blob/main/src/idioms/mem-replace.md)


```
enum MyEnum {
    A { name: String },
    B { name: String },
}

fn swizzle(e: &mut MyEnum) {
    use self::MyEnum::*;
    *e = match *e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { ref mut name } => B { name: mem::replace(name, String::new()) },
        B { ref mut name } => A { name: mem::replace(name, String::new()) },
    }
}
```

### Обойти двойное заимствование

```
use std::default::Default;
use std::mem;
use std::collections::HashSet;

#[derive(Debug,PartialEq)]
struct Names {
    exclusions: Vec<String>,
    names: HashSet<String>,
}

impl Names {
    //fn apply_exclusions(&mut self) {
    // error: for_each уже позаимствовал self
    //    self.exclusions.drain(..).for_each(|name| {
    //        self.names.remove(&name);
    //    })
    //}
     fn apply_exclusions(&mut self) {
        let mut names = mem::replace(&mut self.names, HashSet::new());
        self.exclusions.drain(..).for_each(|name| {
            names.remove(&name);
        });
        mem::replace(&mut self.names, names);
    }
}

fn main() {
    let mut e = Names{exclusions:vec!["Einar".to_owned(), "Olaf".to_owned(), "Harald".to_owned()],
    names: [ "Einar".to_owned(), "Olaf".to_owned(), "Harald".to_owned() ].iter().cloned().collect::<HashSet<String>>()};
   
    e.apply_exclusions();
    println!("{:?}",e);
}
```

### Мутация внутренней коллекции

```
struct Names {
    exclusions: Vec<String>,
    names: HashSet<String>,
}

impl Names {
    fn apply_exclusions(&mut self) {
        self.exclusions.drain(..).for_each(|name| {
            self.names.remove(&name);
        })
    }
}
``` 

```
который не компилируется из-за двух изменяемых заимствований:

error[E0500]: closure requires unique access to `self` but it is already borrowed
  --> src/main.rs:10:44
   |
10 |         self.exclusions.drain(..).for_each(|name| {
   |         ---------------           -------- ^^^^^^ closure construction occurs here
   |         |                         |
   |         |                         first borrow later used by call
   |         borrow occurs here
11 |             self.names.remove(&name);
   |             ---- second borrow occurs due to use of `self` in closure

```

```
Использование mem::replaceздесь позволяет нам практически бесплатно избежать проблемы двух изменяемых заимствований ( HashSet::new() это без операций), заменяя значение во временной переменной:

impl Names {
    fn apply_exclusions(&mut self) {
        let mut names = mem::replace(&mut self.names, HashSet::new());
        self.exclusions.drain(..).for_each(|name| {
            names.remove(&name);
        });
        mem::replace(&mut self.names, names);
    }
}

```


[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_2_mem_replace)

[Official mem::replace docs](https://doc.rust-lang.org/std/mem/fn.replace.html)

[Official mem::swap docs](https://doc.rust-lang.org/std/mem/fn.swap.html)

[Official mem::take docs](https://doc.rust-lang.org/std/mem/fn.take.html)

[Karol Kuczmarski: Moving out of a container in Rust](http://xion.io/post/code/rust-move-out-of-container.html)