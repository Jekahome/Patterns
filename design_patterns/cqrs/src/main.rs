#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

/*
Создал с помошью ChatGPT и отрефакторино.

В этом примере, CommandHandler обрабатывает команды (создание, обновление, удаление сущности) и возвращает соответствующие события. 
GetEntityQueryHandler обрабатывает запрос для получения данных о сущности. 
EntityRepository представляет репозиторий для хранения сущностей.
*/

// Команда для создания новой сущности
#[derive(Debug)]
struct CreateEntityCommand {
    id: u64,
    name: String,
}

// Команда для обновления сущности
#[derive(Debug)]
struct UpdateEntityCommand {
    id: u64,
    name: String,
}

// Команда для удаления сущности
#[derive(Debug)]
struct DeleteEntityCommand {
    id: u64,
}

// Событие, сигнализирующее об успешном создании сущности
#[derive(Debug)]
struct EntityCreatedEvent {
    id: u64,
    name: String,
}

// Событие, сигнализирующее об успешном обновлении сущности
#[derive(Debug)]
struct EntityUpdatedEvent {
    id: u64,
    name: String,
}

// Событие, сигнализирующее об успешном удалении сущности
#[derive(Debug)]
struct EntityDeletedEvent {
    id: u64,
}

// Запрос для получения данных о сущности
struct GetEntityQuery {
    id: u64,
}

// Результат запроса для данных о сущности
#[derive(Debug)]
struct GetEntityQueryResult {
    id: u64,
    name: String,
}

// Обработчик команд и событий
trait Handler<TCommand, TEvent> {
    fn handle(&mut self, command: TCommand) -> Vec<TEvent>;
}

// Обработчик запросов
trait QueryHandler<TQuery, TResult> {
    fn handle(&self, query: TQuery) -> TResult;
}

// Репозиторий для хранения сущностей
struct EntityRepository {
    entities: HashMap<u64, String>,
}

impl EntityRepository {
    fn new() -> Self {
        EntityRepository {
            entities: HashMap::new(),
        }
    }

    fn create(&mut self, id: u64, name: String) {
        self.entities.insert(id, name);
    }

    fn update(&mut self, id: u64, name: String) {
        if let Some(entity) = self.entities.get_mut(&id) {
            *entity = name;
        }
    }

    fn delete(&mut self, id: u64) {
        self.entities.remove(&id);
    }

    fn get(&self, id: u64) -> Option<&String> {
        self.entities.get(&id)
    }
}

// Обработчик команд для создания, обновления и удаления сущностей
struct CommandHandler  {
    entity_repository: Rc<RefCell<EntityRepository>>,
}

impl Handler<CreateEntityCommand, EntityCreatedEvent> for CommandHandler {
    fn handle(&mut self, command: CreateEntityCommand) -> Vec<EntityCreatedEvent> {
        self.entity_repository.borrow_mut().create(command.id, command.name.clone());

        vec![EntityCreatedEvent {
            id: command.id,
            name: command.name,
        }]
    }
}

impl Handler<UpdateEntityCommand, EntityUpdatedEvent> for CommandHandler {
    fn handle(&mut self, command: UpdateEntityCommand) -> Vec<EntityUpdatedEvent> {
        self.entity_repository.borrow_mut().update(command.id, command.name.clone());

        vec![EntityUpdatedEvent {
            id: command.id,
            name: command.name,
        }]
    }
}

impl Handler<DeleteEntityCommand, EntityDeletedEvent> for CommandHandler {
    fn handle(&mut self, command: DeleteEntityCommand) -> Vec<EntityDeletedEvent> {
        self.entity_repository.borrow_mut().delete(command.id);

        vec![EntityDeletedEvent { id: command.id }]
    }
}

// Обработчик запроса для получения данных о сущности
struct GetEntityQueryHandler {
    entity_repository: Rc<RefCell<EntityRepository>>//&'a EntityRepository,
}

impl QueryHandler<GetEntityQuery, Option<GetEntityQueryResult>> for GetEntityQueryHandler {
    fn handle(&self, query: GetEntityQuery) -> Option<GetEntityQueryResult> {
        if let Some(name) = self.entity_repository.borrow().get(query.id) {
            Some(GetEntityQueryResult {
                id: query.id,
                name: name.clone(),
            })
        } else {
            None
        }
    }
}

// cargo run --bin cqrs
fn main() {
    let entity_repository: Rc<RefCell<EntityRepository>> = Rc::new(RefCell::new(EntityRepository::new()));
    let mut command_handler = CommandHandler {
        entity_repository: Rc::clone(&entity_repository),
    };
    let query_handler = GetEntityQueryHandler {
        entity_repository: Rc::clone(&entity_repository),
    };

    // Создание новой сущности
    let create_command = CreateEntityCommand {
        id: 1,
        name: "Entity1".to_string(),
    };
    let create_events = command_handler.handle(create_command);
    println!("Create Events: {:?}", create_events);

    // Обновление сущности
    let update_command = UpdateEntityCommand {
        id: 1,
        name: "UpdatedEntity1".to_string(),
    };
    let update_events = command_handler.handle(update_command);
    println!("Update Events: {:?}", update_events);

    // Удаление сущности
    let delete_command = DeleteEntityCommand { id: 1 };
    let delete_events = command_handler.handle(delete_command);
    println!("Delete Events: {:?}", delete_events);

    // Запрос данных о сущности
    let get_query = GetEntityQuery { id: 1 };
    let result = query_handler.handle(get_query);
    println!("Query Result: {:?}", result);
}
