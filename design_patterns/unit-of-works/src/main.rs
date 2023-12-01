#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Transaction;
use std::convert::TryFrom;

/*
`Unit of Work` может выполняет сразу две важные задачи:

1. Уменьшить количество запросов по сети к хранилищу данных, за счет хранение состояния обьектов в памяти.
   После завершения всех операций обновления, отправляет или нет текущее состояние обьекта в используемое хранилище (MYSQL, POSTGRES, ...)

2. Обеспечивает целосность бизнес транзакции, за счет обьединения всех операций над обьектами участниками в одну общую транзакцию.
   Транзакция либо выполнится для всех, либо произойдет откат в стабильное состояние (по аналогии с ACID - Атомарность и Согласованность).
   Механизм отката обеспечивается для баз данных поддерживающих транзакции.
   Для баз данных не поддержиющей транзакцию, придется хранить и сбрасывать состояние обьекта находящегося в памяти.

Repository - выполняет операции бизнес логики с presentation обьектами. Получает и отдает обьекты у Mapper
Mapper - отвечает за преобразование DB представление обьекта в presentation представление обьекта
Identity - отвечает за загрузку в кеш уникальных обьектов (т.е. для этого хранить весь обьект он не обязан и выполнять CREATE/UPDATE/DELETE тоже не должен, только SELECT если до этого не загрул и ответить простым bool)
UnitOfWork - 1.собирает связанные операции с обьектами в одну транзакцию и 2.посылает один запрос на сущность в базу
*/


// Реализация без поддержки состояния, реализация целосности бизнес транзакции и абстракция источника данных.
// p.s. Асинронный источник данных меняет интерфейс реализации полностью.
type ShortResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
trait UnitOfWork<'a, T> {
    fn unit_of_work<'b: 'a, U>(&'b mut self, unit: U) -> ShortResult<()>
    where
        U: FnOnce(&mut T) -> ShortResult<()>;
}

use domain_entities::{Id, Order, OrderDTO, User, UserDTO};
pub mod domain_entities {
    use super::TryFrom;

    #[derive(Debug)]
    pub struct User {
        pub id: Id,
        pub name: String,
        pub money: u64,
    }
    pub struct UserDTO<'a> {
        pub name: &'a str,
        pub money: u64,
    }

    #[derive(Debug)]
    pub struct Order {
        pub id: Id,
        pub user_id: Id,
        pub product: String,
        pub total_cost: u64,
    }
    pub struct OrderDTO<'a> {
        pub user_id: Id,
        pub product: &'a str,
        pub total_cost: u64,
    }

    #[derive(Debug)]
    pub struct Id(i64);
    impl Id {
        pub fn new(id: i64) -> Self {
            Id(id)
        }
    }
    impl TryFrom<i64> for Id {
        type Error = &'static str;

        fn try_from(value: i64) -> Result<Self, Self::Error> {
            if value == 0 {
                Err("Id only accepts value than zero!")
            } else {
                Ok(Id(value))
            }
        }
    }
    impl From<Id> for i64 {
        fn from(a: Id) -> Self {
            a.0
        }
    }
    impl PartialEq<usize> for Id {
        fn eq(&self, other: &usize) -> bool {
            self.0 as usize == *other
        }
    }
}

use uow::UnitOfWorkWithoutState;
pub mod uow {
    use super::db_layer::{self, ConcreteDb, DbContext};
    use super::{ShortResult, UnitOfWork};
    pub struct UnitOfWorkWithoutState {
        db: ConcreteDb,
        pool: Option<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>>,
    }
    impl UnitOfWorkWithoutState {
        pub fn new(db: ConcreteDb) -> Self {
            Self { db, pool: None }
        }
    }
    impl<'a> UnitOfWork<'a, db_layer::ImplTransaction<'a>> for UnitOfWorkWithoutState {
        // TODO: задача - обернуть все вызовы c общим соединением в транзакцию
        fn unit_of_work<'b: 'a, U>(&'b mut self, unit: U) -> ShortResult<()>
        where
            U: FnOnce(&mut db_layer::ImplTransaction<'a>) -> ShortResult<()>,
        {
            self.pool = Some(self.db.get_pool()?);
            if let Some(ref mut pool) = &mut self.pool {
                let mut tx/* : dyn DbContext*/ = db_layer::ImplTransaction::new(pool.transaction()?);
                unit(&mut tx)?;
                tx.commit();
            }
            Ok(())
        }
    }
}

use test_uow::{TestImplTransaction, TestUnitOfWorkWithoutState};
pub mod test_uow {
    use super::db_layer::{self, ConcreteDb, DbContext};
    use super::domain_entities::{Id, OrderDTO, User};
    use super::{ShortResult, UnitOfWork};
    use std::convert::TryFrom;
    use std::marker::PhantomData;

