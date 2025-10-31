#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

/*
Репозиторий - это абстракция бизнес модели над даееыми из которых она состоит. Реализация репозитория инкапсулирует доступ к данным.
Бизнес логика работает с абстракным репозиторием, поэтому бизнес логика остается персистентной
т.е. постоянной и не подвергается изменениям если источник данных будет изменятся.

Шаблон репозитория можно реализовать следующими способами:

    1.Один репозиторий на объект (не универсальный).
    Этот тип реализации предполагает использование одного класса репозитория для каждого объекта.
    Например, если у вас есть две сущности «Заказ» и «Клиент», каждая сущность будет иметь свой собственный репозиторий.

    2.Общий репозиторий. Общий репозиторий — это тот, который можно использовать для всех объектов,
    другими словами, его можно использовать либо для Заказа, либо для Клиента, либо для любого другого объекта.

*/
use std::convert::TryInto;
type ShortResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait UserRepository {
    fn create(&mut self, name: &str, money: u64) -> ShortResult<Id>;
    fn read(&mut self, id: Id) -> ShortResult<()>;
    fn update(&mut self, id: Id, user: UserDTO) -> ShortResult<()>;
    fn delete(&mut self, id: Id) -> ShortResult<()>;
}

use domain::*;
pub mod domain {
    use std::convert::TryFrom;
    use std::fmt;
    use std::str::FromStr;
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
    impl fmt::Display for Id {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl FromStr for Id {
        type Err = std::num::ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let n = s.parse::<i64>()?;
            Ok(Id(n))
        }
    }

    pub struct User {
        pub id: Id,
        pub name: String,
        pub money: u64,
    }
    impl User {
        pub fn new(name: &str, money: u64) -> Self {
            Self {
                id: Id(0),
                name: name.to_owned(),
                money,
            }
        }
    }
    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "(User)\nid: {} \nname: {} \nmoney:{}",
                self.id, self.name, self.money
            )
        }
    }
    impl From<User> for String {
        fn from(user: User) -> Self {
            format!("{}|{}|{}\n", user.id, user.name, user.money)
        }
    }
    impl TryFrom<&str> for User {
        type Error = &'static str;
        fn try_from(s: &str) -> Result<Self, Self::Error> {
            let v: Vec<&str> = s.split("|").collect();
            if v.len() == 3 {
                if let Ok(id) = v[0].parse::<i64>() {
                    if let Ok(user_id) = Id::try_from(id) {
                        if let Ok(money) = v[2].parse::<u64>() {
                            let mut user = User::new(v[1], money);
                            user.id = user_id;
                            return Ok(user);
                        }
                    }
                }
            }
            Err("invalid data")
        }
    }
}

pub struct UserDTO {
    pub name: Option<String>,
    pub money: Option<u64>,
}

