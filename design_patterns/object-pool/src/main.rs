/*
`Object Pool Pattern`  предоставляет метод повторного использования инициализированных объектов вместо создания новых.
Из соображений эффективности может быть весьма удобно держать наготове пул (т.е. непустой набор) инициализированных объектов.
Это может произойти, например, когда у вас есть соединения с базой данных, создание которых требует больших затрат времени и ресурсов.
Это позволяет вызвать определенный объект из пула для использования в течение определенного периода времени,
а затем вернуть его обратно в пул после выполнения задания.
Во время отсутствия этого объекта никакие другие компоненты не могут использовать его, пока он не будет возвращен обратно в пул.

Есть несколько crate, таких как [lockfree-object-pool, object-pool](https://crates.io/crates/lockfree-object-pool) и особенно [opool](https://crates.io/crates/opool), которые также реализуют и расширяют эту функциональность.

*/
use std::sync::{Arc, Mutex};

pub trait Identifiable {
    fn get_id(&self) -> i16;
}

struct ConcreteConnection {
    id: i16,
}

impl ConcreteConnection {
    fn new(value: i16) -> Self {
        ConcreteConnection { id: value }
    }
}

impl Identifiable for ConcreteConnection {
    fn get_id(&self) -> i16 {
        self.id
    }
}
impl Drop for ConcreteConnection {
    fn drop(&mut self) {
        println!("Drop ConcreteConnection id:{:?}", self.get_id())
    }
}

pub struct Pool<T> {
    pool: Arc<Mutex<Vec<T>>>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Pool {
            pool: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn add(&mut self, value: T) {
        self.pool.lock().unwrap().push(value);
    }
    pub fn release(&mut self, value: T) {
        self.pool.lock().unwrap().push(value);
    }
    pub fn get(&mut self) -> Option<T> {
        let mut pool = self.pool.lock().unwrap();
        if pool.len() > 0 {
            return pool.pop();
        }
        None
    }
}
impl<T> Drop for Pool<T> {
    fn drop(&mut self) {
        println!("Drop Pool len:{:?}", self.pool.lock().unwrap().len())
    }
}

fn main() {
    let mut pool: Pool<Box<dyn Identifiable>> = Pool::<Box<dyn Identifiable>>::new();
    pool.add(Box::new(ConcreteConnection::new(1)));
    pool.add(Box::new(ConcreteConnection::new(2)));
    pool.add(Box::new(ConcreteConnection::new(3)));

    {
        let connection = pool.get().unwrap();
        let connection2 = pool.get().unwrap();

        println!("Connection id: {}", connection.get_id());
        println!("Connection id: {}", connection2.get_id());

        pool.release(connection2);
    }
    println!("---------------------------");
}
