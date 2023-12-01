#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
/*
  Суть паттерна Unit Of Work, я считаю, тут отображена полностью.
  Бизнес транзакция выполняется целостно и поддежка состояния уменьшает количество запросов к базе данных.

  p.s. напутано с обязаностями доступа к базе данных и жесткая зависимость от sqlite.
*/
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use rusqlite::Transaction;

type ShortResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait UnitOfWork {
    fn register_new_user(&mut self, entity: UserEntityDTO);
    fn register_new_order(&mut self, entity: OrderEntityDTO);
    fn register_dirty_user(&mut self, id: i64, entity: UserEntityDTO);
    fn register_removed_user(&mut self, id: i64);
    fn commit(&mut self);
    fn clear(&mut self);
}

pub struct NewIdentityEntities {
    user_collection: Vec<UserEntityDTO>,
    order_collection: Vec<OrderEntityDTO>,
}
impl NewIdentityEntities {
    pub fn new() -> Self {
        Self {
            user_collection: vec![],
            order_collection: vec![],
        }
    }
    fn register_new_user(&mut self, entity: UserEntityDTO) {
        self.user_collection.push(entity);
    }
    fn register_new_order(&mut self, entity: OrderEntityDTO) {
        self.order_collection.push(entity);
    }
    pub fn get_users(&self) -> &Vec<UserEntityDTO> {
        &self.user_collection
    }
    pub fn get_orders(&self) -> &Vec<OrderEntityDTO> {
        &self.order_collection
    }
    pub fn clear(&mut self) {
        self.user_collection.clear();
        self.order_collection.clear();
    }
}

use uow::UnitOfWorkWithState;
pub mod uow {
    use super::db_layer;
    use super::domain_entities::{OrderEntity, OrderEntityDTO, UserEntity, UserEntityDTO};
    use super::{NewIdentityEntities, UnitOfWork};

    pub struct UnitOfWorkWithState {
        new_entities: NewIdentityEntities, // для новых сущностей
        db: db_layer::ConcreteDb,
        identity: db_layer::IdentityMap, // хранитель единственного состояния обьекта в системе
    }

    impl UnitOfWorkWithState {
        pub fn new(db: db_layer::ConcreteDb) -> Self {
            let identity = db_layer::IdentityMap::new(db.get_pool().unwrap());
            Self {
                new_entities: NewIdentityEntities::new(),
                db,
                identity,
            }
        }
        pub fn get_user_by_email(&mut self, email: &str) -> Option<UserEntity> {
            self.identity.get_user_by_email(email)
        }
        pub fn get_user(&mut self, id: i64) -> Option<UserEntity> {
            self.identity.get_user_by_id(id)
        }
    }

    impl UnitOfWork for UnitOfWorkWithState {
        fn register_new_user(&mut self, entity: UserEntityDTO) {
            self.new_entities.register_new_user(entity);
        }
        fn register_new_order(&mut self, entity: OrderEntityDTO) {
            self.new_entities.register_new_order(entity);
        }
        fn register_dirty_user(&mut self, id: i64, entity: UserEntityDTO) {
            self.identity.update_user(id, entity);
        }

        fn register_removed_user(&mut self, id: i64) {
            self.identity.removed_user(id);
        }

        fn commit(&mut self) {
            // инициируем подключение к базе данных
            // общее подключение для всех операций в одной транзакции
            if let Ok(mut binding) = self.db.get_pool() {
                let tx = binding.transaction().unwrap();

                // TODO: очередность операций лежит на вас
                // TODO: если обьект по итогу не изменился, зачем его в базе обновлять? (хранить его первоначальный хеш?)
                println!("Committing new entities:");
                let new_user_entities = self.new_entities.get_users();
                if new_user_entities.len() > 0 {
                    self.db.add_users(new_user_entities, &tx);
                }
                let new_order_entities = self.new_entities.get_orders();
                if new_order_entities.len() > 0 {
                    self.db.add_orders(new_order_entities, &tx);
                }

                println!("Committing dirty/removed entities:");
                self.identity.fixed_change_users(&tx);
                self.identity.fixed_change_orders(&tx);

                // Очистка списков изменений после коммита
                self.new_entities.clear();
                self.identity.clear();

                let _ = tx.commit().unwrap();
            } else {
                unimplemented!()
            }
        }

        fn clear(&mut self) {
            println!("clear");
            // Очистка списков
            self.new_entities.clear();
            self.identity.clear();
        }
    }
}

pub mod db_layer {
    use super::domain_entities::{OrderEntity, OrderEntityDTO, UserEntity, UserEntityDTO};
    use rusqlite::params;
    use std::collections::HashMap;

    pub enum Status {
        LOADED,
        DIRTY,
        REMOVED,
    }

