/*

`Pattern Proxy` позволяет подставлять вместо реальных объектов специальные объекты-заменители.
Эти объекты перехватывают вызовы к оригинальному объекту, позволяя сделать что-то до или после передачи вызова оригиналу.

Применение:

1.Ленивая инициализация (виртуальный прокси). Когда у вас есть тяжёлый объект, грузящий данные из файловой системы или базы данных.
Вместо того, чтобы грузить данные сразу после старта программы, можно сэкономить ресурсы и создать объект тогда, когда он действительно понадобится.

2.Удаленный прокси-сервер предоставляет локального представителя объекта, который находится в другом адресном пространстве.
Это то, что обеспечивает код-заглушка в RPC и CORBA.

3.Защитный прокси-сервер контролирует доступ к конфиденциальному главному объекту.
Защита доступа (защищающий прокси). Когда в программе есть разные типы пользователей, и вам хочется защищать объект от неавторизованного доступа. Например, если ваши объекты — это важная часть операционной системы, а пользователи — сторонние программы (хорошие или вредоносные).
Прокси может проверять доступ при каждом вызове и передавать выполнение служебному объекту, если доступ разрешён.

4.Умный прокси выполняет дополнительные действия при доступе к объекту. Типичное использование включает в себя:
Подсчет количества ссылок на реальный объект, чтобы его можно было автоматически освободить, когда ссылок больше нет (так называемый умный указатель),
Загрузка постоянного объекта в память при первом обращении к нему,
Проверка блокировки реального объекта перед доступом к нему, чтобы гарантировать, что никакой другой объект не сможет его изменить.

Эмпирические правила

`Pattern Decorator` и `Pattern Proxy` имеют схожие структуры, но разные назначения. Они похожи тем, что оба построены на принципе композиции и делегируют работу другим объектам.
Паттерны отличаются тем, что `Pattern Proxy` сам управляет жизнью сервисного объекта, а обёртывание Декораторов контролируется клиентом.
*/

pub trait Server {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String);
}

pub mod application {
    use super::Server;

    pub struct Application;

    impl Server for Application {
        fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
            if url == "/app/status" && method == "GET" {
                return (200, "Ok".into());
            }

            if url == "/create/user" && method == "POST" {
                return (201, "User Created".into());
            }

            (404, "Not Ok".into())
        }
    }
}

use nginx::*;
pub mod nginx {
    use std::collections::HashMap;

    use super::{application::Application, Server};

    /// NGINX server is a proxy to an application server.
    pub struct NginxServer {
        application: Application,
        max_allowed_requests: u32,
        rate_limiter: HashMap<String, u32>,
    }

    impl NginxServer {
        pub fn new() -> Self {
            Self {
                application: Application,
                max_allowed_requests: 2,
                rate_limiter: HashMap::default(),
            }
        }

        pub fn check_rate_limiting(&mut self, url: &str) -> bool {
            let rate = self.rate_limiter.entry(url.to_string()).or_insert(1);

            if *rate > self.max_allowed_requests {
                return false;
            }

            *rate += 1;
            true
        }
    }

    impl Server for NginxServer {
        fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
            if !self.check_rate_limiting(url) {
                return (403, "Not Allowed".into());
            }

            self.application.handle_request(url, method)
        }
    }
}

// cargo run --example p_structural_proxy

fn main() {
    let app_status = &"/app/status".to_string();
    let create_user = &"/create/user".to_string();

    let mut nginx = NginxServer::new();

    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx.handle_request(create_user, "POST");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);

    let (code, body) = nginx.handle_request(create_user, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);
}
