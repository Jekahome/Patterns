/*

`Pattern Adapter` работает как мост между двумя несовместимыми интерфейсами.
Эту стратегию можно реализовать с помощью наследования, с помощью агрегации, или композиции, то есть делегирования работы другим объектам.
`Pattern Adapter` делает два несовместимых интерфейса совместимыми без изменения их существующего кода позволяя им работать вместе.
Adapter это объект-переводчик, который трансформирует интерфейс или данные одного объекта в такой вид, чтобы он стал понятен другому объекту.
При этом адаптер оборачивает один из объектов, так что другой объект даже не знает о наличии первого.
Например, вы можете обернуть объект, работающий в метрах, адаптером, который бы конвертировал данные в футы.

Адаптеры могут не только переводить данные из одного формата в другой, но и помогать объектам с разными интерфейсами работать сообща.

Это работает так:
1. `Pattern Adapter` имеет интерфейс, который совместим с одним из объектов.
2. Поэтому этот объект может свободно вызывать методы адаптера.
3. `Pattern Adapter` получает эти вызовы и перенаправляет их второму объекту, но уже в том формате и последовательности, которые понятны второму объекту.


Применимость
- Когда вы хотите использовать сторонний класс, но его интерфейс не соответствует остальному коду приложения.
- `Pattern Adapter` позволяет создать объект-прокладку, который будет превращать вызовы приложения в формат, понятный стороннему классу.

**Отношения с другими паттернами**

Основное различие между `Pattern Brige` и `Pattern Adapter` заключается в том, что `Pattern Adapter` используется
для унификации уже существующих интерфейсов, а `Pattern Brige` используется, когда есть подозрение, что реализация интерфейса со временем изменится.
Т.е. своевременное использование `Pattern Brige` избавит нас от необходимости внедрять `Pattern Adapter`

`Pattern Facade` задаёт новый интерфейс, тогда как `Pattern Adapter` повторно использует старый.
`Pattern Adapter` оборачивает только один класс, а `Pattern Facade` оборачивает целую подсистему.
Кроме того, `Pattern Adapter` позволяет двум существующим интерфейсам работать сообща, вместо того, чтобы задать полностью новый.

`Pattern Adapter` предоставляет другой интерфейс для своего объекта.
`Pattern Proxy` предоставляет тот же интерфейс.
`Pattern Decorator` предоставляет улучшенный интерфейс.

`Pattern Adapter` предназначен для изменения интерфейса существующего объекта.
`Pattern Decorator` улучшает другой объект, не меняя его интерфейс.
Таким образом, декоратор более прозрачен для приложения, чем адаптер.
Как следствие, `Pattern Decorator` поддерживает рекурсивную композицию, что невозможно при использовании чистых адаптеров.
*/

/*
Интрументы работы со строками не совместимы, tools_a работает в асинхронной среде, а tools_b в синхронной
Необходима возможность их соединить для совместной работы
*/
pub mod tools_a {
    pub async fn add_wrap_start(arg: String) -> String {
        std::future::ready(format!("~~~{}", arg)).await
    }
}

pub mod tools_b {
    use std::string::FromUtf8Error;
    pub fn add_wrap_end(arg: &[u8]) -> Result<String, FromUtf8Error> {
        let s = String::from_utf8(arg.to_vec())?;
        let res = format!("{}~~~", s);
        Ok(res)
    }
}

use adapter::*;
pub mod adapter {
    use crate::tools_b;

    use futures::Future;
    pub trait AdapterAsyncToSync {
        fn add_wrap_end(fut: impl Future<Output = String>) -> String;
    }
    pub struct ImplAdapter;
    impl AdapterAsyncToSync for ImplAdapter {
        fn add_wrap_end(fut: impl Future<Output = String>) -> String {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let result = rt.block_on(fut);
            if let Ok(res) = tools_b::add_wrap_end(result.as_bytes()) {
                return res;
            }
            result
        }
    }
}

// cargo run --bin adapter
fn main() {
    let input = "hello";
    let input_with_wrap = {
        let input_a = tools_a::add_wrap_start(input.into());
        let input_b: String = ImplAdapter::add_wrap_end(input_a);
        input_b
    };
    assert_eq!("~~~hello~~~", input_with_wrap)
}