    pub struct TestUnitOfWorkWithoutState {
        db: Vec<u8>,
    }
    impl TestUnitOfWorkWithoutState {
        pub fn new() -> Self {
            Self { db: vec![] }
        }
    }
    impl<'a> UnitOfWork<'a, TestImplTransaction> for TestUnitOfWorkWithoutState {
        // TODO: задача - обернуть все вызовы c общим соединением в транзакцию
        fn unit_of_work<'b: 'a, U>(&'b mut self, unit: U) -> ShortResult<()>
        where
            U: FnOnce(&mut TestImplTransaction) -> ShortResult<()>,
        {
            let mut tx/* : dyn DbContext*/ = TestImplTransaction::new();
            unit(&mut tx)?;
            tx.commit();

            Ok(())
        }
    }

    pub struct TestImplTransaction {}

    impl TestImplTransaction {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl DbContext for TestImplTransaction {
        fn commit(self) {
            println!("TEST COMMIT");
        }
        fn rollback(self) {}
        fn get_user(&self, user_id: i64) -> ShortResult<User> {
            Ok(User {
                id: Id::try_from(user_id).unwrap(),
                name: "AK".into(),
                money: u64::MAX,
            })
        }
        fn update_user_money(&self, user_id: i64, total_cost: u64) -> ShortResult<()> {
            Ok(())
        }
        fn new_order(&self, order: OrderDTO) -> ShortResult<()> {
            Ok(())
        }
    }
}

use db_layer::{
    ConcreteDb, DbContext, ImplTransaction, SqliteOrderRepository, SqliteUserRepository,
};
pub mod db_layer {
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::params;

    use super::domain_entities::{Id, Order, OrderDTO, User};
    use super::{ShortResult, TryFrom};

    pub struct ConcreteDb {
        conn: r2d2::Pool<SqliteConnectionManager>,
    }
    impl ConcreteDb {
        pub fn new(conn: r2d2::Pool<SqliteConnectionManager>) -> Self {
            Self { conn }
        }
        pub fn get_pool(
            &self,
        ) -> Result<r2d2::PooledConnection<SqliteConnectionManager>, r2d2::Error> {
            self.conn.clone().get()
        }
    }
    pub trait DbContext {
        fn commit(self);
        fn rollback(self);

        fn get_user(&self, user_id: i64) -> ShortResult<User>;
        fn update_user_money(&self, user_id: i64, total_cost: u64) -> ShortResult<()>;
        fn new_order(&self, order: OrderDTO) -> ShortResult<()>;
    }

    // Все запросы к базе от всех репозиториев
    pub struct ImplTransaction<'a> {
        tx: rusqlite::Transaction<'a>,
    }
    impl<'a> ImplTransaction<'a> {
        pub fn new(tx: rusqlite::Transaction<'a>) -> Self {
            Self { tx }
        }
    }
    impl<'a> DbContext for ImplTransaction<'a> {
        fn commit(self) {
            // TODO: return Result
            let _ = self.tx.commit();
        }
        fn rollback(self) {
            // TODO: return Result
            let _ = self.tx.rollback();
        }
        fn get_user(&self, user_id: i64) -> ShortResult<User> {
            let mut stmt = self
                .tx
                .prepare("SELECT id, name, money FROM users WHERE id = ?1")
                .unwrap();
            let user = stmt.query_row([&user_id], |row| {
                Ok(User {
                    id: Id::try_from(row.get::<_, i64>(0)?).unwrap(),
                    name: row.get::<_, String>(1)?,
                    money: row.get::<_, u64>(2)?,
                })
            })?;
            Ok(user)
        }
        fn update_user_money(&self, user_id: i64, total_cost: u64) -> ShortResult<()> {
            let mut user = self.get_user(user_id)?;
            if let Some(balance) = user.money.checked_sub(total_cost) {
                user.money = balance;
            } else {
                return Err("Insufficient funds in the account".into());
            }
            let _ = self.tx.execute(
                "UPDATE users SET name = ?1, money = ?2 WHERE id=?3",
                params![&user.name, &user.money, &i64::from(user.id)],
            )?;
            Ok(())
        }
        fn new_order(&self, order: OrderDTO) -> ShortResult<()> {
            self.tx.execute(
                "INSERT INTO orders (user_id, product, total_cost) VALUES (?1, ?2, ?3)",
                (i64::from(order.user_id), order.product, order.total_cost),
            )?;
            Ok(())
        }
    }

    // Конкретные репозитории
    pub struct SqliteUserRepository<'a, T> {
        db_ctx: &'a T,
    }
    impl<'a, 'b, T: DbContext> SqliteUserRepository<'a, T> {
        pub fn new(db_ctx: &'a T) -> Self {
            Self { db_ctx }
        }
        pub fn get_user(&self, user_id: i64) -> ShortResult<User> {
            self.db_ctx.get_user(user_id)
        }
        pub fn update_user_money(&self, user_id: i64, total_cost: u64) -> ShortResult<()> {
            self.db_ctx.update_user_money(user_id, total_cost)?;
            Ok(())
        }
    }