use db_file::FileUserRepository;
pub mod db_file {
    use super::domain::{Id, User};
    use super::ShortResult;
    use super::UserDTO;
    use super::UserRepository;
    use std::convert::{TryFrom, TryInto};
    use std::fs::{File, OpenOptions};
    use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};

    pub struct FileUserRepository {
        dir: PathBuf,
        users: PathBuf,
        last_id: i64,
    }
    impl FileUserRepository {
        pub fn new(dir: &(impl AsRef<Path> + ?Sized)) -> std::io::Result<Self> {
            let path: &Path = dir.as_ref();
            let mut last_id = 0i64;
            if !path.exists() {
                std::fs::create_dir(path)?;
                std::fs::write(path.join("users"), b"0\n")?;
            } else if !path.join("users").exists() {
                std::fs::write(path.join("users"), b"0\n")?;
            } else {
                let f = File::open(path.join("users"))?;
                let lines = BufReader::new(f).lines();
                if let Some(Ok(res)) = lines.take(1).next() {
                    if let Ok(id) = res.parse::<i64>() {
                        last_id = id;
                    } else {
                        std::fs::write(path.join("users"), b"0\n")?;
                    }
                } else {
                    std::fs::write(path.join("users"), b"0\n")?;
                }
            }
            Ok(Self {
                dir: path.to_path_buf(),
                users: path.join("users"),
                last_id,
            })
        }
    }

    impl UserRepository for FileUserRepository {
        fn create(&mut self, name: &str, money: u64) -> ShortResult<Id> {
            let mut user = User::new(name, money);
            self.last_id = self.last_id + 1;
            user.id = self.last_id.try_into()?;
            let mut f = OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.users.as_path())?;

            let s: String = user.into();
            let data = s.as_bytes();

            f.write(data)?;
            f.sync_all()?;

            let mut f = OpenOptions::new().write(true).open(self.users.as_path())?;
            f.seek(SeekFrom::Start(0))?;
            f.write(format!("{}\n", self.last_id).as_bytes())?;
            f.seek(SeekFrom::End(1))?;

            Ok(self.last_id.try_into()?)
        }
        fn read(&mut self, id: Id) -> ShortResult<()> {
            let find_user_id: usize = Into::<i64>::into(id) as usize;
            let f = File::open(self.users.as_path())?;
            let lines = BufReader::new(f).lines();

            for line in lines.skip(1) {
                let line = line?;
                if let Ok(ref user) = User::try_from(line.as_str()) {
                    if user.id == find_user_id {
                        println!("{}", user);
                    }
                }
            }
            Ok(())
        }
        fn update(&mut self, id: Id, user_dto: UserDTO) -> ShortResult<()> {
            let find_user_id: usize = Into::<i64>::into(id) as usize;
            let f = File::open(self.users.as_path())?;
            let lines = BufReader::new(f).lines();
            {
                let mut buffer = "".to_string();
                for line in lines.map(|mut s| {
                    if let Ok(ref mut l) = s {
                        l.push('\n');
                    }
                    s
                }) {
                    let line = line?;
                    if let Ok(mut user) = User::try_from(line.as_str()) {
                        if user.id == find_user_id {
                            if let Some(ref name) = user_dto.name {
                                user.name = name.clone();
                            }
                            if let Some(money) = user_dto.money {
                                user.money = money;
                            }

                            let s: String = user.into();
                            buffer.push_str(s.as_str());
                        } else {
                            buffer.push_str(line.as_str());
                        }
                    } else {
                        buffer.push_str(line.as_str());
                    }
                }
                std::fs::write(self.users.as_path(), buffer.as_bytes())?;
            }

            Ok(())
        }
        fn delete(&mut self, id: Id) -> ShortResult<()> {
            let find_user_id: usize = i64::from(id) as usize;
            let f = File::open(self.users.as_path())?;

            let lines = BufReader::new(f).lines();
            {
                let mut buffer = "".to_string();
                for line in lines.map(|mut s| {
                    if let Ok(ref mut l) = s {
                        l.push('\n');
                    }
                    s
                }) {
                    let line = line?;
                    if let Ok(user) = User::try_from(line.as_str()) {
                        if user.id != find_user_id {
                            buffer.push_str(line.as_str());
                        }
                    } else {
                        buffer.push_str(line.as_str());
                    }
                }
                std::fs::write(self.users.as_path(), buffer.as_bytes())?;
            }
            Ok(())
        }
    }
}

use db_sqlite::SqliteUserRepository;
pub mod db_sqlite {
    use super::domain::{Id, User};
    use super::ShortResult;
    use super::UserDTO;
    use super::UserRepository;
    use rusqlite::{params, Connection, Result};
    use std::convert::TryInto;

    pub struct SqliteUserRepository {
        conn: Connection,
    }
    impl SqliteUserRepository {
        pub fn new() -> Result<Self> {
            let conn = Connection::open_in_memory()?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS users (
                    id    INTEGER PRIMARY KEY,
                    name  TEXT NOT NULL,
                    money INTEGER
                )",
                (), // empty list of parameters.
            )?;
            Ok(Self { conn })
        }
    }
    impl UserRepository for SqliteUserRepository {
        fn create(&mut self, name: &str, money: u64) -> ShortResult<Id> {
            self.conn.execute(
                "INSERT INTO users (name, money) VALUES (?1, ?2)",
                (&name, &money),
            )?;
            let id = self.conn.last_insert_rowid();
            Ok(id.try_into()?)
        }
        fn read(&mut self, id: Id) -> ShortResult<()> {
            let mut stmt = self
                .conn
                .prepare("SELECT id, name, money FROM users WHERE id = ?1")
                .unwrap();
            let user = stmt.query_row([&i64::from(id)], |row| {
                Ok(User {
                    id: Id::new(row.get(0)?),
                    name: row.get(1)?,
                    money: row.get(2)?,
                })
            })?;
            println!("{}", user);
            Ok(())
        }
        fn update(&mut self, id: Id, user: UserDTO) -> ShortResult<()> {
            let id = i64::from(id);
            if let (Some(name), Some(money)) = (user.name.clone(), user.money) {
                let _ = self.conn.execute(
                    "UPDATE users SET name = ?,money - ? WHERE id=?",
                    params![&name, &money, &id],
                )?;
            } else if let Some(name) = user.name {
                let _ = self
                    .conn
                    .execute("UPDATE users SET name = ? WHERE id=?", params![&name, &id])?;
            } else if let Some(money) = user.money {
                let _ = self.conn.execute(
                    "UPDATE users SET money = ? WHERE id=?",
                    params![&money, &id],
                )?;
            }
            Ok(())
        }
        fn delete(&mut self, id: Id) -> ShortResult<()> {
            let _ = self
                .conn
                .execute("DELETE FROM users WHERE id = ?", [&i64::from(id)])?;
            Ok(())
        }
    }
}

use db_test::TestUserRepositiry;
pub mod db_test {
    use super::domain::{Id, User};
    use super::ShortResult;
    use super::UserDTO;
    use super::UserRepository;
    use std::collections::HashMap;
    use std::convert::TryInto;

