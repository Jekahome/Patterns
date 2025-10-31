#![allow(dead_code)]
#![allow(unused_variables)]
/*
`Pattern Flyweight` (Приспособленец, Кэш, Легковес) позволяет вместить бóльшее количество объектов в отведённую оперативную память.
`Pattern Flyweight` экономит память, разделяя общее состояние объектов между собой, вместо хранения одинаковых данных в каждом объекте.
`Pattern Flyweight` позволяет экономить память, кешируя одинаковые данные, используемые в разных объектах.
`Pattern Flyweight` - это шаблон, который помогает минимизировать использование памяти за счет совместного использования и повторного использования данных.

Неизменяемые данные объекта принято называть «внутренним состоянием». Все остальные данные — это «внешнее состояние».
`Pattern Flyweight` предлагает не хранить в классе внешнее состояние, а передавать его в те или иные методы через параметры.
Таким образом, одни и те же объекты можно будет повторно использовать в различных контекстах.
Но главное — понадобится гораздо меньше объектов, ведь теперь они будут отличаться только внутренним состоянием, а оно имеет не так много вариаций.

**Структура**

Вы всегда должны помнить о том, что Легковес применяется в программе, имеющей громадное количество одинаковых объектов.
Этих объектов должно быть так много, чтобы они не помещались в доступную оперативную память без ухищрений.
Паттерн разделяет данные этих объектов на две части — легковесы и контексты.

`Pattern Flyweight` содержит состояние, которое повторялось во множестве первоначальных объектов.
Один и тот же легковес можно использовать в связке со множеством контекстов.
Состояние, которое хранится здесь, называется внутренним, а то, которое он получает извне — внешним.

Контекст содержит «внешнюю» часть состояния, уникальную для каждого объекта. Контекст связан с одним из объектов-легковесов, хранящих оставшееся состояние.

Поведение оригинального объекта чаще всего оставляют в Легковесе, передавая значения контекста через параметры методов.
Тем не менее, поведение можно поместить и в контекст, используя легковес как объект данных.

Клиент вычисляет или хранит контекст, то есть внешнее состояние легковесов.
Для клиента легковесы выглядят как шаблонные объекты, которые можно настроить во время использования, передав контекст через параметры.

Фабрика легковесов управляет созданием и повторным использованием легковесов. Фабрика получает запросы, в которых указано желаемое состояние легковеса.
Если легковес с таким состоянием уже создан, фабрика сразу его возвращает, а если нет — создаёт новый объект.

**Эмпирические правила**

`Pattern Composite` часто совмещают с `Pattern Flyweight`, чтобы реализовать общие ветки дерева и сэкономить при этом память.

`Pattern Flyweight` объясняет, когда и как можно совместно использовать объекты `Pattern State`.

*/
use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use flyweight::FlyweightTree;
pub mod flyweight {
    use super::{Any, Factory, Hash, Hasher};

    #[derive(Clone)]
    pub struct FlyweightTree {
        color: (u8, u8, u8),
        texture: Vec<u8>,
    }
    impl FlyweightTree {
        pub fn new(color: (u8, u8, u8), texture: Vec<u8>) -> Self {
            Self { color, texture }
        }
        pub fn draw(&self, canvas: &impl Any, context: (i32, i32)) {
            println!(
                "draw: {}",
                Factory::generate_key_hash(&(&self.color, &self.texture))
            );
        }
    }
    impl Hash for FlyweightTree {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.color.hash(state);
            self.texture.hash(state);
        }
    }
}

// TODO: Owner FlyweightTree
struct Factory {
    flyweight_trees: HashMap<u64, Rc<FlyweightTree>>,
}
impl Factory {
    fn new() -> Self {
        Self {
            flyweight_trees: HashMap::new(),
        }
    }
    fn get_tree_type(&mut self, color: (u8, u8, u8), texture: Vec<u8>) -> Rc<FlyweightTree> {
        let key = Factory::generate_key_hash(&(&color, &texture));
        if self.flyweight_trees.contains_key(&key) {
            return Rc::clone(&self.flyweight_trees.get(&key).unwrap());
        } else {
            let obj = Rc::new(FlyweightTree::new(color, texture));
            self.flyweight_trees.insert(key, obj);
            return Rc::clone(&self.flyweight_trees.get(&key).unwrap());
        }
    }
    fn generate_key_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

// Контекстный объект, из которого мы выделили легковес FlyweightTree
// В программе могут быть тысячи объектов Tree, так как накладные расходы на их хранение совсем небольшие — в
// памяти нужно держать всего три целых числа (две координаты и Rc ссылка).
use tree::Tree;
pub mod tree {
    use super::{Any, FlyweightTree, Rc};

    #[derive(Clone)]
    pub struct Tree {
        x: i32,
        y: i32,
        flyweight_tree: Rc<FlyweightTree>,
    }
    impl Tree {
        pub fn new(x: i32, y: i32, flyweight_tree: Rc<FlyweightTree>) -> Self {
            Self {
                x,
                y,
                flyweight_tree,
            }
        }
        pub fn draw(&self, canvas: &impl Any) {
            let context: (i32, i32) = (self.x, self.y);
            self.flyweight_tree.draw(canvas, context);
        }
    }
}

// Классы Tree и Forest являются клиентами Легковеса. При желании их можно слить в один класс, если вам не нужно
// расширять класс Tree далее.
struct Forest {
    trees: Vec<Tree>,
    factory: Factory,
}
impl Forest {
    fn new() -> Self {
        Self {
            trees: vec![],
            factory: Factory::new(),
        }
    }
    fn add_tree(&mut self, x: i32, y: i32, color: (u8, u8, u8), texture: Vec<u8>) {
        let tree_type: Rc<FlyweightTree> = self.factory.get_tree_type(color, texture);
        let tree: Tree = Tree::new(x, y, tree_type);
        self.trees.push(tree);
    }
    fn draw(&self, canvas: &impl Any) {
        for tree in self.trees.iter() {
            tree.draw(canvas);
        }
    }
}

// cargo run --bin flyweight
fn main() {
    let (x1, y1, color1, texture1) = (5, 4, (255, 0, 0), vec![1, 2, 3]);
    let (x2, y2, color2, texture2) = (1, 4, (255, 255, 0), vec![3, 3, 3]);
    let mut forest = Forest::new();
    forest.add_tree(x1, y1, color1, texture1.clone());
    forest.add_tree(x1, y1, color1, texture1.clone());
    forest.add_tree(x1, y1, color1, texture1.clone());
    forest.add_tree(x2, y2, color2, texture2.clone());
    forest.add_tree(x2, y2, color2, texture2.clone());
    forest.draw(&true);
}
