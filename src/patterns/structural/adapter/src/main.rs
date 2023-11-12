/*

`Template Adapter` работает как мост между двумя несовместимыми интерфейсами.
Эту стратегию можно реализовать с помощью наследования, с помощью агрегации, или композиции, то есть делегирования работы другим объектам.
`Template Adapter` делает два несовместимых интерфейса совместимыми без изменения их существующего кода позволяя им работать вместе.
Adapter это объект-переводчик, который трансформирует интерфейс или данные одного объекта в такой вид, чтобы он стал понятен другому объекту.
При этом адаптер оборачивает один из объектов, так что другой объект даже не знает о наличии первого.
Например, вы можете обернуть объект, работающий в метрах, адаптером, который бы конвертировал данные в футы.

Адаптеры могут не только переводить данные из одного формата в другой, но и помогать объектам с разными интерфейсами работать сообща.

Это работает так:
1. `Template Adapter` имеет интерфейс, который совместим с одним из объектов.
2. Поэтому этот объект может свободно вызывать методы адаптера.
3. `Template Adapter` получает эти вызовы и перенаправляет их второму объекту, но уже в том формате и последовательности, которые понятны второму объекту.


Применимость
- Когда вы хотите использовать сторонний класс, но его интерфейс не соответствует остальному коду приложения.
- `Template Adapter` позволяет создать объект-прокладку, который будет превращать вызовы приложения в формат, понятный стороннему классу.

**Отношения с другими паттернами**

Основное различие между `Template Brige` и `Template Adapter` заключается в том, что `Template Adapter` используется
для унификации уже существующих интерфейсов, а `Template Brige` используется, когда есть подозрение, что реализация интерфейса со временем изменится.
Т.е. своевременное использование `Template Brige` избавит нас от необходимости внедрять `Template Adapter`

`Template Facade` задаёт новый интерфейс, тогда как `Template Adapter` повторно использует старый.
`Template Adapter` оборачивает только один класс, а `Template Facade` оборачивает целую подсистему.
Кроме того, `Template Adapter` позволяет двум существующим интерфейсам работать сообща, вместо того, чтобы задать полностью новый.

`Template Adapter` предоставляет другой интерфейс для своего объекта.
`Template Proxy` предоставляет тот же интерфейс.
`Template Decorator` предоставляет улучшенный интерфейс.

`Template Adapter` предназначен для изменения интерфейса существующего объекта.
`Template Decorator` улучшает другой объект, не меняя его интерфейс.
Таким образом, декоратор более прозрачен для приложения, чем адаптер.
Как следствие, `Template Decorator` поддерживает рекурсивную композицию, что невозможно при использовании чистых адаптеров.
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

fn main() {
    let input = "hello";
    let input_with_wrap = {
        let input_a = tools_a::add_wrap_start(input.into());
        let input_b: String = ImplAdapter::add_wrap_end(input_a);
        input_b
    };
    assert_eq!("~~~hello~~~", input_with_wrap)
}