    pub struct SqliteOrderRepository<'a, T> {
        db_ctx: &'a T,
    }
    impl<'a, 'b, T: DbContext> SqliteOrderRepository<'a, T> {
        pub fn new(db_ctx: &'a T) -> Self {
            Self { db_ctx }
        }
        pub fn new_order(&self, order: OrderDTO) -> ShortResult<()> {
            self.db_ctx.new_order(order)
        }
    }
}

fn init_schema(
    pool: r2d2::Pool<SqliteConnectionManager>,
    user_id: i64,
    name: &str,
    money: u64,
) -> ShortResult<()> {
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
            money    INTEGER NOT NULL
        )",
        params![],
    )?;

    let user = User {
        id: Id::try_from(user_id)?,
        name: name.to_owned(),
        money,
    };
    println!("-------------------------------");
    println!("Before orders: {:?}", user);
    tx.execute(
        "INSERT OR IGNORE INTO users (id, name, money) VALUES (?1, ?2, ?3)",
        (&i64::from(user.id), user.name, user.money),
    )?;
    tx.commit()?;
    Ok(())
}

fn domain_controller_new_order<'a, U, T>(
    uow: &'a mut U,
    user_id: i64,
    product: &str,
    total_cost: u64,
) -> ShortResult<()>
where
    U: UnitOfWork<'a, T>,
    T: db_layer::DbContext,
{
    let product = product.to_owned();
    // TODO: формирование единицы работы в виде замыкания FnOnce, которая должна быть по аналогии с ACID - Атомарна и Согласованна
    let _ = uow.unit_of_work(move |db_ctx: &mut T| -> ShortResult<()> {
        let rep_user: db_layer::SqliteUserRepository<'_, T> = db_layer::SqliteUserRepository::new(db_ctx);
        rep_user.update_user_money(user_id, total_cost)?;

        let user = rep_user.get_user(user_id)?;

        let order = domain_entities::OrderDTO {
            user_id: user.id,
            product: product.as_str(),
            total_cost,
        };
        let rep_order = db_layer::SqliteOrderRepository::new(db_ctx);
        rep_order.new_order(order)?;
        Ok(())
    })?;
    Ok(())
}

// cargo run --bin unit-of-works
// cargo run --bin unit-of-works --features "test"
fn main() {
    let pool: r2d2::Pool<SqliteConnectionManager> = {
        let manager = SqliteConnectionManager::file("design_patterns/unit-of-works/db/uow_without_state.db");
        r2d2::Pool::new(manager).unwrap()
    };

    // ------------------------------------------------ PREPARE --------------------------------------------------
    let user_id: i64 = 1;
    {
        let name: &str = "Alex";
        let money: u64 = 100;
        init_schema(pool.clone(), user_id, name, money).unwrap();
    }

    // ------------------------------------------------ UOW ------------------------------------------------------
    let mut uow: UnitOfWorkWithoutState =
        UnitOfWorkWithoutState::new(ConcreteDb::new(pool.clone()));

    #[cfg(feature = "test")]
    let mut uow: TestUnitOfWorkWithoutState = TestUnitOfWorkWithoutState::new();

    print!("new order - (77$) ");
    if let Err(e) = domain_controller_new_order(&mut uow, user_id, "TV Toshiba", 77) {
        println!("ERROR: {:?}", e);
    } else {
        println!("SUCCESS");
    }
    print!("new order - (3$) ");
    if let Err(e) = domain_controller_new_order(&mut uow, user_id, "Bread", 3) {
        println!("ERROR: {:?}", e);
    } else {
        println!("SUCCESS");
    }
    print!("new order - (24$) ");
    if let Err(e) = domain_controller_new_order(&mut uow, user_id, "Notepad iPad", 24) {
        println!("ERROR: {:?}", e);
    } else {
        println!("SUCCESS");
    }

    // ------------------------------------------------ SHOW -----------------------------------------------------
    show(pool, user_id);
}

fn show(pool: r2d2::Pool<SqliteConnectionManager>, user_id: i64){
    let conn = pool.get().unwrap();
 
    let mut stmt = conn
        .prepare("SELECT id, user_id, product, total_cost FROM orders WHERE user_id = ?1")
        .unwrap();
    let orders_rows = stmt
        .query_map([&user_id], |row| {
            Ok(Order {
                id: Id::try_from(row.get::<_, i64>(0)?).unwrap(),
                user_id: Id::try_from(row.get::<_, i64>(1)?).unwrap(),
                product: row.get::<_, String>(2)?,
                total_cost: row.get::<_, u64>(3)?,
            })
        })
        .unwrap();
    let orders: Vec<Order> = orders_rows.into_iter().map(|o| o.unwrap()).collect();
    println!("Orders:");
    for order in orders {
        println!("{:?}", order);
    }

    let mut stmt = conn
    .prepare("SELECT id, name, money FROM users WHERE id = ?1")
    .unwrap();
    let user = stmt.query_row([&user_id], |row| {
        Ok(User {
            id: Id::try_from(row.get::<_, i64>(0)?).unwrap(),
            name: row.get::<_, String>(1)?,
            money: row.get::<_, u64>(2)?,
        })
    });
    println!("After orders: {:?}", user.unwrap());
}