    pub struct TestUserRepositiry {
        users: HashMap<i64, (String, u64)>, // user_id,(name,money)
        last_id: i64,
    }
    impl TestUserRepositiry {
        pub fn new() -> Self {
            Self {
                users: HashMap::new(),
                last_id: 0,
            }
        }
    }
    impl UserRepository for TestUserRepositiry {
        fn create(&mut self, name: &str, money: u64) -> ShortResult<Id> {
            self.last_id = self.last_id + 1;
            self.users.insert(self.last_id, (name.to_owned(), money));
            Ok(self.last_id.try_into()?)
        }
        fn read(&mut self, id: Id) -> ShortResult<()> {
            if let Some(val) = self.users.get(&i64::from(id)) {
                println!("{}", User::new(&val.0, val.1));
            }
            Ok(())
        }
        fn update(&mut self, id: Id, user: UserDTO) -> ShortResult<()> {
            let id = i64::from(id);
            if let (Some(name), Some(money)) = (user.name.clone(), user.money) {
                if let Some(ref mut val) = self.users.get_mut(&id) {
                    val.0 = name;
                    val.1 = money;
                }
            } else if let Some(name) = user.name {
                if let Some(ref mut val) = self.users.get_mut(&id) {
                    val.0 = name;
                }
            } else if let Some(money) = user.money {
                if let Some(ref mut val) = self.users.get_mut(&id) {
                    val.1 = money;
                }
            }
            Ok(())
        }
        fn delete(&mut self, id: Id) -> ShortResult<()> {
            let id = i64::from(id);
            if self.users.contains_key(&id) {
                self.users.remove(&i64::from(id)).unwrap();
            }
            Ok(())
        }
    }
}

// Единственный репозиторий не нуждается в `Unit Of Work` так как нет бизнес транзакции нуждающейся в контроле сохраняемого состояния не связанных сущностей!
// Когда в системе появляются связи сущностей (репозитории), тогда есть запрос на поддержания консистентности их состояния в хранилище данных.

trait Controller: UserRepository {}
struct ControllerImpl<R> {
    pub rep: R, // TODO: вот тут бизнес логика не зависит от источника данных, все что нужно для ее работы это методы UserRepository
}
impl<R: UserRepository> ControllerImpl<R> {
    pub fn new(rep: R) -> Self {
        Self { rep }
    }
}
impl<R: UserRepository> UserRepository for ControllerImpl<R> {
    fn create(&mut self, name: &str, money: u64) -> ShortResult<Id> {
        self.rep.create(name, money)
    }
    fn read(&mut self, id: Id) -> ShortResult<()> {
        self.rep.read(id)
    }
    fn update(&mut self, id: Id, user: UserDTO) -> ShortResult<()> {
        self.rep.update(id, user)
    }
    fn delete(&mut self, id: Id) -> ShortResult<()> {
        self.rep.delete(id)
    }
}
impl<R: UserRepository> Controller for ControllerImpl<R> {}

// cargo run --bin repository
fn main() {
    /*
        let mut type_repository = String::new();
        println!("Enter type of repository (file or sqlite or test): ");
        let _ = std::io::stdin().read_line(&mut type_repository).unwrap();

        let mut controller:Box<dyn Controller>;
        if type_repository == "file" {
            controller = Box::new(ControllerImpl::new(FileUserRepository::new("design_patterns/unit-of-works-2/examples/db_file").unwrap()));
        }else if type_repository == "sqlite" {
            controller = Box::new(ControllerImpl::new(SqliteUserRepository::new().unwrap()));
        }else{
            controller = Box::new(ControllerImpl::new(TestUserRepositiry::new()));
        }

        let _ = controller.create("Jeka", 200).unwrap();
        let _ = controller.create("Alex", 300).unwrap();
        let _ = controller.create("Zorro", 100).unwrap();
        let _ = controller.delete(1.try_into().unwrap()).unwrap();
        let _ = controller.update(2.try_into().unwrap(), UserDTO{name:Some("Hola".into()),money: None}).unwrap();
        let _ = controller.read(2.try_into().unwrap()).unwrap();
    */
    // ----------------------------------------------------------------------------------------------------------------------------------

    let mut controller =
        ControllerImpl::new(FileUserRepository::new("design_patterns/repository/db").unwrap());
    // or
    let mut controller = ControllerImpl::new(SqliteUserRepository::new().unwrap());
    // or
    let mut controller = ControllerImpl::new(TestUserRepositiry::new());

    let _ = controller.create("Jeka", 200).unwrap();
    let _ = controller.create("Alex", 300).unwrap();
    let _ = controller.create("Zorro", 100).unwrap();
    let _ = controller.delete(1.try_into().unwrap()).unwrap();
    let _ = controller
        .update(
            2.try_into().unwrap(),
            UserDTO {
                name: Some("Hola".into()),
                money: None,
            },
        )
        .unwrap();
    let _ = controller.read(2.try_into().unwrap()).unwrap();
}