    pub struct IdentityMap {
        conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
        order_identity: HashMap<i64, (OrderEntity, Status)>,
        user_identity: HashMap<i64, (UserEntity, Status)>,
    }
    impl IdentityMap {
        pub fn new(conn: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>) -> Self {
            Self {
                conn,
                order_identity: HashMap::new(),
                user_identity: HashMap::new(),
            }
        }
        pub fn get_user_by_email(&mut self, email: &str) -> Option<UserEntity> {
            if let Some(user) = self
                .user_identity
                .values()
                .into_iter()
                .find(|(user, status)| user.get_email() == email)
            {
                return Some(user.0.clone());
            }
            if self.load_user_by_email(email) {
                return self.get_user_by_email(email);
            }
            None
        }
        pub fn get_user_by_id(&mut self, id: i64) -> Option<UserEntity> {
            if self.user_identity.contains_key(&id) {
                if let Some(user) = &self.user_identity.get(&id) {
                    return Some(user.0.clone());
                }
            }
            if self.load_user(id) {
                return self.get_user_by_id(id);
            }
            None
        }
        pub fn update_user(&mut self, id: i64, user: UserEntityDTO) -> bool {
            if !self.user_identity.contains_key(&id) {
                self.load_user(id);
            }
            if self.user_identity.contains_key(&id) {
                let new_user = UserEntity::new(id, user.name, user.email, user.money);
                self.user_identity.insert(id, (new_user, Status::DIRTY));
                return true;
            }
            false
        }
        pub fn removed_user(&mut self, id: i64) {
            if self.user_identity.contains_key(&id) {
                if let Some((user, status)) = self.user_identity.get_mut(&id) {
                    *status = Status::REMOVED;
                }
            }
        }
        pub fn fixed_change_users(&mut self, tx: &rusqlite::Transaction) {
            for (key, (user, status)) in &self.user_identity {
                match *status {
                    Status::DIRTY => {
                        tx.execute(
                            "UPDATE users SET name = ?1, email = ?2, money = ?3 WHERE id=?4",
                            params![
                                user.get_name(),
                                user.get_email(),
                                user.get_money(),
                                user.get_id()
                            ],
                        )
                        .unwrap();
                    }
                    Status::REMOVED => {
                        tx.execute("DELETE FROM users WHERE id=?1", params![user.get_id()])
                            .unwrap();
                    }
                    Status::LOADED => {}
                }
            }
        }
        pub fn fixed_change_orders(&mut self, tx: &rusqlite::Transaction) {
            // unimplemented!()
        }
        fn load_user(&mut self, id: i64) -> bool {
            let mut stmt = self
                .conn
                .prepare("SELECT id, name, email, money FROM users WHERE id = ?1")
                .unwrap();
            let res = stmt.query_row([&id], |row| {
                Ok(UserEntity::new(
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, u64>(3)?,
                ))
            });
            if let Ok(user) = res {
                self.user_identity.insert(id, (user, Status::LOADED));
                return true;
            }
            false
        }
        fn load_user_by_email(&mut self, email: &str) -> bool {
            let mut stmt = self
                .conn
                .prepare("SELECT id, name, email, money FROM users WHERE email = ?1")
                .unwrap();
            let res = stmt.query_row([&email], |row| {
                Ok(UserEntity::new(
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, u64>(3)?,
                ))
            });
            if let Ok(user) = res {
                self.user_identity
                    .insert(user.get_id(), (user, Status::LOADED));
                return true;
            }
            false
        }
        pub fn clear(&mut self) {
            self.order_identity.clear();
            self.user_identity.clear();
        }
    }

    pub struct ConcreteDb {
        conn: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    }
    impl ConcreteDb {
        pub fn new(conn: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>) -> Self {
            Self { conn }
        }
        pub fn get_pool(
            &self,
        ) -> Result<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>, r2d2::Error>
        {
            self.conn.clone().get()
        }
        pub fn add_users(&self, users: &Vec<UserEntityDTO>, tx: &rusqlite::Transaction) {
            for user in users {
                tx.execute(
                    "INSERT INTO users (name, email, money) VALUES (?1, ?2, ?3)",
                    (&user.name, &user.email, &user.money),
                )
                .unwrap();
            }
            // p.s. BATCH INSERT ?
        }
        pub fn add_orders(&self, orders: &Vec<OrderEntityDTO>, tx: &rusqlite::Transaction) {
            for order in orders {
                tx.execute(
                    "INSERT INTO orders (user_id, product, total_cost) VALUES (?1, ?2, ?3)",
                    (&order.user_id, &order.product, &order.total_cost),
                )
                .unwrap();
            }
            // p.s. BATCH INSERT ?
        }
    }
}

use domain_entities::{OrderEntity, OrderEntityDTO, UserEntity, UserEntityDTO};
pub mod domain_entities {
    use std::fmt::Debug;

    #[derive(Debug, Clone)]
    pub struct UserEntity {
        id: i64,
        name: String,
        email: String,
        money: u64,
    }
    impl UserEntity {
        pub fn new(id: i64, name: String, email: String, money: u64) -> Self {
            Self {
                id,
                name,
                email,
                money,
            }
        }
        pub fn get_id(&self) -> i64 {
            self.id
        }
        pub fn get_name(&self) -> &str {
            &self.name
        }
        pub fn get_email(&self) -> &str {
            &self.email
        }
        pub fn get_money(&self) -> u64 {
            self.money
        }
    }

