#![allow(unused_variables)]
#![allow(dead_code)]

/*
`Pattern Prototype` инкапсуляция создания клона

Проблема
У вас есть объект, который нужно скопировать.
Как это сделать? Нужно создать пустой объект такого же класса, а затем поочерёдно скопировать значения всех полей из старого объекта в новый.
Но у нас может не быть доступа к приватных полям. И копирующий код становиться жество зависим от конкретного копируемого класса.

`Pattern Prototype` хочет что бы обьекты создавали свои копии самостоятельно и использовали эдиный интерфейс `method clone()`
Объект, который копируют, называется прототипом (откуда и название паттерна)
Также есть место для инкапсуляции логики создания клона

Если создание объекта требует много времени и средств, и у вас уже есть наиболее похожий экземпляр объекта,
тогда вы клонируете уже приготовленный с его текущим состоянием.

Если вам нужна глубокая копия, вы можете использовать сериализацию в качестве хитрости, чтобы выполнить глубокую копию.
*/
use cell::{Prototype, SomeCell};
use std::fmt::Debug;
mod cell {
    use std::cmp::PartialEq;
    use std::fmt::Debug;
    use std::time::{SystemTime, UNIX_EPOCH};

    pub trait Prototype: Clone {}

    pub struct SomeCell {
        pub cytoplasm: Vec<u8>,
        pub mitochondria: Vec<u8>,
        pub chloroplasts: Vec<u8>,
        secret: u128,
    }
    impl Clone for SomeCell {
        fn clone(&self) -> Self {
            let _clone @ SomeCell {
                cytoplasm,
                mitochondria,
                chloroplasts,
                ..
            } = self;
            let mut cytoplasm_cap: Vec<u8> = Vec::with_capacity(cytoplasm.capacity());
            let mut mitochondria_cap: Vec<u8> = Vec::with_capacity(mitochondria.capacity());
            let mut chloroplasts_cap: Vec<u8> = Vec::with_capacity(chloroplasts.capacity());
            cytoplasm_cap.clone_from(&cytoplasm);
            mitochondria_cap.clone_from(&mitochondria);
            chloroplasts_cap.clone_from(&chloroplasts);
            Self {
                cytoplasm: cytoplasm_cap,
                mitochondria: mitochondria_cap,
                chloroplasts: chloroplasts_cap,
                secret: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            }
        }
    }
    impl Prototype for SomeCell {}
    impl Debug for SomeCell {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "SomeCell {{ cytoplasm: {:?}, mitochondria: {:?}, chloroplasts: {:?} }}",
                self.cytoplasm, self.mitochondria, self.chloroplasts
            )
        }
    }
    impl<'a> PartialEq<SomeCell> for &'a SomeCell {
        fn eq(&self, other: &SomeCell) -> bool {
            let _clone @ SomeCell {
                cytoplasm,
                mitochondria,
                chloroplasts,
                ..
            } = other;
            self.cytoplasm == *cytoplasm
                && self.mitochondria == *mitochondria
                && self.chloroplasts == *chloroplasts
        }
    }

    impl<'a> PartialEq<SomeCell> for SomeCell {
        fn eq(&self, other: &SomeCell) -> bool {
            let _clone @ SomeCell {
                cytoplasm,
                mitochondria,
                chloroplasts,
                ..
            } = other;
            self.cytoplasm == *cytoplasm
                && self.mitochondria == *mitochondria
                && self.chloroplasts == *chloroplasts
        }
    }
    impl Default for SomeCell {
        fn default() -> Self {
            Self {
                cytoplasm: Vec::default(),
                mitochondria: Vec::default(),
                chloroplasts: Vec::default(),
                secret: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            }
        }
    }
    impl SomeCell {
        pub fn new() -> Self {
            Self::default()
        }
    }
}

fn client<T: Prototype + Debug>(cell: &T)
where
    for<'a> &'a T: PartialEq<T>,
{
    let clone_cell_2 = cell.clone();
    assert_eq!(cell, clone_cell_2)
}
// cargo run --example p_creational_prototype
fn main() {
    let mut clone_cell_1 = SomeCell::new();
    clone_cell_1.cytoplasm = vec![1, 2, 3]; // продолжить настраивать с текущего состояния
    client(&clone_cell_1);
}