    pub struct UserEntityDTO {
        pub name: String,
        pub email: String,
        pub money: u64,
    }
    impl UserEntityDTO {
        pub fn new(name: &str, email: &str, money: u64) -> Self {
            Self {
                name: name.to_owned(),
                email: email.to_owned(),
                money,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct OrderEntity {
        id: i64,
        user_id: i64,
        product: String,
        total_cost: u64,
    }
    impl OrderEntity {
        pub fn new(id: i64, user_id: i64, product: String, total_cost: u64) -> Self {
            Self {
                id,
                user_id,
                product,
                total_cost,
            }
        }
    }

    pub struct OrderEntityDTO {
        pub user_id: i64,
        pub product: String,
        pub total_cost: u64,
    }
    impl OrderEntityDTO {
        pub fn new(user_id: i64, product: &str, total_cost: u64) -> Self {
            Self {
                user_id,
                product: product.to_owned(),
                total_cost,
            }
        }
    }
}

fn init_schema(pool: r2d2::Pool<SqliteConnectionManager>) -> ShortResult<()> {
    use rusqlite::params;
    let mut conn = pool.get()?;
    let tx: Transaction = conn.transaction()?;
    tx.execute(
        "CREATE TABLE IF NOT EXISTS orders (
            id    INTEGER PRIMARY KEY,
            user_id    INTEGER NOT NULL,
            product    TEXT NOT NULL,
            total_cost    INTEGER NOT NULL
        )",
        params![],
    )?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id    INTEGER PRIMARY KEY,
            name     TEXT NOT NULL,
            email    TEXT NOT NULL,
            money    INTEGER NOT NULL,
            UNIQUE(email)
        )",
        params![],
    )?;

    tx.commit()?;
    Ok(())
}

// cargo run --example uow-with-state
fn main() -> ShortResult<()> {
    // ------------------------------------------------ PREPARE --------------------------------------------------
    let pool: r2d2::Pool<SqliteConnectionManager> = {
        let manager =
            SqliteConnectionManager::file("design_patterns/unit-of-works/db/uow_with_state.db");
        r2d2::Pool::new(manager).unwrap()
    };

    let _ = init_schema(pool.clone());

    let mut uow = UnitOfWorkWithState::new(db_layer::ConcreteDb::new(pool.clone()));

    let user_id: i64 = {
        let (name, email_unique, money) = ("Alex", "Alex@email.com", 100);
        let entity = UserEntityDTO::new(name, email_unique, money);
        uow.register_new_user(entity);
        uow.commit();
        let new_user = uow.get_user_by_email(email_unique).unwrap(); // TODO: get_user не для UOW

        println!("-------------------------------");
        println!("Before orders: {:?}", new_user);
        new_user.get_id()
    };

    // ------------------------------------------------ UOW ------------------------------------------------------
    // Сценарий: Новый заказ (этот сценарий не показывает преимущество Unit Of Work с поддержкой состояния,
    //    т.е. количество запросов в базу не сокращает, но под транзакцией все же есть оптимизация в базе данных)
    // Если взять за транзакцию больше операций, то выгода от состояния появялется.

    // -- START TRANSACTION --
    for (product, total_cost) in [
        ("TV Toshiba", 77u64),
        ("Bread", 3u64),
        ("Notepad iPad", 19u64),
    ] {
        // Изменение существующей сущности User
        let current_user: UserEntity = uow.get_user(user_id).unwrap();
        if let Some(res) = current_user.get_money().checked_sub(total_cost) {
            uow.register_dirty_user(
                current_user.get_id(),
                UserEntityDTO::new(current_user.get_name(), current_user.get_email(), res),
            );
        } else {
            uow.clear();
            return Err("Insufficient funds in the account".into());
        }

        // Создание новой сущности Order
        uow.register_new_order(OrderEntityDTO::new(
            current_user.get_id(),
            product,
            total_cost,
        ));
    }
    uow.commit();
    // -- STOP TRANSACTION --

    // ------------------------------------------------ SHOW -----------------------------------------------------
    show(pool, user_id)
}

fn show(pool: r2d2::Pool<SqliteConnectionManager>, user_id: i64) -> ShortResult<()> {
    let conn = pool.get().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, user_id, product, total_cost FROM orders WHERE user_id = ?1")
        .unwrap();
    let orders_rows = stmt
        .query_map([&user_id], |row| {
            Ok(OrderEntity::new(
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, u64>(3)?,
            ))
        })
        .unwrap();
    let orders: Vec<OrderEntity> = orders_rows.into_iter().map(|o| o.unwrap()).collect();
    println!("Orders:");
    for order in orders {
        println!("{:?}", order);
    }

    let mut stmt = conn
        .prepare("SELECT id, name, email, money FROM users WHERE id = ?1")
        .unwrap();
    let user = stmt.query_row([&user_id], |row| {
        Ok(UserEntity::new(
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, u64>(3)?,
        ))
    });
    println!("After orders: {:?}", user.unwrap());

    Ok(())
}
