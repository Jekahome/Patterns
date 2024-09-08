# Topics

* [Clean Code Principles](https://github.com/Jekahome/Patterns#clean-code-principles)
* [Семантическая и цикломатическая сложность кода](https://github.com/Jekahome/Patterns?tab=readme-ov-file#%D1%81%D0%B5%D0%BC%D0%B0%D0%BD%D1%82%D0%B8%D1%87%D0%B5%D1%81%D0%BA%D0%B0%D1%8F-%D0%B8-%D1%86%D0%B8%D0%BA%D0%BB%D0%BE%D0%BC%D0%B0%D1%82%D0%B8%D1%87%D0%B5%D1%81%D0%BA%D0%B0%D1%8F-%D1%81%D0%BB%D0%BE%D0%B6%D0%BD%D0%BE%D1%81%D1%82%D1%8C-%D0%BA%D0%BE%D0%B4%D0%B0)
* [Programming Paradigms](https://github.com/Jekahome/Patterns#programming-paradigms)
* [Что такое паттерны, зачем и почему?](https://github.com/Jekahome/Patterns#%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B-%D0%B7%D0%B0%D1%87%D0%B5%D0%BC-%D0%B8-%D0%BF%D0%BE%D1%87%D0%B5%D0%BC%D1%83)
* [Подходят ли ООП паттерны для Rust?](https://github.com/Jekahome/Patterns#%D0%BF%D0%BE%D0%B4%D1%85%D0%BE%D0%B4%D1%8F%D1%82-%D0%BB%D0%B8-%D0%BE%D0%BE%D0%BF-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B-%D0%B4%D0%BB%D1%8F-rust)
* [Rust idioms](https://github.com/Jekahome/Patterns#rust-idioms)
* [Anti patterns](https://github.com/Jekahome/Patterns#anti-patterns)
* [Design principles](https://github.com/Jekahome/Patterns#design-principles-solid-kiss-dry-yagni-grasp): [SOLID](https://github.com/Jekahome/Patterns?tab=readme-ov-file#solid), [KISS](https://github.com/Jekahome/Patterns?tab=readme-ov-file#kiss), [DRY](https://github.com/Jekahome/Patterns?tab=readme-ov-file#dry), [YAGNI](https://github.com/Jekahome/Patterns?tab=readme-ov-file#yagni), [GRASP](https://github.com/Jekahome/Patterns?tab=readme-ov-file#grasp), [LoD](https://github.com/Jekahome/Patterns?tab=readme-ov-file#lod), [SoC](https://github.com/Jekahome/Patterns?tab=readme-ov-file#soc)
* [Gangs of Four (GoF) Design Patterns](https://github.com/Jekahome/Patterns#gangs-of-four-gof-design-patterns)
* * [Порождающие паттерны](https://github.com/Jekahome/Patterns#%D0%BF%D0%BE%D1%80%D0%BE%D0%B6%D0%B4%D0%B0%D1%8E%D1%89%D0%B8%D0%B5-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B)
* * [Структурирующие паттерны](https://github.com/Jekahome/Patterns#%D1%81%D1%82%D1%80%D1%83%D0%BA%D1%82%D1%83%D1%80%D0%B8%D1%80%D1%83%D1%8E%D1%89%D0%B8%D0%B5-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B)
* * [Паттерны поведения](https://github.com/Jekahome/Patterns#%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B-%D0%BF%D0%BE%D0%B2%D0%B5%D0%B4%D0%B5%D0%BD%D0%B8%D1%8F)
* [Other Design Patterns](https://github.com/Jekahome/Patterns#other-design-patterns) (...Dependency injection (DI))
* PoSA
* [Database Patterns](https://github.com/Jekahome/Patterns#database-patterns) ([Repository](https://github.com/Jekahome/Patterns#repository), [Unit of Work](https://github.com/Jekahome/Patterns#unit-of-work-uow), Lazy Load)
* [Architecture](https://github.com/Jekahome/Patterns#architecture) ([CQRS](https://github.com/Jekahome/Patterns#architectural-pattern-cqrs), [DDD](https://github.com/Jekahome/Patterns#architectural-pattern-domain-driven-design-ddd), [Layered architecture](https://github.com/Jekahome/Patterns#layered-architecture))
* [Refactoring](https://github.com/Jekahome/Patterns#refactoring)

# Clean Code Principles

`rust unimplemented!`

"Чистый код" - это концепция, предложенная Робертом Мартином в его книге "Clean Code: A Handbook of Agile Software Craftsmanship". 
Принципы чистого кода призывают к написанию программного кода, который легко читаем, понятен, поддается тестированию и легко поддается изменениям. Вот некоторые из основных принципов чистого кода:

* #### Ясность (Clarity):

    Код должен быть написан ясно и читаемо. 
    Максимально избавиться от сложного кода, лучше проще и глупее (в смысле понимания другими программистами). 
    Имена переменных, функций и классов должны быть выразительными и информативными.
    Код не должен содержать непонятных или амбигуальных частей.

    **Амбигуальный код** делает программу сложной для понимания и сопровождения, а также увеличивает вероятность возникновения ошибок. 

    **Амбигуальные** части кода (или амбигуальный код) — это участки кода, которые могут быть интерпретированы неоднозначно.
    - Неоднозначные выражения (неочевыдные правила приоритета операций могут отличаться в разных языках)
    - Проблемы с областью видимости (перекрытие глобальных переменных локальными)
    - Неоднозначные функции или методы (поведение метода меняется в зависимости от типа аргумента)
    - Неявные преобразования типов (когда язык программирования автоматически преобразует типы данных)
    - Проблемы с параллелизмом и многопоточностью (когда порядок выполнения потоков не гарантирован)

* #### Правила дизайна

    1. **Держите настраиваемые данные на высоких уровнях**:
        - Конфигурируемые данные, такие как параметры, настройки и конфигурации, следует держать на высоких уровнях системы, чтобы их можно было легко изменять и управлять ими. Это делает систему более гибкой и удобной для настройки без необходимости вносить изменения в низкоуровневый код.

    2. **Предпочитайте полиморфизм конструкциям if/else или switch/case**:
        - Вместо использования множества условных операторов (if/else или switch/case) для выбора поведения программы, лучше использовать полиморфизм. Полиморфизм позволяет создавать объекты различных классов, которые реализуют один и тот же интерфейс или наследуют от одного базового класса. Это делает код более гибким и расширяемым.

    3. **Разделяйте код многопоточности**:
        - Многопоточный код может быть сложным и трудным для отладки. Поэтому его следует изолировать и разделять от остального кода. Это делает программу более устойчивой и уменьшает вероятность возникновения ошибок, связанных с параллелизмом.

    4. **Предотвращайте чрезмерную конфигурируемость**:
        - Хотя конфигурируемость важна, её избыток может усложнить систему и сделать её трудной для понимания и сопровождения. Не стоит делать все аспекты системы настраиваемыми; следует балансировать между гибкостью и сложностью.

    5. **Используйте внедрение зависимостей (Dependency Injection)**:
        - Внедрение зависимостей означает, что объект получает (или "внедряет") свои зависимости извне, а не создаёт их самостоятельно. Это упрощает тестирование и замену зависимостей, делая код более модульным и легко поддающимся изменениям.

    6. **Следуйте Закону Деметры**:
        - Закон Деметры, также известный как принцип "не говори с незнакомцами", гласит, что объект должен взаимодействовать только со своими непосредственными зависимостями. Это значит, что объект не должен напрямую обращаться к компонентам других объектов. Соблюдение этого принципа уменьшает связанность и упрощает сопровождение кода.

* #### Советы по понятности

    1. **Будьте последовательны. Если вы делаете что-то определённым образом, делайте все подобные вещи таким же образом**:

        - Последовательное использование одного и того же стиля кода и подходов делает ваш код более предсказуемым и легким для понимания.

        ```rust
        // Плохо: разные способы именования функций
        fn get_user() { /* ... */ }
        fn fetch_order() { /* ... */ }
        fn retrieve_product() { /* ... */ }

        // Хорошо: единообразный стиль именования функций
        fn get_user() { /* ... */ }
        fn get_order() { /* ... */ }
        fn get_product() { /* ... */ }
        ```

    2. **Используйте поясняющие переменные**:

        - Поясняющие переменные делают ваш код более читаемым и самодокументируемым.

        ```rust
        // Плохо
        let s = "2024-07-20T12:34:56Z";
        let t = DateTime::parse_from_rfc3339(s).unwrap();

        // Хорошо
        let date_string = "2024-07-20T12:34:56Z";
        let parsed_date = DateTime::parse_from_rfc3339(date_string).unwrap();
        ```

    3. **Инкапсулируйте граничные условия. Граничные условия трудно отслеживать. Поместите их обработку в одно место**:

        - Граничные условия (например, минимальные и максимальные значения) должны быть инкапсулированы в отдельные функции или методы для предотвращения ошибок и упрощения сопровождения.

        ```rust
        // Плохо
        fn is_valid_age(age: u32) -> bool {
            age >= 0 && age <= 120
        }

        // Хорошо
        fn is_valid_age(age: u32) -> bool {
            const MIN_AGE: u32 = 0;
            const MAX_AGE: u32 = 120;
            age >= MIN_AGE && age <= MAX_AGE
        }
        ```

    4. **Предпочитайте специализированные объекты значений вместо примитивных типов**:

        - Специализированные объекты (типы) делают код более выразительным и помогают предотвратить ошибки.

        ```rust
        // Плохо
        fn calculate_total(price: f64, tax: f64) -> f64 {
            price + tax
        }

        // Хорошо
        struct Money {
            amount: f64,
        }

        impl Money {
            fn new(amount: f64) -> Self {
                Money { amount }
            }

            fn add(&self, other: Money) -> Money {
                Money::new(self.amount + other.amount)
            }
        }

        let price = Money::new(100.0);
        let tax = Money::new(20.0);
        let total = price.add(tax);
        ```

    5. **Избегайте логической зависимости. Не пишите методы, которые работают правильно, завися от чего-то ещё в том же классе**:

        - Методы должны быть независимыми и не полагаться на состояние или поведение других методов в классе.

        ```rust
        // Плохо
        struct Order {
            items: Vec<Item>,
            total: f64,
        }

        impl Order {
            fn calculate_total(&mut self) {
                self.total = self.items.iter().map(|item| item.price).sum();
            }

            fn print_total(&self) {
                println!("{}", self.total); // Работает правильно только после вызова calculate_total()
            }
        }

        // Хорошо
        struct Order {
            items: Vec<Item>,
        }

        impl Order {
            fn calculate_total(&self) -> f64 {
                self.items.iter().map(|item| item.price).sum()
            }

            fn print_total(&self) {
                println!("{}", self.calculate_total());
            }
        }
        ```

    6. **Избегайте отрицательных условий**:

        - Положительные условия легче понимать, чем отрицательные. Переписывайте отрицательные условия на положительные, когда это возможно.

        ```rust
        // Плохо
        fn is_not_error(status: &str) -> bool {
            status != "error"
        }

        // Хорошо
        fn is_success(status: &str) -> bool {
            status == "success"
        }
        ```

 
* #### Правила для функции

    1. **Делайте только одно**:
        - Функция должна выполнять только одну задачу. Это делает её более понятной, легко тестируемой и повторно используемой. 

        ```rust
        // Плохо
        fn process_order(order: &Order) {
            validate_order(order);
            save_to_database(order);
            send_confirmation_email(order);
        }

        // Хорошо
        fn validate_order(order: &Order) -> bool {
            // Логика валидации заказа
        }

        fn save_to_database(order: &Order) {
            // Логика сохранения заказа в базу данных
        }

        fn send_confirmation_email(order: &Order) {
            // Логика отправки подтверждающего письма
        }

        ```

    2. **Предпочитайте меньшее количество аргументов**:
        - Ограничивайте количество аргументов функции. Идеально, когда их не больше трёх. Если их больше, подумайте о создании объекта для их объединения.

        ```rust
        // Плохо
        fn create_user(name: &str, age: u32, email: &str, address: &str) {
            // Логика создания пользователя
        }

        // Хорошо
        struct User {
            name: String,
            age: u32,
            email: String,
            address: String,
        }

        fn create_user(user: User) {
            // Логика создания пользователя
        }

        ```

    3. **Не имейте побочных эффектов**:
        - Функции не должны изменять состояние программы вне своего тела. Избегайте изменения глобальных переменных или изменения аргументов.

        ```rust
        // Плохо
        static mut GLOBAL_STATE: HashMap<String, String> = HashMap::new();

        fn update_state(key: String, value: String) {
            unsafe {
                GLOBAL_STATE.insert(key, value);
            }
        }

        // Хорошо
        fn update_state(state: &mut HashMap<String, String>, key: String, value: String) {
            state.insert(key, value);
        }

        ```

    4. **Не используйте флаговые аргументы. Разделите метод на несколько независимых методов, которые могут быть вызваны клиентом без флага**:
        - Флаговые аргументы указывают на то, что функция выполняет более одной задачи. Разделите такие функции на несколько, каждая из которых выполняет свою собственную задачу.

        ```rust
        // Плохо
        fn set_user_status(user: &mut User, active: bool) {
            if active {
                user.status = "active".to_string();
            } else {
                user.status = "inactive".to_string();
            }
        }

        // Хорошо
        fn activate_user(user: &mut User) {
            user.status = "active".to_string();
        }

        fn deactivate_user(user: &mut User) {
            user.status = "inactive".to_string();
        }

        ```

* #### Правила комментариев
 
    1. **Всегда старайтесь объяснять себя в коде**:
        - Пишите код таким образом, чтобы он был самодокументируемым. Используйте понятные имена переменных, функций и классов, чтобы минимизировать необходимость в комментариях.

        ```rust
        // Плохо
        // Функция для проверки, является ли пользователь активным
        fn check(user: &User) -> bool {
            user.status == "active"
        }

        // Хорошо
        fn is_user_active(user: &User) -> bool {
            user.status == "active"
        }
        ```

    2. **Не будьте избыточными**:
        - Не добавляйте комментарии, которые просто повторяют то, что уже очевидно из кода.

        ```rust
        // Плохо
        let x = x + 1; // Увеличиваем x на 1

        // Хорошо
        let x = x + 1;

        ```

    3. **Не добавляйте очевидный шум**:
        - Избегайте комментариев, которые не добавляют никакой ценности и только засоряют код.

        ```rust
        // Плохо
        let mut i = 0; // Устанавливаем i в 0

        // Хорошо
        let mut i = 0;

        ```

    4. **Не используйте комментарии закрывающей скобки**:
        - Нет необходимости добавлять комментарии после закрывающей скобки, так как современный код, как правило, короткий и читаемый.

        ```rust
        // Плохо
        if condition {
            do_something();
        } // Конец if

        // Хорошо
        if condition {
            do_something();
        }

        ```

    5. **Не комментируйте код. Просто удаляйте его**:
        - Закомментированный код создает беспорядок и затрудняет чтение. Если код не нужен, лучше его удалить.

        ```rust
        // Плохо
        // fn old_function() {
        //     // старый код
        // }

        // Хорошо
        fn new_function() {
            // новый код
        }

        ```

    6. **Используйте комментарии для объяснения намерений**:
        - Объясняйте, почему вы что-то делаете, особенно если это не очевидно из самого кода.

        ```rust
        // Плохо
        let result = calculate();

        // Хорошо
        // Используем временную переменную для хранения результата вычислений
        let result = calculate();

        ```

    7. **Используйте комментарии для пояснения кода**:
        - Объясняйте сложные или нетривиальные части кода, чтобы облегчить понимание другим разработчикам.

        ```rust
        // Плохо
        let data = fetch_data();

        // Хорошо
        // Получаем данные из внешнего API и преобразуем их в формат JSON
        let data = fetch_data();

        ```

    8. **Используйте комментарии для предупреждения о последствиях**:
        - Указывайте на возможные последствия или побочные эффекты, особенно если они не очевидны.

        ```rust
        // Плохо
        delete_user(user_id);

        // Хорошо
        // Удаление пользователя приведет к удалению всех связанных данных
        delete_user(user_id);

    ```

* #### Структура исходного кода

    1. **Разделяйте концепции вертикально**:

        - Размещайте разные концепции на отдельных участках кода. Это делает код более организованным и читабельным.
           "разделение концепций вертикально" - означает организацию кода таким образом, чтобы разные концепции, функциональные блоки или уровни абстракции были размещены в разных частях файла или модуля, а не перемешаны друг с другом.

        ```rust
        // Плохо
        fn calculate_area(length: f64, width: f64) -> f64 {
            length * width
        }

        fn process_order(order: &Order) {
            // Логика обработки заказа
        }

        fn is_valid_age(age: u32) -> bool {
            age >= 0 && age <= 120
        }

        // Хорошо
        fn calculate_area(length: f64, width: f64) -> f64 {
            length * width
        }

        fn is_valid_age(age: u32) -> bool {
            age >= 0 && age <= 120
        }

        fn process_order(order: &Order) {
            // Логика обработки заказа
        }
        ```

    2. **Связанный код должен быть размещён плотно вертикально**:

        - Код, который тесно связан, должен быть размещён вместе, чтобы облегчить его понимание.

        ```rust
        // Плохо
        fn add_item(order: &mut Order, item: Item) {
            order.items.push(item);
        }

        fn calculate_total(order: &Order) -> f64 {
            order.items.iter().map(|item| item.price).sum()
        }

        struct Item {
            price: f64,
        }

        struct Order {
            items: Vec<Item>,
        }

        // Хорошо
        struct Item {
            price: f64,
        }

        struct Order {
            items: Vec<Item>,
        }

        fn add_item(order: &mut Order, item: Item) {
            order.items.push(item);
        }

        fn calculate_total(order: &Order) -> f64 {
            order.items.iter().map(|item| item.price).sum()
        }
        ```

    3. **Объявляйте переменные близко к месту их использования**:

        - Это уменьшает область видимости переменных и облегчает понимание кода.

        ```rust
        // Плохо
        let mut total = 0.0;
        for item in &order.items {
            total += item.price;
        }
        println!("Total: {}", total);

        // Хорошо
        let total: f64 = order.items.iter().map(|item| item.price).sum();
        println!("Total: {}", total);
        ```

    4. **Зависимые функции должны быть близко**:

        - Функции, которые зависят друг от друга, должны быть расположены рядом для облегчения понимания их взаимодействия.

        ```rust
        // Плохо
        fn process_payment() {
            // ...
        }

        fn validate_order(order: &Order) {
            // ...
        }

        fn complete_order(order: &Order) {
            validate_order(order);
            process_payment();
            // ...
        }

        // Хорошо
        fn validate_order(order: &Order) {
            // ...
        }

        fn process_payment() {
            // ...
        }

        fn complete_order(order: &Order) {
            validate_order(order);
            process_payment();
            // ...
        }
        ```

    5. **Похожие функции должны быть близко**:

        - Функции, которые выполняют схожие задачи, должны быть расположены рядом для облегчения их поиска и понимания.

        ```rust
        // Плохо
        fn add_item(order: &mut Order, item: Item) {
            order.items.push(item);
        }

        fn calculate_total(order: &Order) -> f64 {
            order.items.iter().map(|item| item.price).sum()
        }

        fn remove_item(order: &mut Order, item_id: usize) {
            order.items.remove(item_id);
        }

        // Хорошо
        fn add_item(order: &mut Order, item: Item) {
            order.items.push(item);
        }

        fn remove_item(order: &mut Order, item_id: usize) {
            order.items.remove(item_id);
        }

        fn calculate_total(order: &Order) -> f64 {
            order.items.iter().map(|item| item.price).sum()
        }
        ```

    6. **Размещайте функции в нисходящем порядке**:

        - Функции должны быть размещены таким образом, чтобы каждая функция использовала функции, определённые выше неё.

        ```rust
        // Плохо
        fn process_order(order: &Order) {
            let total = calculate_total(order);
            println!("Total: {}", total);
        }

        fn calculate_total(order: &Order) -> f64 {
            order.items.iter().map(|item| item.price).sum()
        }

        // Хорошо
        fn calculate_total(order: &Order) -> f64 {
            order.items.iter().map(|item| item.price).sum()
        }

        fn process_order(order: &Order) {
            let total = calculate_total(order);
            println!("Total: {}", total);
        }
        ```

    7. **Держите строки короткими**:

        - Короткие строки облегчают чтение кода и предотвращают горизонтальный скроллинг.

        ```rust
        // Плохо
        let very_long_variable_name = "This is a very long string that exceeds the recommended line length for readability";

        // Хорошо
        let very_long_variable_name = "This is a very long string \
                                    that exceeds the recommended \
                                    line length for readability";
        ```

    8. **Не используйте горизонтальное выравнивание**:

        - Горизонтальное выравнивание переменных и комментариев затрудняет внесение изменений и поддержание кода.
          Когда вы выравниваете переменные и комментарии по горизонтали, изменение одного элемента может потребовать перестановки остальных элементов на той же линии. 

        ```rust
        // Плохо
        let first_variable  = 1;    // Первый комментарий
        let second_variable = 2;    // Второй комментарий

        // Хорошо
        let first_variable = 1;  // Первый комментарий
        let second_variable = 2; // Второй комментарий
        ```

    9. **Используйте пробелы для связывания связанных элементов и разделения слабо связанных**:

        - Используйте пробелы для визуального разделения разных блоков кода и связывания логически связанных частей.

        ```rust
        // Плохо
        let a = 1;
        let b = 2;
        let c = 3;

        if a > b {
            println!("a is greater than b");
        }
        let result = a + b + c;
        println!("Result: {}", result);

        // Хорошо
        let a = 1;
        let b = 2;
        let c = 3;

        if a > b {
            println!("a is greater than b");
        }

        let result = a + b + c;
        println!("Result: {}", result);
        ```

    10. **Не нарушайте отступы**:

        - Следуйте стандартным правилам отступов для вашего языка программирования, чтобы поддерживать код в читаемом состоянии.

        ```rust
        // Плохо
        fn main() {
        let x = 5;
            if x > 0 {
            println!("x is positive");
            }
        }

        // Хорошо
        fn main() {
            let x = 5;
            if x > 0 {
                println!("x is positive");
            }
        }
        ```

* #### Объекты и структуры данных

 
    1. **Скрывайте внутреннюю структуру**

    Скрытие внутренней структуры помогает избежать зависимостей от деталей реализации и делает ваш код более устойчивым к изменениям.

    **Пример:**

    ```rust
    // Плохо: внутренние детали структуры видны
    pub struct Order {
        pub items: Vec<Item>,
        pub total: f64,
    }

    // Хорошо: скрытие деталей внутренней структуры
    pub struct Order {
        items: Vec<Item>,
        total: f64,
    }

    impl Order {
        pub fn new() -> Self {
            Order {
                items: Vec::new(),
                total: 0.0,
            }
        }

        pub fn add_item(&mut self, item: Item) {
            self.items.push(item);
            self.update_total();
        }

        pub fn get_total(&self) -> f64 {
            self.total
        }

        fn update_total(&mut self) {
            self.total = self.items.iter().map(|item| item.price).sum();
        }
    }
    ```

    2. **Предпочитайте структуры данных**

    Использование простых структур данных, таких как `struct` в Rust, помогает избежать излишнего усложнения и повышает ясность.

    **Пример:**

    ```rust
    // Плохо: сложный объект с множеством методов и состояний
    pub struct Order {
        items: Vec<Item>,
        total: f64,
    }

    impl Order {
        pub fn new() -> Self {
            // ...
        }

        pub fn add_item(&mut self, item: Item) {
            // ...
        }

        pub fn calculate_total(&self) -> f64 {
            // ...
        }
    }

    // Хорошо: простая структура данных
    pub struct Order {
        items: Vec<Item>,
    }

    impl Order {
        pub fn new() -> Self {
            Order {
                items: Vec::new(),
            }
        }

        pub fn add_item(&mut self, item: Item) {
            self.items.push(item);
        }

        pub fn calculate_total(&self) -> f64 {
            self.items.iter().map(|item| item.price).sum()
        }
    }
    ```

    3. **Избегайте гибридных структур (половина объект, половина данные)**

    Гибридные структуры, которые смешивают поведение и данные, могут быть трудны для понимания и сопровождения. Стремитесь к тому, чтобы структура данных и объекты имели ясное разделение.

    **Пример:**

    ```rust
    // Плохо: гибридная структура
    pub struct Order {
        items: Vec<Item>,
        pub total: f64,
    }

    impl Order {
        pub fn add_item(&mut self, item: Item) {
            self.items.push(item);
            self.total += item.price; // Гибридный подход
        }
    }

    // Хорошо: разделение данных и поведения
    pub struct Order {
        items: Vec<Item>,
    }

    impl Order {
        pub fn new() -> Self {
            Order {
                items: Vec::new(),
            }
        }

        pub fn add_item(&mut self, item: Item) {
            self.items.push(item);
        }

        pub fn calculate_total(&self) -> f64 {
            self.items.iter().map(|item| item.price).sum()
        }
    }
    ```

    4. **Объекты и структуры данных должны быть маленькими**

    Меньшие объекты и структуры данных легче понимать и поддерживать.

    **Пример:**

    ```rust
    // Плохо: большая структура с множеством переменных и методов
    pub struct Order {
        items: Vec<Item>,
        total: f64,
        // Дополнительные переменные и методы
    }

    // Хорошо: небольшие и сфокусированные структуры
    pub struct Order {
        items: Vec<Item>,
    }

    impl Order {
        pub fn new() -> Self {
            Order {
                items: Vec::new(),
            }
        }

        pub fn add_item(&mut self, item: Item) {
            self.items.push(item);
        }
    }
    ```

    5. **Объекты должны делать одно дело**

    Объекты и структуры данных должны быть сфокусированы на выполнении одной задачи.

    **Пример:**

    ```rust
    // Плохо: объект делает слишком много вещей
    pub struct Order {
        items: Vec<Item>,
        total: f64,
        // Методы для обработки и расчета
    }

    // Хорошо: объект делает только одно дело
    pub struct Order {
        items: Vec<Item>,
    }

    impl Order {
        pub fn add_item(&mut self, item: Item) {
            self.items.push(item);
        }

        pub fn calculate_total(&self) -> f64 {
            self.items.iter().map(|item| item.price).sum()
        }
    }
    ```

    6. **Малое количество переменных экземпляра**

    Минимизируйте количество переменных экземпляра, чтобы объект был более простым и понятным.

    **Пример:**

    ```rust
    // Плохо: много переменных экземпляра
    pub struct Order {
        items: Vec<Item>,
        total: f64,
        discount: f64,
        // Дополнительные переменные
    }

    // Хорошо: минимальное количество переменных
    pub struct Order {
        items: Vec<Item>,
    }

    impl Order {
        pub fn new() -> Self {
            Order {
                items: Vec::new(),
            }
        }

        pub fn add_item(&mut self, item: Item) {
            self.items.push(item);
        }
    }
    ```

    7. **Базовый класс не должен знать о своих производных**

    В объектно-ориентированном программировании базовый класс не должен иметь зависимости от своих производных классов. В Rust это может быть реализовано через использование трейтів и неявных зависимостей.

    **Пример:**

    ```rust
    // Плохо: базовый класс знает о производных
    pub trait Animal {
        fn make_sound(&self) -> String;
    }

    pub struct Dog;

    impl Animal for Dog {
        fn make_sound(&self) -> String {
            "Woof".to_string()
        }
    }

    pub struct Cat;

    impl Animal for Cat {
        fn make_sound(&self) -> String {
            "Meow".to_string()
        }
    }

    // Хорошо: базовый класс не знает о производных
    pub trait Animal {
        fn make_sound(&self) -> String;
    }

    pub struct Dog;

    impl Animal for Dog {
        fn make_sound(&self) -> String {
            "Woof".to_string()
        }
    }

    pub struct Cat;

    impl Animal for Cat {
        fn make_sound(&self) -> String {
            "Meow".to_string()
        }
    }
    ```

    8. **Лучше иметь много функций, чем передавать код в функцию для выбора поведения**

    Разделение кода на множество функций позволяет лучше организовать логику и избежать сложных конструкций, таких как флаговые аргументы.

    **Пример:**

    ```rust
    // Плохо: использование флага для выбора поведения
    pub fn handle_order(order: &Order, use_discount: bool) {
        if use_discount {
            // Применение скидки
        } else {
            // Без скидки
        }
    }

    // Хорошо: использование отдельных функций
    pub fn handle_order_with_discount(order: &Order) {
        // Применение скидки
    }

    pub fn handle_order_without_discount(order: &Order) {
        // Без скидки
    }
    ```

    9. **Предпочитайте нестатические методы статическим**

    Нестатические методы позволяют работать с экземплярами класса и поддерживают полиморфизм, тогда как статические методы не могут использовать состояние экземпляра.

    **Пример:**

    ```rust
    // Плохо: статические методы, которые могут использовать состояние экземпляра
    pub struct Order {
        items: Vec<Item>,
    }

    impl Order {
        pub fn calculate_total(items: &[Item]) -> f64 {
            items.iter().map(|item| item.price).sum()
        }
    }

    // Хорошо: нестатические методы для работы с состоянием экземпляра
    pub struct Order {
        items: Vec<Item>,
    }

    impl Order {
        pub fn add_item(&mut self, item: Item) {
            self.items.push(item);
        }

        pub fn calculate_total(&self) -> f64 {
            self.items.iter().map(|item| item.price).sum()
        }
    }
    ```

* #### Code smells
 
    Кодовые запахи (code smells) — это признаки того, что код может содержать потенциальные проблемы или области для улучшения. Вот основные кодовые запахи и их описание, а также примеры на Rust и способы их устранения:

    1. **Rigidity (Жесткость)**

    **Описание:**
    Программное обеспечение становится трудным для изменения, поскольку небольшое изменение вызывает каскад последующих изменений. Это происходит из-за сильных зависимостей между компонентами системы.

    **Пример на Rust:**

    ```rust
    // Плохо: жесткий код
    struct Order {
        items: Vec<Item>,
        discount: f64,
    }

    impl Order {
        fn new() -> Self {
            Order {
                items: Vec::new(),
                discount: 0.0,
            }
        }

        fn apply_discount(&mut self) {
            let total = self.items.iter().map(|item| item.price).sum::<f64>();
            self.discount = total * 0.1;
        }
    }

    struct Item {
        price: f64,
    }

    // Изменение структуры Item потребует изменения в Order
    ```

    **Как исправить:**

    Используйте абстракции, такие как трейты и интерфейсы, чтобы уменьшить взаимные зависимости.

    ```rust
    trait DiscountStrategy {
        fn apply_discount(&self, total: f64) -> f64;
    }

    struct PercentageDiscount;

    impl DiscountStrategy for PercentageDiscount {
        fn apply_discount(&self, total: f64) -> f64 {
            total * 0.1
        }
    }

    struct Order<T: DiscountStrategy> {
        items: Vec<Item>,
        discount_strategy: T,
    }

    impl<T: DiscountStrategy> Order<T> {
        fn new(discount_strategy: T) -> Self {
            Order {
                items: Vec::new(),
                discount_strategy,
            }
        }

        fn apply_discount(&self) -> f64 {
            let total = self.items.iter().map(|item| item.price).sum();
            self.discount_strategy.apply_discount(total)
        }
    }
    ```

    2. **Fragility (Хрупкость)**

    **Описание:**
    Программное обеспечение ломается в нескольких местах из-за одного изменения. Это часто вызвано слишком сильной связью между компонентами.

    **Пример на Rust:**

    ```rust
    // Плохо: хрупкий код
    struct Order {
        items: Vec<Item>,
    }

    impl Order {
        fn add_item(&mut self, item: Item) {
            self.items.push(item);
            self.update_inventory();
        }

        fn update_inventory(&self) {
            // Логика обновления инвентаря
        }
    }

    struct Item {
        price: f64,
    }
    ```

    **Как исправить:**

    Разделите ответственность между компонентами и используйте принципы инкапсуляции и декомпозиции.

    ```rust
    struct Inventory;

    impl Inventory {
        fn update(&self) {
            // Логика обновления инвентаря
        }
    }

    struct Order {
        items: Vec<Item>,
        inventory: Inventory,
    }

    impl Order {
        fn add_item(&mut self, item: Item) {
            self.items.push(item);
            self.inventory.update();
        }
    }
    ```

    3. **Immobility (Иммобильность)**

    **Описание:**
    Невозможно повторно использовать части кода в других проектах из-за связанных рисков и высоких затрат.

    **Пример на Rust:**

    ```rust
    // Плохо: сложно повторно использовать
    struct Order {
        items: Vec<Item>,
    }

    impl Order {
        fn calculate_total(&self) -> f64 {
            self.items.iter().map(|item| item.price).sum()
        }

        fn apply_discount(&self) -> f64 {
            let total = self.calculate_total();
            total * 0.1
        }
    }
    ```

    **Как исправить:**

    Сделайте ваш код более модульным и независимым, выделяя общие компоненты в библиотеки.

    ```rust
    pub struct Item {
        pub price: f64,
    }

    pub trait DiscountStrategy {
        fn apply_discount(&self, total: f64) -> f64;
    }

    pub struct PercentageDiscount;

    impl DiscountStrategy for PercentageDiscount {
        fn apply_discount(&self, total: f64) -> f64 {
            total * 0.1
        }
    }

    pub struct Order<'a, T: DiscountStrategy> {
        pub items: Vec<Item>,
        discount_strategy: &'a T,
    }

    impl<'a, T: DiscountStrategy> Order<'a, T> {
        pub fn new(discount_strategy: &'a T) -> Self {
            Order {
                items: Vec::new(),
                discount_strategy,
            }
        }

        pub fn calculate_total(&self) -> f64 {
            self.items.iter().map(|item| item.price).sum()
        }

        pub fn apply_discount(&self) -> f64 {
            let total = self.calculate_total();
            self.discount_strategy.apply_discount(total)
        }
    }
    ```

    4. **Needless Complexity (Избыточная сложность)**

    **Описание:**
    Код содержит ненужные усложнения, которые делают его трудным для понимания и поддержки.

    **Пример на Rust:**

    ```rust
    // Плохо: избыточная сложность
    struct Order {
        items: Vec<Item>,
    }

    impl Order {
        fn calculate(&self, tax: f64, discount: f64) -> f64 {
            let subtotal = self.items.iter().map(|item| item.price).sum::<f64>();
            let taxed = subtotal * (1.0 + tax);
            let discounted = taxed - discount;
            discounted
        }
    }
    ```

    **Как исправить:**

    Разделите код на более простые и понятные части.

    ```rust
    struct Order {
        items: Vec<Item>,
    }

    impl Order {
        fn subtotal(&self) -> f64 {
            self.items.iter().map(|item| item.price).sum()
        }

        fn total(&self, tax: f64, discount: f64) -> f64 {
            let subtotal = self.subtotal();
            let taxed = subtotal * (1.0 + tax);
            taxed - discount
        }
    }
    ```

    5. **Needless Repetition (Избыточное повторение)**

    **Описание:**
    Код содержит повторяющиеся фрагменты, что делает его сложным для поддержки и увеличивает вероятность ошибок.

    **Пример на Rust:**

    ```rust
    // Плохо: избыточное повторение
    fn calculate_price(item: &Item, quantity: u32) -> f64 {
        item.price * quantity as f64
    }

    fn calculate_total(order: &Order) -> f64 {
        let mut total = 0.0;
        for item in &order.items {
            total += calculate_price(item, 1); // Количество всегда 1
        }
        total
    }
    ```

    **Как исправить:**

    Используйте общие функции и абстракции для устранения повторений.

    ```rust
    fn calculate_price(item: &Item, quantity: u32) -> f64 {
        item.price * quantity as f64
    }

    fn calculate_total(order: &Order) -> f64 {
        order.items.iter().map(|item| calculate_price(item, 1)).sum()
    }
    ```

    6. **Opacity (Непрозрачность)**

    **Описание:**
    Код трудно понять из-за неясных или запутанных конструкций, что затрудняет его поддержку.

    **Пример на Rust:**

    ```rust
    // Плохо: непрозрачный код
    fn process_data(data: &str) -> String {
        let mut result = String::new();
        for c in data.chars() {
            if c.is_alphabetic() {
                result.push(c.to_ascii_uppercase());
            } else if c.is_numeric() {
                result.push('0');
            }
        }
        result
    }
    ```

    **Как исправить:**

    Упрощайте код и добавляйте комментарии, чтобы сделать его более понятным.

    ```rust
    fn process_data(data: &str) -> String {
        let mut result = String::new();
        
        for c in data.chars() {
            if c.is_alphabetic() {
                result.push(c.to_ascii_uppercase());
            } else if c.is_numeric() {
                result.push('0'); // Заменяем цифры на '0'
            }
        }
        
        result
    }
    ```


* #### Минимизация повторений (DRY - Don't Repeat Yourself):

    Избегайте дублирования кода. Если у вас есть повторяющийся код, вынесите его в отдельную функцию, метод или класс.

* #### Маленькие функции (Small Functions):

    Функции должны быть небольшими и выполнять одну четко определенную задачу.
    Если функция становится слишком большой, разделите ее на более мелкие функции с понятными именами.

* #### Принцип единственной ответственности (Single Responsibility Principle - SRP):

    Каждый класс или функция должны быть ответственными только за одну вещь. Это облегчает понимание и изменение кода.

* #### Принцип открытости/закрытости (Open/Closed Principle - OCP):

    Код должен быть открыт для расширения, но закрыт для модификации. Это достигается путем использования абстракций и полиморфизма.

* #### Соблюдение стандартов форматирования:

    Используйте стандарты форматирования кода, чтобы обеспечить единообразие внутри проекта. Это может включать в себя правила отступов, расположение фигурных скобок и т.д.

* #### Тестирование:

    Пишите тесты для вашего кода, чтобы обеспечить его корректность и устойчивость к изменениям.
    Следуйте принципу "Тестирование приводит к чистому коду".

* #### Если нашли место для рефакторинга, следует это рефакторить:

    При внесении изменений в код, следите за тем, чтобы код оставался чистым или становился чище, чем был до ваших изменений.
    Правило бойскаута. Оставьте место стоянки чище, чем оно было до вас.

* #### Избегание магических чисел и строк:

    Избегайте использования "магических" (хардкодированных) чисел и строк. Используйте константы или переменные с понятными именами.

* #### Комментарии с умеренностью:

    Комментарии должны использоваться только там, где это действительно необходимо для понимания кода. Избегайте лишних или бессмысленных комментариев.
    Эти принципы и многие другие детали описаны в книге Роберта Мартина "Clean Code", которая является отличным ресурсом для разработчиков, стремящихся писать чистый, поддерживаемый и эффективный код.

**p.s.** читать «Совершенный код» (Code Complete) Макконнелла ....


# Семантическая и цикломатическая сложность кода

#### 1. Семантическая сложность

**Семантическая сложность** измеряет сложность понимания кода, учитывая его логику, намерения программиста, и то, как он взаимодействует с другими частями системы. Эта метрика не так легко формализуется, как цикломатическая сложность, и зачастую требует качественной оценки. 

##### Факторы, влияющие на семантическую сложность:
- **Читаемость кода:** Легкость понимания того, что делает код. Хорошие имена переменных и функций, наличие комментариев и четкая структура кода помогают уменьшить семантическую сложность.
- **Уровень абстракции:** Использование различных уровней абстракции и паттернов проектирования может как уменьшить, так и увеличить сложность, в зависимости от того, насколько они понятны программистам, работающим с кодом.
- **Сложность алгоритмов:** Алгоритмы с сложными логическими условиями и многочисленными ветвлениями усложняют понимание кода.
- **Зависимости и взаимодействия:** Множество зависимостей между модулями и сложное взаимодействие между компонентами системы также увеличивают семантическую сложность.

#### 2. Цикломатическая сложность

**Цикломатическая сложность** (Cyclomatic Complexity) измеряет количество независимых путей через программу и дает представление о структурной сложности кода. Эта метрика основана на теории графов, где каждая программа представляется графом потока управления.

##### Расчет цикломатической сложности:
Цикломатическая сложность \( M \) для графа потока управления программы рассчитывается по формуле:
\[ M = E - N + 2P \]
где:
- \( E \) — количество рёбер в графе.
- \( N \) — количество узлов в графе.
- \( P \) — количество связанных компонентов (для большинства программ это обычно 1).

##### Пример:
Для следующего кода:
```python
def example(a, b):
    if a > b:
        return a - b
    elif a < b:
        return b - a
    else:
        return a + b
```

**Граф потока управления:**
1. Начало
2. Условие \( a > b \)
3. Условие \( a < b \)
4. Возврат \( a - b \)
5. Возврат \( b - a \)
6. Возврат \( a + b \)

**Граф:**
- Узлы (N): 6 (включая условия и возвраты)
- Рёбра (E): 7 (каждое условие добавляет по 2 ветки, и один возврат добавляет одну ветку)
- Связанные компоненты (P): 1

\[ M = E - N + 2P = 7 - 6 + 2 \times 1 = 3 \]

Цикломатическая сложность этого примера равна 3, что означает, что в программе три независимых пути.

##### Значение цикломатической сложности:
- **1-10:** Простая программа с низкой сложностью.
- **11-20:** Средняя сложность, потенциально требует рефакторинга.
- **21-50:** Высокая сложность, код трудно поддерживать и тестировать.
- **50+:** Очень высокая сложность, рекомендуется разбиение на более простые модули.

### Заключение

- **Семантическая сложность** связана с человеческим восприятием и пониманием кода. Она учитывает читаемость, ясность и взаимодействие компонентов.
- **Цикломатическая сложность** предоставляет количественную оценку структурной сложности кода и помогает определить потенциальные области для улучшения и упрощения.

Обе метрики важны для обеспечения качества кода, его поддержки и тестируемости.

# Programming Paradigms

#### **Парадигма** - это стиль написания и использования средств языка, могут совмешаться в зыках (мкльтипарадигма)

* **Процедурное программирование** - указание компьютеру что делать (так выглядит C). Используется: `ф-ции, циклы, if`
* **Функциональное программирование** - Haskell интерпритация говорит про понятие ссылочной прозрачности программы
    **Функциональное программирование** - Lisp интерпритация говорит про программы построенную на основе композиции функции:  `fnc().func().func()`. В Rust это итераторы

* Парадигма **Императивное программирование** — последовательность **КОМАНД** которые последовательно выполняются, как магинный код. Акцент на подход **КАК** решать задачу в отличии от декларативного. Все языки являются ииперативными. 

* Парадигма **Процедурное программирование** —  чтобы не дублировать одни и те же **КОМАНД**ы их можно использовать повторно через процедуры(есть входные параметры и нет выходных)/функции(есть входные и выходные параметры). Почти все языке являются процедурными. 

* Парадигма **Функциональное программирование** —  (подкатегория декларативного т.е. акцент на описание спецификации **ЧТО** мы хотим). Более понятен чем императивеая парадигма. Оперирует основными замыканиями, выполняющимися когда их вызовут.
Конструировать программу с помощью функций в которых выход одной функций является входом для другой функций, и комбинируя функций решать задачу.
Поэтому в функциональном программировании есть ограничение на функции, они должны быть чистыми (не изменяет состояние, принимает аргументы и возвращает результат, код функции не выходит за ее собственные пределы). 
В информатике функциональное программирование — это парадигма программирования, в которой программы создаются путем применения и составления функций. Это парадигма декларативного программирования, в которой определения функций представляют собой деревья выражений, каждое из которых возвращает значение, а не последовательность императивных операторов, которые изменяют состояние программы.
Список ф-циональных языков: `LISP, HASKELL, ERLANG, CLOJURE, #F`

* Парадигма **Декларативное программирование**.
Несмотря на то, что исторически первым был применен декларативный подход в программировании, первые языки программирования компьютеров (машинный, ассемблер, фортран, алгол, кобол) были императивными в силу простоты подхода.
Парадигма Декларативное программирование — задаётся спецификация решения задачи: т.е. акцент на **ЧТО** мы хотим, описывается, что представляет собой проблема и ожидаемый результат, но без описания способа КАК достичь этого результата. Пример - `SQL`.
Зачастую декларативные программы не используют понятия состояния и, в частности, не содержат переменных и операторов присваивания, обеспечивая ссылочную прозрачность. 
К подвидам декларативного программирования часто относят и функциональное программирование. 

* Парадигма **Структурное программирование** — появилась необходимость в абстрациях, из-за сложности программ. Парадигма следует особому струтурированию кода: последовательность, ветвления `if/else`, цикл `while`, процедуры и функции, блоки кода. Представитель - язык `C,Go,C#`

* Парадигма **Обьектно-ориентированное программирование (ООП)** — появилась необходимость в абстрациях, из-за сложности программ. Оперирует понятиями класса и обьекта,а также инкапсуляция(упаковка),наследование(расширение),полиморфизм(повторное использование). Дает гибкость в структурировании программ. Это архитектура приложения, способ моделирования предметной области. При котором используется разделение на сущности по принципу недопустить возможность ввести сущность в некорректное состояния.

# Что такое паттерны, зачем и почему?

Способ решения часто встречающихся задач. 
Они делают наше программное обеспечение более модульным, удобным в сопровождении и расширяемым. 
Более того, эти шаблоны предоставляют разработчикам общий язык, что делает их отличным инструментом для эффективного общения при решении проблем в группах.

Лучшее понимание паттернов и алгоритмов, повышает гибкость мышления и качество конечного продукта.
Глобально, Мир станет совершеннее, если перестать **плодить** мусор. Следует различать **создание** мусора в процессе разработки с последующим его рефакторингом, и конечный результат в виде мусора (**плодить**), так как он остается на обозрение пользователей.

# Подходят ли ООП паттерны для Rust?

Rust - статически типизированный язык системного уровня программирования, с парадигмой функционального и процедурного стиля. Используется не только для системного программирования интрументария но и для прикладных программ пользователю, WEB бэкенд и фронтенд, игр, встроенных систем микроконтроллеров.

В информатике функциональное программирование — это парадигма программирования, в которой программы создаются путем применения и составления функций.
Это парадигма декларативного программирования, в которой определения функций представляют собой деревья выражений, каждое из которых возвращает значение, а не последовательность императивных операторов, которые изменяют состояние программы.

Парадигмы - это стили написания кода, в Rust есть возможности структурировать код в функциональном стиле используя методы итератора,вектора или собственными силами соблюдая правила:
неизменяемость данных,чистые функции(не имеют побочных эффектов),композиция функций(объединения нескольких функций в одну),функции высшего порядка(принимают другие функции в качестве аргументов или возвращают),отсутвие операций присвоения для сохранения промежуточных результатов вызовов функций.

Так же сможем придерживаться процедурного стиля, компоновать код для повторного использования с применением присвоения для сохранением промежуточных данных между вызовами процедур. 
И это не запрещает нам структурировать код в ООП стиле (нет наследования данных и реализаций, только интерфейсов через super-trait, есть инкапсуляция c помошью mod и есть полиморфизм c помощью trait-object и параметрический полиморфизм дженериков и ограничение трейтами). 
Сдедовательно, только наши цели (предметная область) и ограничения или особенности языка (идиомы), задают каким образом использовать паттерны.  

**Императив**
```rust
let mut sum = 0;
for i in 1..11 {
    sum += i;
}
println!("{}", sum);
```

**Декларативный**
```rust
println!("{}", (1..11).fold(0, |a, b| a + b));
// `fold` это функция, которая составляет функции.
```
Помните, что в декларативных программах мы описываем, **ЧТО** делать, а не **КАК** это делать. 

Также благодаря `static dispatch` мы можем вынести потенциальные ошибки использования типов на этап компиляции кода.  
Система типов Rust может превратить многие виды проблем программирования в проблемы `static dispatch`. 
Это одно из самых больших преимуществ при выборе функционального языка, и оно имеет решающее значение для многих гарантий времени компиляции Rust.
В Rust параметр универсального типа создает то, что в функциональных языках называется «**ограничением класса типа**».
Это называется **мономорфизацией**, когда разные типы создаются из полиморфного кода.

**Альтернативы**

Если типу требуется «раздельнный API» из-за конструкции или частичной инициализации, вместо этого рассмотрите **Builder Pattern**.

Если API между типами не меняется — меняется только поведение — тогда лучше использовать **Strategy Pattern** 
(в Rust нет необходимости в шаблоне стратегии , потому что мы можем просто использовать трейты :thinking:).

# Rust idioms

#### Рекомендации, которым следует следовать при кодировании. Это согласованные нормы общества. Нарушать их следует только в том случае, если у вас есть для этого веская причина.

- Type safety (Newtype, Typestates) 

- Mem replace (hook lifetime)

- Bound impl (ограничение поведения)

- Generic in type out (абстракция аргументов)

- Exhaustivity (проверка на полноту вариантов)

- Sealing (запечатывание реализации)

- Конструктор с помощью ф-ции `new` и конструктор по умолчанию `Default`

- Динамическая диспетчеризация на стеке

- Передача переменных в замыкание 

- Временная мутабельность

- Возвращать использованный аргумент при ошибке

[Rust idioms rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms)

[Rust idioms rust-unofficial](https://rust-unofficial.github.io/patterns/idioms/index.html)

[Rust idioms rust-unofficial github](https://github.com/Jekahome/RustDesignPatterns/tree/main/src/idioms)

[Реализация конечного автомата (переходы между состояниями)](https://hoverbear.org/blog/rust-state-machine-pattern/)

[Typestate шаблон Состояние](https://doc.rust-lang.ru/book/ch17-03-oo-design-patterns.html)

# Anti patterns

#### Методы решения распространенных проблем при кодировании. Однако, хотя шаблоны проектирования дают нам преимущества, антишаблоны создают больше проблем.

- Клонирование для проверки заимствований (borrow checker)

- Использование подавление предупреждений компилятора при сборке

- Неправильное использование трейта `Deref`

- Золотой молоток (Golden hammer)

- Мягкое кодирование (Soft code)/ Жёсткое кодирование (Hard code)

[Anti patterns sourcemaking.com](https://sourcemaking.com/antipatterns)

- [Антипаттерны разработки программного обеспечения](https://sourcemaking.com/antipatterns/software-development-antipatterns)

- [Антипаттерны архитектуры программного обеспечения](https://sourcemaking.com/antipatterns/software-architecture-antipatterns)

- [Управление программными проектами](https://sourcemaking.com/antipatterns/software-project-management-antipatterns)

[Что такое анти-паттерны?](https://habr.com/ru/articles/59005/)


[Анемичная модель](https://www.martinfowler.com/bliki/AnemicDomainModel.html) - антипаттерн

Предпочитайте нормальную модель бизнес логики(иногда называют Rich Domain Model), а не процедурную, анемичную модель с DTO вместо полноценных сущностей. Анемичная модель не имеет ничего общего с ООП и ее следует рассматривать как неудачный пример процедурного программирования.


# Design principles: SOLID, KISS, DRY, YAGNI, GRASP, LoD, SoC

#### SOLID

Принципы SOLID — это набор из пяти принципов проектирования, введенных Робертом Мартином, призванных прояснить изначальные ограничения объектно-ориентированного программирования и сделать программы более гибкими и адаптируемыми.

##### Принцип единой ответственности (**SRP**)

Принцип единой ответственности (**SRP**): класс должен иметь только одну ответственность, то есть только изменения в одной части спецификации программного обеспечения должны иметь возможность повлиять на спецификацию класса.

«Соберите вместе вещи, которые изменяются по одним и тем же причинам. Разделите те вещи, которые изменяются по разным причинам»

Это очень похоже на SoC, не правда ли? Разница между этими двумя принципами в том, что SRP нацелен на разделение на уровне классов, в то время как SoC — это общий подход, который работает как на высоком уровне (например, слои, системы, сервисы), так и на низком уровне (классы, функции и т. д.) абстракции.

Принцип единой ответственности обладает всеми преимуществами SoC, в частности, он способствует высокой связности и низкой связанности, а также позволяет избежать анти-шаблона [«божественного объекта»](https://en.wikipedia.org/wiki/God_object)

##### Принцип открытости/закрытости (**OCP**)

«Программные объекты… должны быть открыты для расширения, но закрыты для модификации».

При внедрении новой функции следует избегать внесения в существующий код изменений, нарушающих его работу.

Класс считается открытым , когда вы можете расширить его и добавить необходимые изменения. Класс считается закрытым , когда он имеет четко определенные интерфейсы и не будет меняться в будущем, т. е. он доступен для использования в другом фрагменте кода.

Представьте себе классическое наследование ООП: вы создали родительский класс, а затем позже расширили его дочерним классом с дополнительной функциональностью. Затем по какой-то причине вы решили изменить внутреннюю структуру родительского класса (например, добавить новое поле или удалить какой-то метод), которая также доступна или напрямую влияет на производный класс. Делая это, вы нарушаете этот принцип, потому что теперь вам не только нужно изменить родительский класс, но и адаптировать дочерний класс для новых изменений. Это происходит из-за того, что сокрытие информации не применяется должным образом. Вместо этого, если вы даете дочернему классу стабильный контракт через открытое свойство или метод, вы можете свободно изменять свою внутреннюю структуру, пока это не влияет на этот контракт.

Это поощряет зависимость клиента от абстракции (например, интерфейса или абстрактного класса), а не от реализации (конкретного класса). Действуя таким образом, клиент, который зависит от абстракции, считается закрытым, но в то же время он открыт для расширения, поскольку все новые модификации, которые соответствуют этой абстракции, могут быть бесшовно интегрированы для клиента.


##### Принцип замены Лискова (**LSP**)

«Объекты в программе должны быть заменены экземплярами их подтипов без изменения корректности этой программы».

Проще говоря, когда вы расширяете класс, вы не должны нарушать установленный в нем контракт. Под «нарушением контракта» подразумевается невыполнение одного из следующих требований:

1. Не изменяйте параметры в производных классах: дочерние классы должны соответствовать сигнатурам методов родительского класса, т. е. принимать те же параметры, что и родительский класс, или принимать более абстрактные параметры.
2. Не изменяйте тип возвращаемого значения в производных классах: дочерние классы должны возвращать тот же тип, что и родительский класс, или возвращать более конкретные (подтипные) параметры.
3. Не выбрасывайте исключение в производных классах: дочерние классы не должны выбрасывать исключение в своих методах, если только родительский класс этого не делает. В этом случае тип исключения должен быть тем же или быть подтипом исключения родителя.
4. Не усиливайте предварительные условия в производных классах: дочерние классы не должны изменять поведение ожидающего клиента, ограничивая его работу каким-либо условием, например, в родительском классе вы принимаете строку, но в дочернем классе вы принимаете строку длиной не более 100 символов.
5. Не ослабляйте постусловия в производных классах: дочерние классы не должны изменять поведение ожидающего клиента, позволяя отказаться от какой-либо работы, например, не очищать состояние после операции, не закрывать сокет и т. д.
6. Не ослабляйте инварианты в производных классах: дочерние классы не должны изменять условия, определенные в родительском классе, например, не переназначайте поле родительского класса, поскольку вы можете не осознать всю логику этого.

##### Принцип разделения интерфейсов (**ISP**)

«Интерфейс отвечающий за одну область задач лучше, чем один интерфейс общего назначения».

Любой код не должен зависеть от методов, которые ему не нужны. Если клиент не использует какое-то поведение объекта, почему он должен быть вынужден зависеть от него? Аналогично, если клиент не использует какие-то методы, почему реализатор должен быть вынужден предоставлять эту функциональность?

Разбейте «толстые» интерфейсы на более конкретные. Если вы измените конкретный интерфейс, эти изменения не повлияют на не связанных с ним клиентов.

##### Принцип инверсии зависимостей (**DIP**) 

Нужно «зависеть от абстракций, а не от конкретики».

Дядя Боб описал этот принцип как строгое следование OCP и LSP:

«В этой колонке мы обсуждаем структурные последствия OCP и LSP. Структура, которая возникает в результате строгого использования этих принципов, может быть обобщена в принцип сам по себе. Я называю его «Принцип инверсии зависимости» (DIP)». — Роберт Мартин.

Инверсия зависимости состоит из двух основных утверждений:
1. Модули высокого уровня не должны зависеть от модулей низкого уровня. Оба должны зависеть от абстракций
2. Абстракции не должны зависеть от деталей. Детали должны зависеть от абстракций.

#### [KISS](https://en.wikipedia.org/wiki/KISS_principle)

Большинство систем работают лучше всего, если их сохранять простыми, а не усложнять; следовательно, простота должна быть ключевой целью проектирования, и следует избегать ненужной сложности.
Простота кода – превыше всего, потому что простой код – наиболее понятный.
Если вы используете паттерн проектирования там, где нет проблемы, которую решает данный паттерн – то вы нарушаете KISS, внося ненужные усложнения в код. 
Если вы НЕ используете паттерн проектирования там, где есть проблема, соответствующая паттерну – то вы опять-таки нарушаете KISS, делая код сложнее, чем он мог бы быть.

Фраза KISS (будь проще, глупый) была придумана авиаинженером Келли Джонсоном, который поставил перед своей командой инженеров задачу: реактивный самолет, который они проектируют, должен быть ремонтопригоден среднестатистическим механиком в полевых условиях в боевых условиях, используя только специальные инструменты.
Основная идея заключается в том, чтобы сосредоточиться на простоте системы, что повышает ее понимание и снижает излишнюю сложность, используя только те инструменты, которые вам действительно нужны. [link](https://levelup.gitconnected.com/the-20-essential-principles-of-software-development-lod-soc-solid-and-beyond-7a39a98b685d)

[KISS — принцип проектирования, содержащий все остальные принципы проектирования](https://habr.com/ru/articles/249639/)

#### DRY

"**Не повторяйте себя.**"

Короче говоря, всякий раз, когда вы ловите себя на том, что пишете один и тот же код дважды, у вас есть возможность стать более эффективным.
Следование этому принципу означает, что ваша цель — **сократить количество повторяющихся шаблонов, дублирования кода, логики, в пользу модульного кода, на который можно ссылаться т.е. использовать повторно**.

В книге «Программист-прагматик» мы можем увидеть такое определение DRY:
«Каждая часть знаний должна иметь единственное, однозначное и авторитетное представление в системе»

Это означает, что у вас не должно быть дублированного кода. 
Легче поддерживать код, который находится только в одном месте, потому что если вам нужно что-то изменить в коде, вам просто нужно изменить это в одном месте. 
Кроме того, если у вас есть один и тот же код в двух или более местах, вероятность того, что этот код со временем станет другим, высока, и когда это произойдет, это станет простым способом внести ошибки в вашу систему.
Дублированный код также делает код более сложным и неоправданно большим.

#### YAGNI 

"**Тебе это не понадобится**"

Это означает, что вам не следует реализовывать функциональность только потому, что вы думаете, что она вам когда-нибудь понадобится, а реализовывать ее только тогда, когда она вам действительно понадобится. Поступая так, вы избежите траты времени на реализации, которые даже не были необходимы и, возможно, никогда не будут использоваться.

Закладывая функциональность для лучшей адаптации под будущие возможные требования вы нарушаете еще и принцип KISS (увеличивая избыточную сложность решения), так как вы не подозреваете о возможных последствиях обслуживании и отладки этого кода.

[Design principles](https://rust-unofficial.github.io/patterns/additional_resources/design-principles.html)

[dry-kiss-yagni-principles](https://henriquesd.medium.com/dry-kiss-yagni-principles-1ce09d9c601f)

#### GRASP

Общие принципы распределения ответственности (GRASP) — это набор из девяти принципов, используемых в объектно-ориентированном проектировании, представленных Крейгом Ларманом в его книге «Применение UML и шаблонов».

1. **Information Expert** (Информационный эксперт) Шаблон определяет базовый принцип распределения обязанностей:

Обязанность должна быть назначена тому, кто владеет максимумом необходимой информации для исполнения — информационному эксперту.

Этот шаблон — самый очевидный и важный из девяти. 

Это способствует **уменьшению зависимости между объектами и повышает инкапсуляцию**.

Если его не учесть — получится спагетти-код, в котором трудно разобраться.

Локализация же обязанностей, проводимая согласно шаблону:

Повышает:

* Инкапсуляцию;
* Простоту восприятия;
* Готовность компонентов к повторному использованию;

Снижает: 

* Степень зацеплений.

2. **Creator** (Создатель)

Шаблон определяет, какой объект должен быть ответственным за создание экземпляров другого объекта. 

Обычно это объект, который:
* Использует создаваемый объект,
* Обладает агрегированными данными для создаваемого объекта,
* Является родительским для создаваемого объекта,
* Хранит или записывает экземпляры создаваемого объекта.
 
Альтернатива — шаблон «Фабрика» (создание объектов концентрируется в отдельном классе).

3. **Controller** (Контроллер)

Этот шаблон определяет объект, который принимает и координирует выполнение операций.

Контроллеры обычно представляют собой объекты, управляющие жизненным циклом других объектов.

Отвечает за операции, запросы на которые приходят от пользователя, и может выполнять сценарии одного или нескольких вариантов использования (например, создание и удаление).

Не выполняет работу самостоятельно, а **делегирует** компетентным исполнителям;

Может представлять собой:

* Систему в целом;
* Подсистему;
* Корневой объект;
* Устройство.

4. **Low Coupling** (Низкая связанность)

Принцип низкой связанности предполагает **минимизацию зависимостей между классами**. Чем меньше классы зависят друг от друга, тем легче их модифицировать и тестировать.

«Степень зацепления» (сопряжения[2]) — мера неотрывности элемента от других элементов (либо мера данных, имеющихся у него о них).

«Слабое» зацепление — распределение обязанностей и данных, обеспечивающее взаимную независимость классов. Класс со «слабым» зацеплением:

* Не зависит от внешних изменений;
* Прост для повторного использования.

5. **High Cohesion** (Высокая когезия)

Принцип высокой когезии направлен на то, чтобы классы и модули были сфокусированы на выполнении небольшого количества связанных задач. 
Это делает классы более простыми для понимания и повторного использования.

Если класс имеет низкую связность, это означает, что он выполняет работу, не связанную с его основным назначением, или выполняет работу, которую можно делегировать другой подсистеме.

Т.е. не реализовывать большой функционал в один класс,а вместо этого разнести на несколько классов.

Предметные области следует разделять по классам.

Связность класса — мера подобия предметных областей его методов:

«Высокая» степень — сфокусированные подсистемы (предметная область определена, управляема и понятна);

«Низкая» степень — абстрактные подсистемы. 

Затруднены:
* Восприятие;
* Повторное использование;
* Поддержка;
* Устойчивость к внешним изменениям.

6. **Polymorphism** (Полиморфизм)

Использование полиморфизма позволяет заменить условные операторы (например, `if` или `switch`) вызовами методов, которые могут быть реализованы различными способами в разных классах. 
Это улучшает расширяемость и изменяемость кода.

Устройство и поведение системы:

* Определяется данными;
* Задано полиморфными операциями её интерфейса.

Пример: Адаптация коммерческой системы к многообразию систем учёта налогов может быть обеспечена через внешний интерфейс объектов-адаптеров (смотрите также: Шаблон «Адаптеры»).

7. **Pure Fabrication** (Чистая фабрикация)

Это создание класса, не являющегося частью реального мира, для обеспечения высокой когезии и низкой связанности. 
Например, это может быть класс, реализующий функциональность, которая не может быть естественно присвоена другим объектам.

Не относится к предметной области, но:
* Уменьшает зацепление;
* Повышает связность;
* Упрощает повторное использование.

«Pure Fabrication» отражает концепцию сервисов в модели проблемно-ориентированного проектирования.

Пример задачи: Не используя средства класса «А», внести его объекты в базу данных.

Решение: Создать класс «Б» для записи объектов класса «А» (смотрите также: «Data Access Object»).

8. **Indirection** (Косвенность)

Этот шаблон вводит посредника для управления взаимодействием между объектами, с целью достижения низкой связанности и высокой когезии. 

Например, использование паттерна "Посредник" (Mediator) для управления коммуникацией между модулями.
 
Слабое зацепление между элементами системы (и возможность повторного использования) обеспечивается назначением промежуточного объекта их посредником.

Пример: В архитектуре Model-View-Controller, контроллер (англ. controller) ослабляет зацепление данных (англ. model) за их представление (англ. view).

9. **Protected Variations** (Устойчивость к изменениям)   

Принцип, при котором система разрабатывается таким образом, чтобы защитить части системы от влияния вариаций в других частях. Это может быть достигнуто с помощью интерфейсов, абстракций и других методов.
Шаблон защищает элементы от изменения другими элементами (объектами или подсистемами) с помощью вынесения взаимодействия в фиксированный интерфейс, через который (и только через который) возможно взаимодействие между элементами. Поведение может варьироваться лишь через создание другой реализации интерфейса.

#### LoD

The Law of Demeter (LoD) [Закон Деметры](https://backendinterview.ru/architecture/principles.html#%D0%97%D0%B0%D0%BA%D0%BE%D0%BD-%D0%94%D0%B5%D0%BC%D0%B5%D1%82%D1%80%D1%8B)

Принцип Деметры (или Law of Demeter, сокращенно LoD) — это принцип проектирования программного обеспечения, направленный на минимизацию связности между различными компонентами системы. Он также известен как "принцип наименьшего знания".

**Суть принципа:**
Объект должен взаимодействовать только с теми объектами, которые он непосредственно знает и с которыми связан, а не с объектами, которые являются "посредниками" или находятся на уровне глубже.

**Простыми словами:**
Каждый объект должен знать о структуре других объектов как можно меньше. То есть объект не должен обращаться напрямую к внутренностям других объектов и передавать управление дальше по цепочке.

Говоря упрощённо, каждый программный модуль:

* должен обладать ограниченным знанием о других модулях: знать о модулях, которые имеют «непосредственное» отношение к этому модулю.

* должен взаимодействовать только с известными ему модулями «друзьями», не взаимодействовать с незнакомцами.
обращаться только к непосредственным «друзьям».

Аналогия из жизни: Если Вы хотите, чтобы собака побежала, глупо командовать её лапами, лучше отдать команду собаке, а она уже разберётся со своими лапами сама.

Основной идеей является то, что объект должен иметь как можно меньше представления о структуре и свойствах чего угодно (включая собственные подкомпоненты).

Общее описание правила: Объект A не должен иметь возможность получить непосредственный доступ к объекту C, если у объекта A есть доступ к объекту B и у объекта B есть доступ к объекту C.

Более формально, Закон Деметры для функций требует, что метод М объекта О должен вызывать методы только следующих типов объектов:

* собственно самого О
* параметров М
* других объектов, созданных в рамках М
* прямых компонентных объектов О
* глобальных переменных, доступных О, в пределах М

Практически, объект-клиент должен избегать вызовов методов объектов, внутренних членов, возвращенных методом объекта-сервиса.

В общем случае можно сказать, что LoD не работает, когда к одному объекту применено более двух точек, например, `object.friend.stranger` вместо `object.friend` или такое нарушение принципа `String cityName = person.getAddress().getCity().getName();`

#### SoC

**SoC (Separation of Concerns)** — это принцип разделения обязанностей или ответственности в программной инженерии, который предполагает, что разные части программы должны решать строго определённые задачи и быть независимыми друг от друга. 

**Суть принципа:**
Каждый компонент системы должен отвечать только за одну "зону ответственности" или "аспект". Эти зоны ответственности должны быть максимально разделены, чтобы изменения в одной зоне не затрагивали другие.

**Основные идеи SoC:**
1. **Каждый модуль или компонент должен иметь свою четкую цель**.
2. **Независимость компонентов**: Компоненты с разными зонами ответственности должны быть изолированы и не зависеть друг от друга.
3. **Легкость изменений**: Изменения в одной зоне не должны приводить к изменению других частей системы, что повышает гибкость и упрощает сопровождение.
4. **Повторное использование кода**: Компоненты, отвечающие за одну задачу, проще повторно использовать в других системах или контекстах.

**Примеры применения SoC:**
1. **Архитектурные паттерны**:
   - **MVC (Model-View-Controller)**: Яркий пример применения SoC. Модель отвечает за логику данных, представление — за отображение данных, контроллер — за управление потоком данных между моделью и представлением.
   - **Микросервисы**: Каждая служба отвечает за конкретную часть системы, что позволяет разделить различные зоны ответственности (например, учет пользователей, обработка платежей и т.д.).

2. **Модули и библиотеки**:
   В больших проектах различная функциональность может быть разделена на независимые модули или библиотеки. Например, один модуль отвечает за работу с базой данных, другой — за взаимодействие с внешними API.

3. **UI и логика**:
   Разделение логики пользовательского интерфейса и бизнес-логики также является примером SoC. UI-компоненты отвечают только за визуализацию и взаимодействие с пользователем, а бизнес-логика реализует обработку данных и принятие решений.

**Преимущества SoC:**
1. **Поддерживаемость**: Поскольку каждая часть системы отвечает только за одну задачу, изменения и ошибки проще локализовать.
2. **Гибкость**: Разделенные компоненты можно изменять, заменять или улучшать независимо друг от друга.
3. **Тестируемость**: Компоненты проще тестировать по отдельности, так как они изолированы от других частей системы.
4. **Повторное использование**: Компоненты с четкими зонами ответственности легче использовать повторно в других проектах.

**Пример:**
В приложении для онлайн-покупок можно выделить несколько зон ответственности:
- Модуль для управления пользователями (регистрация, аутентификация).
- Модуль для управления товарами (каталог, поиск).
- Модуль для обработки платежей.
- Модуль для обработки заказов.

Каждая из этих частей имеет свою зону ответственности и может развиваться независимо от других.

**Итог:**
**SoC** — это один из ключевых принципов при разработке программного обеспечения, который помогает построить масштабируемую, гибкую и легко поддерживаемую архитектуру. Разделение ответственности способствует уменьшению сложности системы и улучшает её структуру.

# Gangs of Four (GoF) Design Patterns

#### Методы решения распространенных проблем при кодировании.

Шаблоны проектирования `GoF` делятся на три категории:

- **Порождающие** паттерны связанны с созданием объекта. `Singleton, Builder, Factory, Fabric Method, Prototype, Fold`

- **Структурные** паттерны связаны со структурой классов, такой как наследование и композиция. `Adapter, Bridge, Composite, Decorator, Facade, Proxy, Flyweight`

- **Поведенческие** паттерны обеспечивают решение для лучшего взаимодействия между объектами, обеспечения потери связнности и гибкости для легкого расширения в будущем. `Chain of Responsibility, Command, Interpreter, Iterator, Mediator, Memento, Observer, State, Strategy, Template Method, Visitor`

## <ins>Порождающие паттерны</ins>

Паттерны которые создают новые объекты, или позволяют получить доступ к уже существующим. 
То есть те шаблоны, по которым можно создать новый автомобиль и как это лучше сделать.

- ## Builder	

`Pattern Builder` — это шаблон проектирования, который позволяет шаг за шагом создавать сложные объекты. Он позволяет создавать разные типы и представления объекта, используя один и тот же код построения, отделяя построение сложного объекта от его представления.

Создайте объект с помощью вызовов помощника-строителя.
`Pattern Builder` — это порождающий паттерн проектирования, который позволяет создавать сложные объекты пошагово. 
`Pattern Builder` даёт возможность использовать один и тот же код строительства для получения разных представлений объектов.
`Pattern Builder` особенно подходит, когда при построении T есть побочные эффекты, такие как создание потока или запуск процесса.
Полезно, когда в противном случае вам потребовалось бы много конструкторов или когда конструкция имеет побочные эффекты.

Преимущества
Отделяет методы построения от других методов.
Предотвращает распространение конструкторов.
Может использоваться для однострочной инициализации, а также для более сложной конструкции.

Этот шаблон чаще встречается в Rust (и для более простых объектов), чем во многих других языках, поскольку в Rust отсутствует перегрузка. 
Поскольку у вас может быть только один метод с заданным именем, иметь несколько конструкторов в Rust менее удобно, чем в C++, Java или других.

Этот шаблон часто используется там, где объект-строитель полезен сам по себе, а не просто является строителем. 
Например, см [std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html)
```
    Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")

```

[Builder rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)

[Builder web.archive.org](https://web.archive.org/web/20210104103100/https://doc.rust-lang.org/1.12.0/style/ownership/builders.html)

[Builder www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/builder-design-pattern-in-rust.html)

[Builder www.lurklurk.org](https://www.lurklurk.org/effective-rust/builders.html)

- ## Fold

Создание новой коллекции посредством изменения элементов применив соответсвующий реализованный алгоритм.

`Pattern Fold` аналогичен `Pattern Visitor`, но создает новую версию посещенной структуры данных.
Они разделяют концепцию обхода структуры данных, выполняя операцию на каждом узле. 
Однако посетитель не создает новую структуру данных и не использует старую.

**Мотивация**

Обычно требуется отобразить структуру данных, выполнив некоторую операцию над каждым узлом структуры. 
Для простых операций с простыми структурами данных это можно сделать с помощью `Iterator::map`. 
Для более сложных операций, возможно, когда более ранние узлы могут повлиять на операцию на более поздних 
узлах или где итерация по структуре данных нетривиальна, использование шаблона свертывания является более подходящим.

Как и `Pattern Visitor`, `Pattern Fold` позволяет нам отделить обход структуры данных от операций, выполняемых с каждым узлом.

**Обсуждение**

Такое отображение структур данных распространено в функциональных языках. 
В объектно-ориентированных языках более распространено изменение структуры данных на месте. 
«Функциональный» подход распространен в Rust, в основном из-за предпочтения неизменяемости. 
Использование новых структур данных вместо изменения старых в большинстве случаев упрощает анализ кода.

Компромисс между эффективностью и возможностью повторного использования можно изменить, изменив способ приема узлов методами fold_*.

В приведенном выше примере мы работаем с Boxуказателями. Поскольку они владеют исключительно своими данными, 
исходную копию структуры данных нельзя использовать повторно. 
С другой стороны, если узел не изменяется, его повторное использование очень эффективно.

Если бы нам пришлось работать с заимствованными ссылками, исходную структуру данных можно было бы использовать повторно; 
однако узел необходимо клонировать, даже если он не изменился, что может быть дорогостоящим.

Использование указателя с подсчетом ссылок дает лучшее из обоих миров: мы можем повторно использовать исходную структуру данных, 
и нам не нужно клонировать неизмененные узлы. 
Однако они менее эргономичны в использовании и означают, что структуры данных не могут быть изменены.

У итераторов есть fold метод, однако он сворачивает структуру данных в значение, а не в новую структуру данных. 

[Fold rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/patterns/creational/fold.html)

- ## Abstract factory	

Его основное назначение - предоставить интерфейс для создания семейства взаимосвязанных объектов, не специфицируя их классы.
Используется в тех случаях, когда необходимо изменять поведение системы, варьируя создаваемыми объектами, при этом сохраняя интерфейсы. 
Он позволяет создавать группы взаимосвязанных объектов, реализующих общее поведение. 
Например, в зависимости от конкретных условий

[Abstract factory www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/abstract-factory-design-pattern-in-rust.html)

- ## Factory method	
 
Вместо прямого вызова конструктора объекта для создания объекта используется фабричный метод, что обеспечивает большую гибкость и разделение обязанностей.
В оличии от `Abstract Factory` которая использует фабрику для создания всего набора обьектов, то `Factory Method` использует только один метод для создания единственного обьекта.

`Factory Method`
фабрике заранее неизвестно, объекты каких подклассов ему нужно создавать.
фабрика проектируется так, чтобы объекты, которые она создаёт, определялись ее подклассами.
т.е. делегирует свои обязанности одному из нескольких вспомогательных подклассов.

 
`Factory Method` избавляют проектировщика от необходимости встраивать в код классы выаолняющие создание кокнретных реализаций.
Также используется когда клиент не имеет прав или доступа или к деталям реализации классов.(инкапсуляция создания конкретных типов обьектов)
Фабричный метод используется, когда продуктам не нужно знать, как они созданы.

**Применимость**

- Когда заранее неизвестны типы и зависимости объектов, с которыми должен работать ваш код.
  `Factory Method` отделяет код производства продуктов от остального кода, который эти продукты использует.

- Когда вы хотите экономить системные ресурсы, повторно используя уже созданные объекты, вместо порождения новых.
  Т.е. `Factory Method` может еще управлять логикой создания обьектов, взаимодействуя с хранилищем `Redis,Json,...` этих обьектов.

**Отношения с другими паттернами**

Многие архитектуры начинаются с применения `Factory Method` (более простого и расширяемого через подклассы) 
и эволюционируют в сторону `Abstract Factory`, `Pattern Prototype` или `Pattern Builder` (более гибких, но и более сложных).

Классы `Abstract Factory` чаще всего реализуются с помощью `Factory Method`, хотя они могут быть построены и на основе `Pattern Prototype`.

[Factory method refactoring.guru](https://refactoring.guru/ru/design-patterns/factory-method)

[Factory method sourcemaking.com](https://sourcemaking.com/design_patterns/factory_method)

[Factory method chercher.tech](https://chercher.tech/rust/factory-design-pattern-rust)

[Factory method www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/factory-method-design-pattern-in-rust.html)

- ## Prototype	

`Pattern Prototype` инкапсуляция создания клона. Создание объектов на основе шаблона существующего объекта посредством клонирования, не делая систему зависимой от его классов.

**Проблема**

У вас есть объект, который нужно скопировать. 
Как это сделать? Нужно создать пустой объект такого же класса, а затем поочерёдно скопировать значения всех полей из старого объекта в новый.
Но у нас может не быть доступа к приватных полям. И копирующий код становиться жество зависим от конкретного копируемого класса.

`Pattern Prototype` хочет что бы обьекты создавали свои копии самостоятельно и использовали эдиный интерфейс `method clone()`
Объект, который копируют, называется прототипом (откуда и название паттерна)
Также есть место для инкапсуляции логики создания клона

Если создание объекта требует много времени и средств, и у вас уже есть наиболее похожий экземпляр объекта, 
тогда вы клонируете уже приготовленный с его текущим состоянием.

Если вам нужна глубокая копия, вы можете использовать сериализацию в качестве хитрости, чтобы выполнить глубокую копию. 

Экземпляры класса могут иметь лишь несколько различных комбинаций состояний, и создание новых экземпляров обходится дороже, чем копирование существующего.

[Prototype refactoring.guru](https://refactoring.guru/ru/design-patterns/prototype)

[Prototype chercher.tech](https://chercher.tech/rust/prototype-design-pattern-rust)

- ## Singelton	

`Pattern Singleton` (Одиночка) применяется в том случае, когда какой-либо класс может иметь только один экземпляр (или не иметь ни одного) и легко доступен из глобальной видимости.
 
`Pattern Singleton` нарушает принцип единственной ответственности класса (SRP), так как он доступен глобально для различных частей программы следовательно есть вероятность изменения в одной из частей.

`Pattern Singleton` решает сразу две проблемы:

1. Гарантирует наличие единственного экземпляра класса. Чаще всего это полезно для доступа к какому-то общему ресурсу, например, базе данных.

Представьте, что вы создали объект, а через некоторое время пробуете создать ещё один. В этом случае хотелось бы получить старый объект, вместо создания нового.

Такое поведение невозможно реализовать с помощью обычного конструктора, так как конструктор класса всегда возвращает новый объект.

2. Предоставляет глобальную точку доступа. Это не просто глобальная переменная, через которую можно достучаться к определённому объекту.  

Недостатки:

Модульное тестирование клиентского кода Singleton может быть затруднено, поскольку многие среды тестирования полагаются на наследование при создании фиктивных объектов. Поскольку конструктор класса-одиночки является закрытым и переопределение статических методов невозможно в большинстве языков, вам нужно будет придумать творческий способ имитировать синглтон. Или просто не пишите тесты. Или не используйте шаблон Singleton.

Требует специальной обработки в многопоточной среде (хотя не мутирующий обьект созданный с одинаковыми данными может иметь клонов и будет считаться как Singleton)

[Singleton refactoring.guru](https://refactoring.guru/design-patterns/singleton/rust/example#example-1)

## <ins>Структурирующие паттерны</ins>

Данные паттерны помогают внести порядок и научить разные объекты более правильно взаимодействовать друг с другом.

- ## Adapter	

`Pattern Adapter` работает как мост между двумя несовместимыми интерфейсами. 
Он действует как мост между двумя несовместимыми интерфейсами, предоставляя оболочку, позволяющую использовать один объект так, как если бы это был другой.
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

`Pattern Brige` проектируют загодя, чтобы развивать большие части приложения отдельно друг от друга. 
`Pattern Adapter` применяется постфактум, чтобы заставить несовместимые классы работать вместе.
`Pattern Adapter` заставляет вещи работать после того, как они были спроектированы; `Pattern Brige` заставляет их работать изначально.

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

![Adapter](https://github.com/Jekahome/Patterns/blob/main/_img/Adapter.png "Adapter")

[Adapter sourcemaking.com](https://sourcemaking.com/design_patterns/adapter)

[Adapter refactoring.guru](https://refactoring.guru/ru/design-patterns/adapter)

[Adapter www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/adapter-design-pattern-in-rust.html)

- ## Bridge

`Pattern Brige` - отделение интерфейса/абстракции (группа поведений) от реализации (конкретное поведение из группы), позволяя им изменяться независимо. 
Предполагает разбиение монолитной системы на две отдельные иерархии: абстракцию и реализацию.

`Pattern Brige` используется, чтобы избежать увеличения количества подклассов, к которому в конечном итоге могут привести механизмы наследования. 
Итак, если у вас есть, скажем, 2 ортогональные обязанности, то вместо создания 2**2 подклассов вы используете композицию для объединения этих обязанностей.

Учитывая, что любое изменение, внесенное в абстракцию, повлияет на все классы, которые ее реализуют, 
`Pattern Brige` предлагает добавить **новый уровень абстракции** между обоими элементами, который позволяет разрабатывать каждый из них независимо. 

**Отношения с другими паттернами**

Основное различие между `Pattern Brige` и `Pattern Adapter` заключается в том, что `Pattern Adapter` используется 
для унификации уже существующих интерфейсов, а `Pattern Brige` используется, когда есть подозрение, что реализация интерфейса со временем изменится.
Т.е. своевременное использование `Pattern Brige` избавит нас от необходимости внедрять `Pattern Adapter`

Bridge Before:

![BridgeBefore](https://github.com/Jekahome/Patterns/blob/main/_img/BridgeBefore.png "BridgeBefore")

Bridge After:

![BridgeAfter](https://github.com/Jekahome/Patterns/blob/main/_img/BridgeAfter.png "BridgeAfter")


[Bridge](http://dron.by/post/pattern-proektirovaniya-bridge-most-na-php.html)

[Bridge sourcemaking.com](https://sourcemaking.com/design_patterns/bridge)

[Bridge refactoring.guru](https://refactoring.guru/ru/design-patterns/bridge)

[Bridge chercher.tech](https://chercher.tech/rust/bridge-design-pattern-rust)

[Bridge www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/bridge-design-pattern-in-rust.html)

- ## Decorator

`Pattern Decorator` (обёртка) - суть работы паттерна заключается в "оборачивании" готового объекта новым "функционалом", 
при этом весь оригинальный интерфейс объекта остается доступным (декоратор переадресует все запросы объекту). 
Смысл заключается в том, чтобы можно было безболезненно комбинировать различные декораторы в произвольном порядке, навешивая их на различные объекты. 

Оба объекта имеют общий интерфейс, поэтому для пользователя нет никакой разницы, с каким объектом работать — чистым или обёрнутым. 
Вы можете использовать несколько разных обёрток одновременно — результат будет иметь объединённое поведение всех обёрток сразу.

**Аналогия**
Уличный аниматор наряжается создавая образ из различных аксессуаров - макияж, одежда, больщие уши, перчатки, обувь, головной убор, кегли, трость, ....
Акссесуары - это все декораторы, комбинируй в любой последовательности

Поскольку этот шаблон решает проблему динамического добавления функций во время выполнения, 
он решает проблему сложного требования к созданию подклассов при расширении функциональности базового класса.

Если есть требования четкой последовательности накидывания поведения то стоит рассмотреть вариации `Pattern Builder` или `Pattern Strategy`

**Эмпирические правила**

- Адаптер предоставляет другой интерфейс для своего объекта. Прокси предоставляет тот же интерфейс. Декоратор предоставляет улучшенный интерфейс.

- Адаптер меняет интерфейс объекта, Декоратор расширяет возможности объекта. Таким образом, Decorator становится более прозрачным для клиента. 
Как следствие, Decorator поддерживает рекурсивную композицию, что невозможно при использовании чистых адаптеров.

- Composite и Decorator имеют схожие структурные диаграммы, отражающие тот факт, что оба используют рекурсивную композицию для организации н
еограниченного количества объектов.

- Декоратор можно рассматривать как вырожденный композит, содержащий только один компонент. 
Однако декоратор добавляет дополнительные обязанности — он не предназначен для агрегации объектов.

- Декоратор предназначен для того, чтобы вы могли добавлять обязанности к объектам без создания подклассов. 
Основное внимание Composite уделяется не украшению, а репрезентации. Эти намерения различны, но дополняют друг друга. 
Следовательно, Composite и Decorator часто используются совместно.

- Composite может использовать цепочку ответственности, чтобы позволить компонентам получать доступ к глобальным свойствам через своего родителя. 
Он также может использовать Decorator для переопределения этих свойств частей композиции.

- Декоратор и Прокси имеют разные цели, но схожую структуру. 
Оба описывают, как обеспечить уровень косвенности к другому объекту, и реализации сохраняют ссылку на объект, которому они перенаправляют запросы.

- Декоратор позволяет изменить внешний вид объекта. Стратегия позволяет вам изменить внутренности.

**Пример**
Построить конструктор фильтров для input полей формы. Помимо множества типов полей, есть еще правила валидации
Так, накидывая различные декораторы на input, мы получаем желаемый результат

**Пример**
Приложение оборачивает класс данных в шифрующую и сжимающую обёртки, которые при чтении выдают оригинальные данные, а при записи — зашифрованные и сжатые.

![Decorator](https://github.com/Jekahome/Patterns/blob/main/_img/Decorator.jpg "Decorator")

[Decorator chercher.tech](https://chercher.tech/rust/decorator-design-pattern-rust)

[Decorator sourcemaking.com](https://sourcemaking.com/design_patterns/decorator)

[Decorator refactoring.guru](https://refactoring.guru/ru/design-patterns/decorator)

[Decorator www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/decorator-design-pattern-in-rust.html)

- ## Facade	

`Pattern Facade` скрывает сложности системы и предоставляет простой интерфейс к сложной системе классов, библиотеке или фреймворку.

Оберните сложную подсистему более простым интерфейсом.
Один класс Facade, представляющий всю подсистему. 

Объект Facade должен быть довольно простым защитником или посредником. Он не должен становиться всезнающим оракулом или объектом «бога».

**Проблема**
Вашему коду приходится работать с большим количеством объектов некой сложной библиотеки или фреймворка. 
Вы должны самостоятельно инициализировать эти объекты, следить за правильным порядком зависимостей и так далее.
В результате бизнес-логика ваших классов тесно переплетается с деталями реализации сторонних классов. 
Такой код довольно сложно понимать и поддерживать.

`Pattern Facade` может иметь урезанный интерфейс, не имеющий 100% функциональности, которой можно достичь, используя сложную подсистему напрямую. 
Но он предоставляет именно те фичи, которые нужны клиенту, и скрывает все остальные.

`Pattern Facade` полезен, если вы используете какую-то сложную библиотеку со множеством подвижных частей, но вам нужна только часть её возможностей.

Аналогия
Когда вы звоните в магазин и делаете заказ по телефону, сотрудник службы поддержки является вашим фасадом ко всем службам и отделам магазина. 
Он предоставляет вам упрощённый интерфейс к системе создания заказа, платёжной системе и отделу доставки.

**Эмпирические правила**

- `Pattern Facade` задаёт новый интерфейс, тогда как `Pattern Adapter` повторно использует старый. 
`Pattern Adapter` оборачивает только один класс, а `Pattern Facade` оборачивает целую подсистему. 
Кроме того, `Pattern Adapter` позволяет двум существующим интерфейсам работать сообща, вместо того, чтобы задать полностью новый.

- Abstract Factory может быть использована вместо `Pattern Facade` для того, чтобы скрыть платформо-зависимые классы.

- Объекты Facade часто являются синглтонами, поскольку требуется только один объект Facade.

![Facade](https://github.com/Jekahome/Patterns/blob/main/_img/Facade.png "Facade")

[Facade refactoring.guru](https://refactoring.guru/ru/design-patterns/facade)

[Facade sourcemaking.com](https://sourcemaking.com/design_patterns/facade)

[Facade www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/facade-design-pattern-in-rust.html)

- ## Composite 	

`Pattern Composite` позволяет объединять объекты в древовидную структуру и работать с ней, как если бы это был отдельный объект.
Замечательной особенностью `Pattern Composite` является возможность рекурсивного запуска методов по всей древовидной структуре и суммирования результатов.
Позволяя одинаково трактовать индивидуальные и составные объекты.

Использование `Pattern Composite` имеет смысл только в том случае, если базовую модель вашего приложения можно представить в виде дерева.
Решает проблему легкого доступа/обхода составных элементов

Пример:

Давайте попробуем понять шаблон Composite на примере файловой системы операционной системы. 
В файловой системе существует два типа объектов: файлы и папки. Бывают случаи, когда с файлами и папками следует обращаться одинаково. 
Вот тут-то и пригодится шаблон Composite.
File и Directory оба trait Component имеют один search метод. 
Для файла он просто просмотрит содержимое файла; для папки он просмотрит все файлы этой папки, чтобы найти это ключевое слово.

![Composite](https://github.com/Jekahome/Patterns/blob/main/_img/Composite.png "Composite")

[Composite refactoring.guru](https://refactoring.guru/design-patterns/composite)

[Composite www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/composite-design-pattern-in-rust.html)

- ## Proxy	

`Pattern Proxy` позволяет подставлять вместо реальных объектов специальные объекты-заменители. 
Эти объекты перехватывают вызовы к оригинальному объекту, позволяя сделать что-то до или после передачи вызова оригиналу.

Представьте, что у вас есть дорогостоящий объект, который потребляет много ресурсов при создании, и вы хотите создавать его только в случае крайней необходимости. Как можно отложить создание экземпляра или контролировать доступ к дорогостоящему объекту?

Применение:

1. Ленивая инициализация (виртуальный прокси). Когда у вас есть тяжёлый объект, грузящий данные из файловой системы или базы данных.
Вместо того, чтобы грузить данные сразу после старта программы, можно сэкономить ресурсы и создать объект тогда, когда он действительно понадобится.

2. Удаленный прокси-сервер предоставляет локального представителя объекта, который находится в другом адресном пространстве. 
Это то, что обеспечивает код-заглушка в RPC и CORBA.

3. Защитный прокси-сервер контролирует доступ к конфиденциальному главному объекту.
Защита доступа (защищающий прокси). Когда в программе есть разные типы пользователей, и вам хочется защищать объект от неавторизованного доступа. Например, если ваши объекты — это важная часть операционной системы, а пользователи — сторонние программы (хорошие или вредоносные).
Прокси может проверять доступ при каждом вызове и передавать выполнение служебному объекту, если доступ разрешён.

4. Умный прокси выполняет дополнительные действия при доступе к объекту. Типичное использование включает в себя:
Подсчет количества ссылок на реальный объект, чтобы его можно было автоматически освободить, когда ссылок больше нет (так называемый умный указатель),
Загрузка постоянного объекта в память при первом обращении к нему,
Проверка блокировки реального объекта перед доступом к нему, чтобы гарантировать, что никакой другой объект не сможет его изменить.

Эмпирические правила

`Pattern Decorator` и `Pattern Proxy` имеют схожие структуры, но разные назначения. Они похожи тем, что оба построены на принципе композиции и делегируют работу другим объектам. 
Паттерны отличаются тем, что `Pattern Proxy` сам управляет жизнью сервисного объекта, а обёртывание Декораторов контролируется клиентом.

[Proxy refactoring.guru](https://refactoring.guru/ru/design-patterns/proxy/rust/example)

[Proxy www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/proxy-design-pattern-in-rust.html)

- ## Flyweight	

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

[Flyweight refactoring.guru](https://refactoring.guru/ru/design-patterns/flyweight)

[Flyweight + картинка](https://github.com/fadeevab/design-patterns-rust/blob/main/structural/flyweight/)

## <ins>Паттерны поведения</ins>

Эта группа паттернов позволяет структурировать подходы к обработке поведения и взаимодействия объектов. Проще говоря, как должны проходить процессы в которых существует несколько вариантов протекания событий.

- ## Command	

`Pattern Command` — это поведенческий паттерн проектирования, который превращает запросы в объекты, 
позволяя передавать их как аргументы при вызове методов, ставить запросы в очередь, логировать их, 
а также поддерживать отмену операций.

Когда использовать паттерн команды:

- Очередь. Когда запросы необходимо обрабатывать в определенные моменты времени и в соответствии с различными триггерными ситуациями.

- Слои. Когда необходимо разделить клиента и поставщика услуг (инкапсуляция получателя, для вызывающего не важно какая команда будет послана)

- Когда возникает необходимость в функции отката для определенных операций

- Когда необходима история запросов

- Когда есть необходимость добавить новые команды

- При необходимости параметризации объектов по действию

Мы хотим, чтобы эти действия или команды выполнялись или вызывались в определенном порядке позже и в другое время. 
Эти команды также могут быть вызваны в результате какого-либо события. 
Например, когда пользователь нажимает кнопку или при получении пакета данных. 
Кроме того, эти команды могут быть отменены. 
Это может оказаться полезным для работы редактора. 
Возможно, нам захочется хранить журналы выполненных команд, чтобы можно было повторно применить изменения позже, если система выйдет из строя.

[Command refactoring.guru](https://refactoring.guru/ru/design-patterns/command)

[Command rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/patterns/behavioural/command.html)

[Command www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/command-design-pattern-in-rust.html)

- ## Command + Composite	

Компоновка команд в блоки

- ## Command + Composite + Chain Of Responsibilities

Компоновка команд в блоки и делегирование другим командам

- ## Command invoker	 

Command и receiver исполнитель назначение команды и выполнение ее исполнителем

- ## Interpreter	

Известен как Little (Small) Language, позволяет создать **свой язык** для гибкости и простоты использования пользователем

Design components:

Abstract Expression - объявляет операцию interpret(), которую переопределяют все узлы (терминальные и нетерминальные) в AST.
    класс абстрактного выражения определяет абстрактный интерфейс для языковых выражений. Он объявляет interpret()
    метод, определяющий логику интерпретации выражений.

Terminal Expression - (NumberExpression): реализует операцию interpret() для терминальных выражений.
    классы терминальных выражений представляют собой элементарные строительные блоки языка. Они реализуют интерфейс
    абстрактных выражений и обеспечивают логику интерпретации терминальных выражений.

Non-Terminal Expression - (AdditionExpression, SubtractionExpression, and MultiplicationExpression):
    реализует операцию interpret() для всех нетерминальных выражений.
    классы нетерминальных выражений представляют собой составные выражения, состоящие из нескольких подвыражений.
    Они также реализуют интерфейс абстрактных выражений и обеспечивают логику интерпретации составных выражений.

Context - содержит информацию, которая является глобальной для интерпретатора.
    класс контекста предоставляет любую необходимую информацию или состояние, необходимое для интерпретации выражений.
    Он сохраняет глобальную информацию, разделяемую между выражениями во время интерпретации.

Client - (ExpressionParser): строит (или предоставляет) AST, собранный из TerminalExpression и NonTerminalExpression.
    Клиент вызывает операцию interpret()

![Interpreter](https://github.com/Jekahome/Patterns/blob/main/_img/Interpreter.png "Interpreter")

[Interpreter sourcemaking.com](https://sourcemaking.com/design_patterns/interpreter)

[Interpreter geeksforgeeks](https://www.geeksforgeeks.org/interpreter-design-pattern/)

[Interpreter medium.com](https://medium.com/@rajeshvelmani/understanding-language-interpretation-with-the-interpreter-design-pattern-in-java-b2a3969eaf9)

[Interpreter www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/interpreter-design-pattern-in-rust.html)

- ## Strategy	

Позволяет эффективно работать коду, благодаря наличию реализованных стратегий основываясь на входных данных.
Все стратегии реализуются по обшему интерфейсу и соответственно код становится гибким/взаимозаменяемым.
Это позволяет отказаться от использования переключателей и/или условных операторов.
К примеру выбор способа сортировки зависит от типа и размера данных, соответственно выбирая подходящий алгоритм мы используем паттерн стратегия.

Если используется правило, которое не подвержено изменениям, нет необходимости обращаться к `Pattern Strategy`.

Одно из преимуществ использования `Pattern Strategy` заключается в том, что мы можем избавиться от ветвления `if/else`.
Достигается это за счет того, что `Client` "знает",
какой алгоритм он хочет использовать и передает объект алгоритма в конструктор класса - `Context`. 
Согласно `Singl responsibility` разделяем классы на подклассы.

Один из принципов SOLID: open/close - предполагает сущность открыта для расширения но закрыта для модификаций. Таким образом при надобности расширить
поведение сущности мы просто добавляем еще одну стратегию, вместо изменения кода сущности.

Мотивы
Программа должна обеспечивать различные варианты алгоритма или поведения
Нужно изменять поведение каждого экземпляра класса
Необходимо изменять поведение объектов на стадии выполнения
Введение интерфейса позволяет классам-клиентам ничего не знать о классах, реализующих этот интерфейс и инкапсулирующих в себе конкретные алгоритмы

Эмпирические правила:

- `Pattern Strategy` похожа на `Pattern Method`, за исключением степени детализации.

- `Pattern State` похож на `Pattern Strategy`, за исключением своего намерения.

- `Pattern State, Strategy, Bridge` (и в некоторой степени `Pattern Adapter`) имеют схожие структуры решений. 
Все они разделяют элементы идиомы «handle/body». Они различаются по назначению – то есть решают разные задачи.

- `Pattern Strategy` имеет две разные реализации, первая похожа на `Pattern State`. 
Разница заключается во времени привязки (`Pattern Strategy` — это шаблон с однократной привязкой, тогда как `Pattern State` более динамичен).
Объекты стратегии часто становятся хорошими легковесами.

- `Pattern Strategy` позволяет вам изменить внутренности объекта. `Pattern Decorator` позволяет менять скин.
И `Pattern Strategy`, и `Pattern Decorator` могут применяться для изменения поведения конкретных классов. 
Достоинство стратегии в том, что интерфейс кастомизации не совпадает с публичным интерфейсом и может быть куда более удобным, а недостаток в том, что для использования стратегии необходимо изначально проектировать класс с возможностью регистрации стратегий.

Состоит:

 - Strategy - абстрактная сущность 

 - ConcreteStrategy - конкретные реализации стратегии 

 - Context - содержит конкретную стратегию
 
 - Client - выбирает какую стратегию применять

[Strategy rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html)

[Strategy sourcemaking.com](https://sourcemaking.com/design_patterns/strategy)

[Strategy refactoring.guru](https://refactoring.guru/ru/design-patterns/strategy)

[Strategy ru.wikipedia.org](https://ru.wikipedia.org/wiki/%D0%A1%D1%82%D1%80%D0%B0%D1%82%D0%B5%D0%B3%D0%B8%D1%8F_(%D1%88%D0%B0%D0%B1%D0%BB%D0%BE%D0%BD_%D0%BF%D1%80%D0%BE%D0%B5%D0%BA%D1%82%D0%B8%D1%80%D0%BE%D0%B2%D0%B0%D0%BD%D0%B8%D1%8F))

[Strategy www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/strategy-design-pattern-in-rust.html)

- ## Visitor	

`Pattern Visitor` позволяет добавлять в программу новые операции, не изменяя разнородные классы объектов, 
над которыми эти операции могут выполняться.
(т.е. все струтуры или перечисления остаюся нетронутыми `...Industrial,Residential,Commercial,Build,LevelBuild`)

`Pattern Visitor` полезен везде, где вы хотите применить алгоритм к разнородным данным. 
Если данные однородны, просто применяем один метод. 
Использование объекта посетителя (а не функционального подхода) позволяет посетителю сохранять состояние и, 
таким образом, передавать информацию между узлами.

Т.е. в идеале мы бы могли имплементироваться от трейта и просто вызывая обший для всех метод решить свою задачу, но мы в силу каких-то
причин так не можем делать. 
Так-же, возможная причина, это неуместная логика в рамках этих структур или потенциальные изменения в новой требуемой логике.

`Pattern Fold` аналогичен `Pattern Visitor`, но создает новую версию посещенной структуры данных.

**Single-serving visitor** (одноразовый посетитель)

Является частным случаем использования `Pattern Visitor`. Если в случае с обычным «посетителем» у нас есть врач которого мы можем отправить к разным больным (и при желании по несколько раз), то в данном паттерне можно привести аналогию, что мы нанимаем врача, отправляем его к одному больному и после обследования сразу увольняем.

**Hierarchical visitor** (иерархический посетитель)

Тот же самый `Pattern Visitor`, однако в данном случае он отправляется к не одному больному, а в целую больницу и обходит там всех больных.

[Visitor refactoring.guru](https://refactoring.guru/ru/design-patterns/visitor)

[Visitor sourcemaking.com](https://sourcemaking.com/design_patterns/visitor)

[Visitor rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)

[Visitor habr.com](https://habr.com/ru/articles/332042/)

[Visitor www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/visitor-design-pattern-in-rust.html)

- ## Observer	

`Pattern Observer` - полезен, когда вас интересует состояние объекта и вы хотите получать уведомления о любых изменениях.
Создаёт механизм подписки, позволяющий одним объектам следить и реагировать на события, происходящие в других объектах.

В `Pattern Observer` объект, который наблюдает за состоянием другого объекта, называется Observer,
а объект, за которым ведется наблюдение, называется Subject.

Определите зависимость «один ко многим» между объектами, чтобы при изменении состояния одного объекта
все его зависимые объекты автоматически уведомлялись и обновлялись.

Когда после изменения состояния одного объекта требуется что-то сделать в других, но вы не знаете наперёд, какие именно объекты должны отреагировать.

**Проблема**

Представьте, что вы имеете два объекта: Покупатель и Магазин. В магазин вот-вот должны завезти новый товар, который интересен покупателю.
Покупатель может каждый день ходить в магазин, чтобы проверить наличие товара. Но при этом он будет злиться, без толку тратя своё драгоценное время.
С другой стороны, магазин может разослать спам каждому своему покупателю. Многих это расстроит, так как товар специфический, и не всем он нужен.
Получается конфликт: либо покупатель тратит время на периодические проверки, либо магазин тратит ресурсы на бесполезные оповещения.

**Проблема**

Большая монолитная конструкция плохо масштабируется, поскольку предъявляются новые требования к графическому отображению или мониторингу.
Вообщем легче отправить сообщение нежели добраться в сложившейся иерархии классов до конкретного обьекта.

`Pattern Observer` предлагает хранить внутри объекта издателя `Observer` список ссылок на объекты подписчиков `Subject`, 
причём издатель не должен вести список подписки самостоятельно. 
Он предоставит методы, с помощью которых подписчики могли бы добавлять или убирать себя из списка.
Когда в издателе будет происходить важное событие, он будет проходиться по списку подписчиков и 
оповещать их об этом, вызывая определённый метод объектов-подписчиков.
Издателю безразлично, какой класс будет иметь тот или иной подписчик, так как все они должны следовать общему интерфейсу и иметь единый метод оповещения.

**Pattern Blackboard** (доска объявлений)

Данный паттерн служит для обеспечения взаимодействия между большим количеством объектов. Он является расширением `Pattern Observer` и позволяет централизованно обслуживать как «наблюдателей», так и «создателей событий». В аналогии подпиской на email уведомления, это будет сам сайт подписки, который обслуживает множество подписчиков и тех, кто для них создает информацию (сообщения).

[Observer chercher.tech](https://chercher.tech/rust/observer-design-pattern-rust)

[Observer refactoring.guru](https://refactoring.guru/ru/design-patterns/observer)

- ## Iterator	

`Pattern Iteratot` - Предоставьте способ последовательного доступа к элементам агрегатного объекта, не раскрывая его базовое представление.

Зачастую этот паттерн используется вместо массива объектов, чтобы не только предоставить доступ к элементам, но и наделить некоторой логикой. 
Это может быть ограничение доступа, сортировка или любая другая операция над множеством объектов.

Идея паттерна Итератор состоит в том, чтобы вынести поведение обхода коллекции из самой коллекции в отдельный класс.

Когда вам нужно иметь несколько вариантов обхода одной и той же структуры данных.

**Аналогия**

Вы планируете полететь в Рим и обойти все достопримечательности за пару дней. 
Но приехав, вы можете долго петлять узкими улочками, пытаясь найти Колизей.
Таким образом, Рим выступает коллекцией достопримечательностей, а ваш мозг, навигатор или гид — итератором по коллекции. 
Вы, как клиентский код, можете выбрать один из итераторов, отталкиваясь от решаемой задачи и доступных ресурсов.

**Проблема**

Также, возможный способ применения, когда вы используете экзотическую коллекцию данных т.е. не тривиально организованные данные.
Но каким способом следует перемещаться по сложной структуре данных? Например, сегодня может быть достаточным обход дерева в глубину,
 но завтра потребуется возможность перемещаться по дереву в ширину.
Добавляя всё новые алгоритмы в код коллекции, вы понемногу размываете её основную задачу, которая заключается в эффективном хранении данных. 
Некоторые алгоритмы могут быть и вовсе слишком «заточены» под определённое приложение и смотреться дико в общем классе коллекции.

**Проблема**

Необходимо «абстрагировать» обход совершенно разных структур данных, чтобы можно было определить алгоритмы, 
способные прозрачно взаимодействовать с каждой из них.

Агрегированный объект, такой как список, должен давать вам возможность доступа к его элементам, не раскрывая его внутреннюю структуру. 
Более того, вы можете захотеть перемещаться по списку разными способами, в зависимости от того, чего вам нужно достичь. 
Но вы, вероятно, не делаете этого. хотите раздуть интерфейс списка операциями для разных обходов, даже если вы можете предвидеть те, 
которые вам потребуются. Вам также может потребоваться иметь более одного ожидающего обхода в одном и том же списке». 
Кроме того, может оказаться полезным предоставление единого интерфейса для обхода многих типов агрегатных объектов (т. е. полиморфной итерации).

Абстракция `Iterator` является фундаментальной для новой технологии, называемой «обобщенным программированием». 
Эта стратегия стремится явно отделить понятие «алгоритм» от понятия «структура данных». 

Эмпирические правила
- Абстрактное синтаксическое дерево `Pattern Interpreter` является составным (поэтому также применимы `Pattern Iteratot` и `Pattern Visitor`).

- `Pattern Memento` часто используется вместе с `Pattern Iteratot`. Итератор может использовать Memento для фиксации состояния итерации. Итератор хранит Memento внутри себя.

В Rust итераторы ленивы, то есть они не действуют, пока вы не вызовете методы, которые используют итератор для его использования.

[Iterator sourcemaking.com](https://sourcemaking.com/design_patterns/iterator)

[Iterator refactoring.guru](https://refactoring.guru/ru/design-patterns/iterator)

[Rust std::iter](https://doc.rust-lang.org/std/iter/index.html)

- ## Mediator	

`Pattern Mediator` (посредник) позволяет уменьшить связанность множества классов между собой (многие ко многим), благодаря перемещению этих связей в один класс-посредник.
Устраняет зависимости между компонентами, позволяя повторно их использовать. Упрощает взаимодействие между компонентами. Централизует управление в одном месте.

Паттерн Посредник заставляет объекты общаться не напрямую друг с другом, а через отдельный объект-посредник, который знает, 
кому нужно перенаправить тот или иной запрос. 
Благодаря этому, компоненты системы будут зависеть только от посредника, а не от десятков других компонентов.

Объектам больше нет нужды вызывать друг друга напрямую. 
Это хорошая альтернатива `Pattern Observer`, если у вас есть “центр интеллекта” вроде контроллера (но не в смысле MVC)
Все компоненты (называемые «Коллеги») объединяются в интерфейс MediatorInterface.
Подписчики или объединенные компоненты делегируют управление медиатору.

Таким образом, посредник скрывает в себе все сложные связи и зависимости между классами отдельных компонентов программы. 
А чем меньше связей имеют классы, тем проще их изменять, расширять и повторно использовать.

**Аналогия**

Пилоты садящихся или улетающих самолётов не общаются напрямую с другими пилотами. 
Вместо этого они связываются с диспетчером, который координирует действия нескольких самолётов одновременно. 
Без диспетчера пилотам приходилось бы все время быть начеку и следить за всеми окружающими самолётами самостоятельно, а это приводило бы к частым катастрофам в небе.
Важно понимать, что диспетчер не нужен во время всего полёта. Он задействован только в зоне аэропорта, когда нужно координировать взаимодействие многих самолётов.

**Применимость**

1. Когда вам сложно менять некоторые классы из-за того, что они имеют множество хаотичных связей с другими классами.
Посредник позволяет поместить все эти связи в один класс, после чего вам будет легче их отрефакторить, сделать более понятными и гибкими.

2. Когда вы не можете повторно использовать класс, поскольку он зависит от уймы других классов.
После применения паттерна компоненты теряют прежние связи с другими компонентами, а всё их общение происходит косвенно, через объект-посредник.

3. Когда вам приходится создавать множество подклассов компонентов, чтобы использовать одни и те же компоненты в разных контекстах.
Если раньше изменение отношений в одном компоненте могли повлечь за собой лавину изменений во всех остальных компонентах, 
то теперь вам достаточно создать подкласс посредника и поменять в нём связи между компонентами.

**Эмпирические правила**

Разница между `Pattern Mediator` и `Pattern Observer` не всегда очевидна. Чаще всего они выступают как конкуренты, но иногда могут работать вместе.

Цель `Pattern Mediator` — убрать обоюдные зависимости между компонентами системы. Вместо этого они становятся зависимыми от самого посредника. 
С другой стороны, цель `Pattern Observer` — обеспечить динамическую одностороннюю связь, в которой одни объекты косвенно зависят от других.

![Mediator](https://github.com/Jekahome/Patterns/blob/main/_img/Mediator.png "Mediator")

![Mediator](https://github.com/Jekahome/Patterns/blob/main/_img/Mediator2.png "Mediator")

[Mediator refactoring.guru](https://refactoring.guru/ru/design-patterns/mediator)

[Mediator github.com/fadeevab](https://github.com/fadeevab/mediator-pattern-rust/blob/main/README.md)

- ## State	

В зависимости от состояния изменяется поведение

`Typestate` делает неправильное использование объекта с состоянием ошибкой времени компиляции.

`Pattern State` невозможно рассматривать в отрыве от концепции машины состояний, также известной как стейт-машина или конечный автомат.
Основная идея в том, что программа может находиться в одном из нескольких состояний, которые всё время сменяют друг друга. 
Набор этих состояний, а также переходов между ними, предопределён и конечен. Находясь в разных состояниях, 
программа может по-разному реагировать на одни и те же события, которые происходят с ней.

`Pattern State` — это решение проблемы того, как заставить поведение зависеть от состояния.

**Аналогия**

Паттерн State позволяет объекту изменять свое поведение при изменении его внутреннего состояния. 
Эту картину можно наблюдать в торговом автомате. 
У торговых автоматов есть состояния, основанные на инвентаре, сумме внесенной валюты, возможности внесения сдачи, выбранном товаре и т. д. 
Когда валюта внесена и сделан выбор, торговый автомат либо доставит продукт без сдачи, либо доставит товар. 
продукт и изменить его, не доставить продукт из-за недостаточности валюты на депозите или не доставить продукт из-за истощения запасов.

**Проблема**

Машину состояний чаще всего реализуют с помощью множества условных операторов, `if` либо `switch`, 
которые проверяют текущее состояние объекта и выполняют соответствующее поведение. 
Основная проблема такой машины состояний проявится в том случае, если в обьект добавить ещё десяток состояний. 
Каждый метод будет состоять из увесистого условного оператора, перебирающего доступные состояния. Такой код крайне сложно поддерживать. 
Малейшее изменение логики переходов заставит вас перепроверять работу всех методов, которые содержат условные операторы машины состояний.
Путаница и нагромождение условий особенно сильно проявляется в старых проектах. 
Набор возможных состояний бывает трудно предопределить заранее, поэтому они всё время добавляются в процессе эволюции программы. 
Из-за этого решение, которое выглядело простым и эффективным в самом начале разработки, может впоследствии стать проекцией большого макаронного монстра.
`Pattern State` предлагает создать отдельные классы для каждого состояния, в котором может пребывать объект, 
а затем вынести туда поведения, соответствующие этим состояниям.

**Эмпирические правила**

Реализация `Pattern State` основана на шаблоне `Pattern Strategy`. Разница между `Pattern State` и `Pattern Strategy` заключается в намерении. 
При использовании `Strategy` выбор алгоритма достаточно стабилен. При использовании `State` изменение состояния объекта «контекст» приводит к 
выбору объектов стратегии из «палитры».

`Flyweight` объясняет, когда и как можно совместно использовать объекты `State`.

[Typestate examples](https://github.com/Jekahome/Patterns/tree/main/src/idioms/1.Type_safety/1.2.Typestates)

[Typestate developerlife.com](https://developerlife.com/2024/05/28/typestate-pattern-rust/)

[Typestate willcrichton.net](https://willcrichton.net/rust-api-type-patterns/typestate.html)

[Typestate cliffle.com](https://cliffle.com/blog/rust-typestate/)

[State sourcemaking.com](https://sourcemaking.com/design_patterns/state)

[State refactoring.guru](https://refactoring.guru/ru/design-patterns/state)

[State doc.rust-lang.ru](https://doc.rust-lang.ru/book/ch17-03-oo-design-patterns.html)

- ## Memento (Хранитель)	

`Pattern Memento` (`Token`) позволяет сохранять и восстанавливать прошлые состояния объектов, не раскрывая подробностей их реализации.
По мере разработки вашего приложения вы можете захотеть сохранить контрольные точки в своем приложении и позже вернуться к этим контрольным точкам.
Предоставить возможность выполнить действие отмены, чтобы восстановить объект в предыдущее состояние.

**Аналония**

Отмена действия, возврат назад или сочетание клавиш Ctrl+Z — одна из наиболее часто используемых операций в редакторе. 
Для реализации операции отмены используется `Pattern Memento`. Это делается путем сохранения текущего состояния объекта по мере его изменения.

Одним из важных моментов, которые следует избегать при реализации `Pattern Memento`, является то, что инкапсуляция объекта не должна подвергаться риску.
Паттерн Снимок поручает создание копии состояния объекта самому объекту.
Паттерн предлагает держать копию состояния в специальном объекте-снимке с ограниченным интерфейсом, позволяющим, например, узнать дату изготовления или название снимка. 
Но, с другой стороны, снимок должен быть открыт для своего создателя, позволяя прочесть и восстановить его внутреннее состояние.
Такая схема позволяет создателям производить снимки и отдавать их для хранения другим объектам, называемым опекунами. 
Опекунам будет доступен только ограниченный интерфейс снимка, поэтому они никак не смогут повлиять на «внутренности» самого снимка. 
В нужный момент опекун может попросить создателя восстановить своё состояние, передав ему соответствующий снимок.

В некоторых языках (например, PHP, Python, JavaScript) сложно гарантировать, чтобы только исходный объект имел доступ к состоянию снимка.

**Эмпирические правила**

`Pattern Command` и `Pattern Memento` можно использовать сообща для реализации отмены операций. 
В этом случае объекты команд будут отвечать за выполнение действия над объектом, а снимки будут хранить резервную копию состояния этого объекта, 
сделанную перед самым запуском команды.

`Pattern Memento` иногда можно заменить Прототипом, если объект, состояние которого требуется сохранять в истории, довольно простой, 
не имеет активных ссылок на внешние ресурсы либо их можно легко восстановить.

**Участники:**

1. Originator (Создатель) может производить снимки своего состояния, а также воспроизводить прошлое состояние, если подать в него готовый снимок.

2. Memento (Снимок) — это простой объект данных, содержащий состояние создателя. Надёжнее всего сделать объекты снимков неизменяемыми, передавая в них состояние только через конструктор.

3. Caretaker (Смотритель/Опекун/Хранитель) должен знать, когда делать снимок создателя и когда его нужно восстанавливать.
  Опекун может хранить историю прошлых состояний создателя в виде стека из снимков. 
  Когда понадобится отменить выполненную операцию, он возьмёт «верхний» снимок из стека и передаст его создателю для восстановления.
 

![Memento](https://github.com/Jekahome/Patterns/blob/main/_img/Memento.png "Memento")

[Memento refactoring.guru](https://refactoring.guru/ru/design-patterns/memento)

[Memento chercher.tech](https://chercher.tech/rust/momento-design-pattern-rust)

- ## Chain Of Responsibilities 

`Pattern Chain Of Responsibilities` используется для достижения слабой связи отправителя запроса с получателем. Делегировать задачу следующему. 

Представьте себе систему, в которой запрос может обрабатываться несколькими типами обработчиков, но действовать над ним должен только один на основе определенных критериев. Вместо того, чтобы жестко запрограммировать логику принятия решений, как мы можем эффективно делегировать запрос через ряд обработчиков?

Позволяет передавать запросы последовательно по цепочке обработчиков. 
Каждый последующий обработчик решает, может ли он обработать запрос сам и стоит ли передавать запрос дальше по цепи.
Механизм цепочки использует рекурсивную композицию, позволяющую связывать неограниченное количество обработчиков.

`Pattern Chain Of Responsibilities` позволяет избежать связывания отправителя запроса с получателем, предоставляя более чем одному объекту возможность обработать запрос.

По сути это набор обработчиков, которые по очереди получают запрос, а затем решают обрабатывать его или нет. 
Если запрос не обработан, то он передается дальше по цепочке. 
Если же он обработан, то паттерн сам решает передавать его дальше или нет.
middleware

Необходимость эффективной обработки запросов без жесткого связывания отношений и приоритета обработчиков или сопоставлений запросов с обработчиками.

Паттерн предлагает связать объекты обработчиков в одну цепь. 
Каждый из них будет иметь ссылку на следующий обработчик в цепи. 
Таким образом, при получении запроса обработчик сможет не только сам что-то с ним сделать, но и передать обработку следующему объекту в цепочке.

Не используйте `Pattern Chain Of Responsibilities`, если каждый запрос обрабатывается только одним обработчиком или когда клиентский объект знает, 
какой объект службы должен обрабатывать запрос.

**Применимость**

1. Когда программа должна обрабатывать разнообразные запросы несколькими способами, но заранее неизвестно, какие конкретно запросы будут приходить и какие обработчики для них понадобятся.
   С помощью `Pattern Chain Of Responsibilities` вы можете связать потенциальных обработчиков в одну цепь и при получении запроса поочерёдно спрашивать каждого из них, не хочет ли он обработать запрос.

2. Когда важно, чтобы обработчики выполнялись один за другим в строгом порядке.
   `Pattern Chain Of Responsibilities` позволяет запускать обработчиков последовательно один за другим в том порядке, в котором они находятся в цепочке.

3. Когда набор объектов, способных обработать запрос, должен задаваться динамически.
   В любой момент вы можете вмешаться в существующую цепочку и переназначить связи так, чтобы убрать или добавить новое звено.

4. Объектно-ориентированный связанный список с рекурсивным обходом.

**Реальные варианты использования**

1. Обработка событий в библиотеках графического интерфейса, где события могут обрабатываться несколькими компонентами (например, кнопкой, диалогом, окном).

2. Промежуточное программное обеспечение на веб-серверах, где запрос может обрабатываться несколькими функциями промежуточного программного обеспечения, прежде чем он достигнет конечного обработчика.

3. Проверка входных данных в конвейерах обработки.

**Аналогия**

Банкоматы используют цепочку ответственности в механизме выдачи денег.
Для сдачи подойдут все купюры из которых возможно составить сумму и они есть в наличии.

Эмпирические правила

`Pattern Chain Of Responsibilities` часто используют вместе с Компоновщиком. В этом случае запрос передаётся от дочерних компонентов к их родителям.

Обработчики в `Pattern Chain Of Responsibilities` могут быть выполнены в виде `Pattern Command`. В этом случае множество разных операций может быть выполнено над одним и тем же контекстом, коим является запрос.
Но есть и другой подход, в котором сам запрос является  `Pattern Command`, посланной по цепочке объектов. В этом случае одна и та же операция может быть выполнена над множеством разных контекстов, представленных в виде цепочки.

`Pattern Chain Of Responsibilities` и `Pattern Decorator` имеют очень похожие структуры. 
Оба паттерна базируются на принципе рекурсивного выполнения операции через серию связанных объектов. Но `Pattern Decorator` не прерывает ход выполнения.

![Сhain-of-responsibility](https://github.com/Jekahome/Patterns/blob/main/_img/chain-of-responsibility.png "Сhain-of-responsibility")

[Chain Of Responsibilities refactoring.guru](https://refactoring.guru/ru/design-patterns/chain-of-responsibility/rust/example)

[Chain Of Responsibilities chercher.tech](https://chercher.tech/rust/chain-of-responsibility-design-pattern-rust)

- ## Template method	

`Template method` - определяет скелет алгоритма, перекладывая ответственность за некоторые его шаги на подклассы. 
Паттерн позволяет подклассам переопределять шаги алгоритма, не меняя его общей структуры.
Идея состоит в том, чтобы позволить наследникам абстрактного
шаблона переопределить поведение алгоритмов родителя.
  
Это простой способ изолировать логику в конкретные классы и уменьшить копипаст,
поэтому вы повсеместно встретите его в том или ином виде.

`Template method` предлагает разбить алгоритм на последовательность шагов, описать эти шаги в отдельных методах и 
вызывать их в одном шаблонном методе друг за другом.

Это позволит подклассам переопределять некоторые шаги алгоритма, оставляя без изменений его структуру и 
остальные шаги, которые для этого подкласса не так важны.

[Template method refactoring.guru](https://refactoring.guru/ru/design-patterns/template-method)

[Template method www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2023/10/template-method-design-pattern-in-rust.html)

# Other Design Patterns
 
- ## Object Pool Pattern	

`Object Pool Pattern`  предоставляет метод повторного использования инициализированных объектов вместо создания новых.
Из соображений эффективности может быть весьма удобно держать наготове пул (т.е. непустой набор) инициализированных объектов. 
Это может произойти, например, когда у вас есть соединения с базой данных, создание которых требует больших затрат времени и ресурсов.
Это позволяет вызвать определенный объект из пула для использования в течение определенного периода времени, 
а затем вернуть его обратно в пул после выполнения задания. 
Во время отсутствия этого объекта никакие другие компоненты не могут использовать его, пока он не будет возвращен обратно в пул.

Есть несколько crate, таких как [lockfree-object-pool, object-pool](https://crates.io/crates/lockfree-object-pool) и особенно [opool](https://crates.io/crates/opool), которые также реализуют и расширяют эту функциональность.

[Object Pool www.hackingwithrust.net](https://www.hackingwithrust.net/2023/10/15/an-object-pool-in-rust-two-implementations/)

- ## Private Class Data	

`Pattern Private Class Data` - инкапсуляция аттрибутов класса.

**Намерение**

1. Управление доступом на запись к атрибутам класса
2. Отделяйте данные от методов, которые их используют.
3. Инкапсулировать инициализацию данных класса.

Мотивация для этого шаблона проектирования исходит из цели проектирования — защитить состояние класса путем минимизации видимости его атрибутов (данных).

Шаблон проектирования данных частного класса направлен на уменьшение раскрытия атрибутов за счет ограничения их видимости.

Это уменьшает количество атрибутов класса, инкапсулируя их в один объект данных. 
Это позволяет разработчику класса лишить права записи атрибутов, которые предназначены для установки только во время создания, даже из методов целевого класса.

P.S.
В Rust'е есть возможность "запечатать" [Sealing](https://github.com/Jekahome/Patterns/tree/main/idioms/6.Sealing) реализацию.

[Private Class Data sourcemaking.com](https://sourcemaking.com/design_patterns/private_class_data)

- ## Specification
 
`Pattern Specification` предлагает решение, позволяющее создавать многократно используемые бизнес-правила, которые можно комбинировать с использованием логики.

[Specification www.hackingwithrust.net](https://www.hackingwithrust.net/2023/11/12/simplified-precision-unraveling-the-simple-specification-pattern-in-rust-for-expressive-code-design/)

- ## Delegation	 

```rust unimplemented! ```

Объект, вместо того чтобы выполнять одну из своих поставленных задач, поручает её связанному вспомогательному объекту.

[Delegation](https://snoekiede.medium.com/easy-delegation-in-rust-the-delegation-pattern-hacking-with-rust-9366f10bf7f2?source=user_profile---------19----------------------------)

- ## Service Locator	

```rust unimplemented! ```

Для реализации слабосвязанной архитектуры, чтобы получить хорошо тестируемый, сопровождаемый и расширяемый код. 
Паттерн Dependency injection (DI) и Service Locator — это реализация паттерна Inversion of Control (IoC)
Анти-паттерн так как нарушает принцип SOLID инверсии зависимости т.е. он избавляет другие классы от этого но сам зависим от конкретных классов.
В случае изменения данных зависимостей мы рискуем сломать функционал классов, которые их используют, вследствие чего затрудняется поддержка системы.

- ## Dependency injection (DI)

Для реализации слабосвязанной архитектуры, относится к категории инверсии управления (Inversion of Control, IoC).
Основная цель DI — отделение создания зависимостей от их использования, что делает код более гибким, тестируемым и поддерживаемым. 

Используя DI, можно легко заменять реальные зависимости на заглушки (stubs) или моки (mocks) при тестировании. Это позволяет изолировать код от внешних зависимостей, таких как базы данных, веб-сервисы или другие ресурсы, что упрощает процесс тестирования и делает тесты более надежными и быстрыми.

DI способствует более гибкой архитектуре приложения. Если зависимости передаются через конструктор или методы, это позволяет легко менять их реализацию на другую, без необходимости изменения кода, который использует эти зависимости.

DI помогает снизить уровень связности между компонентами системы. Вместо жесткого связывания объектов с конкретными реализациями, объекты получают зависимости извне, что способствует более слабой связи (loose coupling) и более модульному и повторно используемому коду.

Для выноса из класса неявных зависимостей тремя способами(путем явной зависимости через конструктор, метод, интерфейс) для явной зависимости что бы была возможность подменить обьекты при тестировании.
Объект отдаёт заботу о построении требуемых ему зависимостей внешнему, специально предназначенному для этого общему механизму
Чтобы получить более тестируемый, сопровождаемый и расширяемый код.
Реализуется через передачу обьекта в конструктор, метод, интерфейс

Внедрение зависимости позволяет переложить часть ответственности за какой-то функционал на другие объекты. 
Например если нам требуется нанять новый персонал, то мы можем не создавать свой отдел кадров, а внедрить зависимость от компании по подбору персонала, которая свою очередь по первому нашему требованию «нам нужен человек», будет либо сама работать как отдел кадров, либо же найдет другую компанию (при помощи «локатора служб»), которая предоставит данные услуги.
«Внедрение зависимости» позволяет перекладывать и взаимозаменять отдельные части компании без потери общей функциональности.

# `PoSA` Архитектура программного обеспечения, ориентированная на шаблоны

[PoSA](https://en.wikipedia.org/wiki/Pattern-Oriented_Software_Architecture)

[PoSA github.com/ppizarro](https://github.com/ppizarro/coursera/blob/master/POSA/Books/Pattern-Oriented%20Software%20Architecture/Pattern-Oriented%20Software%20Architecture%20~%20Vol%203%20~%20Patterns%20for%20Resource%20Management%20(Wiley~2004.06).chm)

```rust unimplemented! ```

...

# Database Patterns

Шаблоны баз данных, сохранять и извлекать данные из баз данных и устанавливать соответствие между объектами базы данных и приложения.

- ## ActiveRecord	

```rust unimplemented! ```

Схема Active Record — это подход к доступу к данным в базе данных. Таблица базы данных или представление обёрнуты в классы. 
Таким образом, объектный экземпляр привязан к единственной строке в таблице. После создания объекта новая строка будет добавляться к таблице на сохранение. 
Любой загруженный объект получает свою информацию от базы данных. Когда объект обновлён, соответствующая строка в таблице также будет обновлена. 
Класс обёртки реализует методы средства доступа или свойства для каждого столбца в таблице или представлении.
Нарушает принцип единственной ответственности (SRP) или нет ?

- ## Identity Map	

(Identity Map => Data Mapper => Repository => Unit of Work)

```rust unimplemented! ```

Обеспечивает однократную загрузку объекта, сохраняя данные об объекте в карте соответствия. 
При обращении к объектам, ищет их в карте соответсвия.
Паттерн `Identity Map` (Карта присутствия / Карта соответствия) хранит записи о всех объектах, которые были считаны из БД за время выполнения одного действия. 
Когда происходит обращение к объекту, проверяется карта соответствия (присутствия), чтобы узнать, загружен ли объект.
В простом случае с изоморфной схемой у вас будет одна карта для каждой таблицы базы данных. 
Когда вы загружаете объект из базы данных, вы сначала проверяете карту. Если в нем есть объект, соответствующий тому, который вы загружаете, вы его возвращаете. Если нет, вы обращаетесь к базе данных и помещаете объекты на карту для дальнейшего использования по мере их загрузки.

Намерение

Гарантирует, что каждый объект загружается только один раз, сохраняя каждый загруженный объект на карте. Ищет объекты, используя карту при обращении к ним.
Карта идентичности хранит запись обо всех объектах, которые были прочитаны из базы данных в ходе одной бизнес-транзакции. Всякий раз, когда вам нужен объект, вы сначала проверяете Карту идентичности, чтобы узнать, есть ли он у вас уже.

Identity Map — это, по сути, способ кэширования данных из какого-то медленного хранилища, например, диска или базы данных. Вместо того, чтобы каждый раз получать новое значение, вам дается ссылка на одно и то же значение где-то в кеше.

[Identity Map martinfowler.com](https://martinfowler.com/eaaCatalog/identityMap.html)

[Identity Map  www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2018/04/identity-map-pattern.html)

- ## Data Mapper

(Identity Map => Data Mapper => Repository => Unit of Work)

```rust unimplemented! ```

`Data Mapper` Преобразователь Данных — это паттерн, который выступает в роли посредника для двунаправленной передачи данных между постоянным хранилищем данных (часто, реляционной базы данных) и представления данных в памяти (слой домена, то что уже загружено и используется для логической обработки). 
Цель паттерна в том, чтобы держать представление данных в памяти и постоянное хранилище данных независимыми друг от друга и от самого преобразователя данных. 
Слой состоит из одного или более mapper-а (или объектов доступа к данным), отвечающих за передачу данных. 
Реализации mapper-ов различаются по назначению. 
Общие mapper-ы могут обрабатывать всевозоможные типы сущностей доменов, а выделенные mapper-ы будет обрабатывать один или несколько конкретных типов.

Предоставляет вам объекты, которые выглядят точно так же, как записи в вашей структуре записей, но доступны с помощью обычных механизмов вашего языка программирования. Все детали доступа к источникам данных скрыты за этим интерфейсом.

В отличии от ActiveRecord где полностью отображается структура реаляционной базы на свойства классов.
В связке с паттреном Repository который представляет слой над DataMaper получаем независимую друг от друга систему общения бизнес-логики с данными.

Персистентность (т.е. минимизация подверженности изменениям ) архитектурного слоя поддерживает Repository за счет того что он берет на себя все изменения/новые требования бизнес-правил и в итоге обьекты DataMapers остаются неизменны.

[Data Mapper](https://martinfowler.com/eaaCatalog/dataMapper.html)

[Data Mapper designpatternsphp.readthedocs.io](https://designpatternsphp.readthedocs.io/ru/latest/Structural/DataMapper/README.html)

[Data Mapper www.sitepoint.com](https://www.sitepoint.com/integrating-the-data-mappers/)

[Row Data Gateway](http://design-pattern.ru/patterns/row-data-gateway.html)

[Table Data Gateway](http://design-pattern.ru/patterns/table-data-gateway.html)

- ## Repository

(Identity Map => Data Mapper => Repository => Unit of Work)

`Pattern Repository` - это абстракция бизнес модели над данными из которых она состоит. Реализация репозитория инкапсулирует доступ к данным.
Бизнес логика работает с абстракным репозиторием, поэтому бизнес логика остается персистентной 
т.е. постоянной и не подвергается изменениям если источник данных будет изменятся.

Единственный репозиторий не нуждается в `Unit Of Work` так как нет бизнес транзакции нуждающейся в контроле сохраняемого состояния не связанных сущностей!
Когда в системе появляются связи сущностей (репозитории), тогда есть запрос на поддержания консистентности их состояния в хранилище данных.

`Pattern Repository` дает унифицированный набор общих операций над сущностью в БД, это то, что на самом деле является моделью - набор методов, реализующих бизнес-логику приложения.
Внутри `Repository` может быть использован `Data Mapper`. Назначение `Data Mapper` именно в маппинге сущности из БД на доменную сущность (или модель).

`Pattern Repository` - это классы или компоненты, которые инкапсулируют логику, необходимую для доступа к источникам данных. 
Они централизуют общие функции доступа к данным, обеспечивая лучшую ремонтопригодность и отделяя инфраструктуру или технологии, используемые для доступа к базам данных, от уровня модели предметной области.

`Pattern Repository` — это абстракция постоянного хранилища. Он скрывает скучные детали доступа к данным, делая вид, что все наши данные находятся в памяти.

По сути, он обеспечивает абстракцию данных, так что ваше приложение может работать с простой абстракцией, интерфейс которой приближается к коллекции. Добавление, удаление, обновление и выбор элементов из этой коллекции осуществляется с помощью ряда простых методов без необходимости решать проблемы с базой данных, такие как соединения, команды, курсоры или устройства чтения. Использование этого шаблона может помочь добиться слабой связи и не учитывать постоянство объектов домена. 

"оградить ваше приложение от изменений в хранилище данных и облегчить автоматизированное модульное тестирование"

Благодаря абстракции от реального хранилища данных, мы можем тестировать приложение (просто подставляете другую реализации репозитория но интерфейс прежний) не опасаясь повреденить реальные данные.
Также если приложение работает с различными источниками данные то репозиторий будет выступать их HUB'ом централизованно управлять. Вы сможете реализовать и централизовать стратегию кэширования (...,`Identity Map`) для источника данных.
Вы сможете улучшить удобство сопровождения и читаемость кода, отделив бизнес-логику от логики доступа к данным или службам.
Также за счет использования строго типизированных бизнес-сущностей (`Data Mapper`), а не примитивов, вы сможете выявлять проблемы во время компиляции, а не во время выполнения.
 
Repository - коллекция хранения объектов инкапсулирует слой определения от слоя использования. Репозиторий позволяет абстрагироваться от конкретных подключений к источникам данных, с которыми работает программа, и является промежуточным звеном между классами, непосредственно взаимодействующими с данными, и остальной программой.

Персистентность (не подверженность изменению) архитектурного слоя Repository поддерживает за счет того, что он берет на себя все изменения/новые требования бизнес-правил и в итоге, обьекты `DataMapers` остаются неизменны.

Есть простые обьекты предметной области/бизнес-логики
Есть mappers обьекты они занимаются сопоставлением/картограф/отображением обьектов предметной области на базу данных и обратно поиск в базе и восзосдание обьекта предметной области, таким образом обьекты предметной области не зависят/игнорируют от базы данных.
Но использовать mappers обьекта в слое бизнес-логики это загрязнение логики издежками инфраструктуры.
А что если бизнес-правила станут еще более извилистей и потребуют более детализированных запросов характерных для бизне-правил?
Тогда придется расширять обьект mappers т.е. впихнуть бизнес правила в обьект отображения базы данных!
Репозиторий является еще одним слоем абстракции над mappers обьектом и берет на себя все изменения/условия бизнес-правил для реализации их требоаний,
являясь единой точкой входа для логики приложения к данным.
Репозиторий эффективно обменивает бизнес-терминологию с клиентским кодом (так называемый универсальный язык, придуманный Эриком Эвансом в его книге «Дизайн, управляемый доменом» )

`Pattern Repository` — это паттерн проектирования, который предоставляет абстрактный интерфейс для работы с коллекцией объектов. 
Он предоставляет механизм для сохранения, извлечения и поиска объектов внутри хранилища (например, базы данных), а также инкапсулирует логику доступа к данным.

###### Основные элементы паттерна Repository:

**Интерфейс репозитория (Repository Interface):** Определяет операции, которые могут быть выполнены над объектами в хранилище. Обычно это включает в себя методы для создания, чтения, обновления и удаления объектов (CRUD).

**Конкретная реализация репозитория (Concrete Repository):** Предоставляет конкретную реализацию интерфейса репозитория и реализует методы для работы с данными в конкретном хранилище (например, в базе данных).

**Объекты-сущности (Entity Objects):** Представляют объекты, которые сохраняются и извлекаются из хранилища. Эти объекты могут представлять бизнес-сущности или данные.

**Контекст хранилища (Storage Context):** Определяет, где и как будут храниться объекты. Это может быть база данных, файловая система или другой источник данных.

Преимущества использования паттерна Repository включают:

**Изоляция кода доступа к данным:** Репозиторий абстрагирует код доступа к данным, что позволяет легко изменять и заменять источники данных без изменения остальной части кода.

**Централизованное управление доступом к данным:** Все операции с данными выполняются через единый интерфейс репозитория, что упрощает управление и поддержку кода.

**Улучшение тестируемости:** Изоляция доступа к данным делает код более тестируемым, так как можно легко создавать заглушки (mocks) или имитации для репозиториев в тестах.

[Repository habr.com](https://habr.com/ru/post/248505/)

[Repository metanit.com](https://metanit.com/sharp/articles/mvc/11.php)

[Repository learn.microsoft.com](https://learn.microsoft.com/en-us/previous-versions/msp-n-p/ff649690(v=pandp.10)?redirectedfrom=MSDN)

- ## Unit of Work (UOW)

(Identity Map => Data Mapper => Repository => Unit of Work)

Этот шаблон принадлежит каталогу объектно-реляционных поведенческих шаблонов, а этот каталог принадлежит шаблонам архитектуры корпоративных приложений.

`Unit of Work` может выполняет сразу две важные задачи:  

1. Уменьшить количество запросов по сети к хранилищу данных, за счет хранение состояния обьектов в памяти. 
   После завершения всех операций обновления, отправляет или нет текущее состояние обьекта в используемое хранилище (MYSQL, POSTGRES, ...)

2. Обеспечивает целосность бизнес транзакции, за счет обьединения всех операций над обьектами участниками в одну общую транзакцию.
   Транзакция либо выполнится для всех, либо произойдет откат в стабильное состояние (по аналогии с ACID - Атомарность и Согласованность).
   Механизм отката обеспечивается для баз данных поддерживающих транзакции.
   Для баз данных не поддержиющей транзакцию, придется хранить и сбрасывать состояние обьекта находящегося в памяти. 

Реализация `Unit of Work` и `Pattern Repository` может помочь оградить ваше приложение от изменений в хранилище данных и облегчить автоматизированное модульное тестирование или разработку через тестирование (TDD).

**Намерение**

Гарантирует, что единица транзакции, охватывающая несколько связанных репозиториев, либо завершится для всех объектов, либо полностью завершится неудачно, обеспечив согласованность базы данных.

Когда бизнес-транзакция завершается, все эти обновления отправляются как одна большая единица работы, которая сохраняется в базе данных за один раз, чтобы свести к минимуму количество обращений к базе данных.

`Unit of Work` использует одну транзакцию или одну единицу работы для нескольких операций вставки, обновления и удаления. Эти операции либо завершаются успешно, либо проваливаются как единое целое. Другими словами, все операции будут зафиксированы как одна транзакция или отменены как единое целое.

**Объяснение**

Шаблон проектирования «Единица работы» выполняет две важные задачи: во-первых, он поддерживает обновления в памяти, а во-вторых, отправляет эти обновления в памяти как одну транзакцию в базу данных.
Итак, для достижения вышеуказанных целей необходимо пройти два этапа:
- Он хранит в памяти списки бизнес-объектов, которые были изменены (вставлены, обновлены или удалены) во время транзакции.
- После завершения транзакции все эти обновления отправляются как одна большая единица работы, которая физически сохраняется в базе данных за один раз.

Ключевой момент в `Unit of Work` заключается в том, что, когда приходит время принятия обязательств, `Unit of Work` решает, что делать. Он открывает транзакцию, выполняет проверку параллелизма и записывает изменения в базу данных. Программисты приложений никогда явно не вызывают методы для обновления базы данных. Таким образом, им не придется отслеживать, что изменилось, или беспокоиться о том, как ссылочная целостность влияет на порядок, в котором им нужно что-то делать.

`Unit of Work` простое хранилище объектов в памяти, которое отслеживает, какие объекты домена должны быть запланированы для вставки, обновления и удаления

Работает вместе с `Repository`
Множество обращений к базе можно уменьшить умной стратегией кеширования.
Обеспечивает целостность данных (атомарную синхронизацию изменений т.е. один момент времени обьект сохраняет единственный метод), выполняется в транзакции.
Для этого нужна реализация `Identity Map` обеспичавающая одну ссылку на обьект по всей системе, тогда все изменения будут только для одного обьекта.
Задача `Identity Map` - сохранение карты созданных объектов, взятых из хранилища с тем чтобы гарантировать что одна единица информации из хранилища представлена ровно одним экземпляром объекта данных в приложении. Это позволяет избежать конфликтов изменений т.к. не допускает ситуации когда два объекта, представляющих один и тот же элемент данных в хранилище, изменены по-разному. Информация из `Identity Map` используется в методе commit() паттерна `Unit of Work` для вычисления разницы между исходными данными и накопленными изменениями.

Пример: в Doctrine, сущность после изменения не сразу обновляется в базе данных, она отслеживается и если вернется на прежнее состояние до вызова метода `flush`
 т.е. явного сохранения в базу, то в базу запроса не будет, а если она изменила свое состояние, то пойдет запрос базу.

Поскольку для вычисления разницы (и, соответственно, определения того что и каким образом должно быть изменено в хранилище) необходимо знать какие данные и как именно хранятся в объектах - как правило необходима также реализация паттерна `Metadata Mapping`, описывающего связь между содержимым хранилища (к примеру таблицами и столбцами базы данных) и классами / свойствами объектов.

Обслуживает набор объектов, изменяемых в бизнес-транзакции (бизнес-действии) и управляет записью изменений и разрешением проблем конкуренции данных.

Когда необходимо писать и читать из БД, важно следить за тем, что вы изменили и если не изменили - не записывать данные в БД. Также необходимо вставлять данные о новых объектах и удалять данные о старых.

**Проблема:**

Можно записывать в БД каждое изменение объекта, но это приведёт к большому количеству мелких запросов к БД, что закончится замедлением работы приложения. Более того, это требует держать открытую транзакцию всё время работы приложения, что непрактично, если приложение обрабатывает несколько запросов одновременно. Ситуация ещё хуже, если необходимо следить за чтением из и БД, чтобы избежать неконсистентного чтения.

Реализация паттерна `Unit of Work` следит за всеми действиями приложения, которые могут изменить БД в рамках одного бизнес-действия. Когда бизнес-действие завершается, `Unit of Work` выявляет все изменения и вносит их в БД.

Также, если данные в хранилище не являются независимыми (к примеру связи между таблицами в базе данных) - может потребоваться реализации ряда паттернов, отвечающих за сохранение информации о связях между данными (это паттерны раздела `Object-Relational Structural Patterns`:  Identity Field, Foreign Key Mapping, Association Table Mapping, Dependent Mapping, Embedded Value, Serialized LOB, Single Table Inheritance, Class Table Inheritance, 
Concrete Table Inheritance, Inheritance Mappers).

Используйте шаблон `Unit of Work`, когда:
 - Оптимизировать время, затрачиваемое на транзакции базы данных.
 - Отправка изменений в базу данных как единица работы, обеспечивающая атомарность транзакции.
 - Чтобы уменьшить количество обращений к базе данных.

[Unit of Work www.sitepoint.com](https://www.sitepoint.com/implementing-a-unit-of-work/)

[Unit of Work gist.github.com/voronkovich](https://gist.github.com/voronkovich/d35cdcdf6eb09e986ab9b16f91a5b2e8)

[Unit of Work design-pattern.ru](http://design-pattern.ru/patterns/unit-of-work.html)

[Unit of Work www.sourcecodeexamples.net](https://www.sourcecodeexamples.net/2018/04/unit-of-work.html)

[Unit of Work learn.microsoft.com](https://learn.microsoft.com/en-us/previous-versions/msp-n-p/ff649690(v=pandp.10)?redirectedfrom=MSDN)

- ## Lazy Load 

```rust unimplemented! ```

Загрузка данных по мере необходимости. Объект, не содержит данных, но знает, где их взять.
При первом обращении или при свободном ресурсе, происходит загрузка, последующие обращения используют тот же обьект не загружая обьект из источника.

**Существует четыре основных варианта ленивой загрузки.**

1. Lazy Initialization (Ленивая Инициализация) использует специальный макер (обычно null), чтобы пометить поле, как не загруженное. При каждом обращении к полю проверяется значение маркера и, если значение поля не загружено - оно загружается.

2. Virtual Proxy (Виртуальный Прокси) - объект с таким же интерфейсом, как и настоящий объект. При первом обращении к методу объекта, виртуальный прокси загружает настоящий объект и перенаправляет выполнение.

3. Value Holder (Контейнер значения) - объект с методом getValue. Клиент вызывает метод getValue, чтобы получить реальный объект. getValue вызывает загрузку.

4. Ghost (Призрак) - объект без каких-либо данных. При первом обращении к его методу, призрак загружает все данные сразу.

P.S. В Rust'е итераторы ленивы, также `std::borrow::Cow` обладает свойствами бережного обращения к русурсам, и smart pointer `Rc/Arc`

[Lazy Load](http://design-pattern.ru/patterns/lazy-load.html)

## [Паттерны Объектно-Реляционного структурирования](http://design-pattern.ru/patterns/)

```rust unimplemented! ```

    Identity Field (Поле первичного ключа)
    Foreign Key Mapping (Разметка внешних ключей)
    Association Table Mapping (Разметка таблиц связей)
    Dependent Mapping (Управление распределением подчинённых сущностей)
    Embedded Value (Объединённое свойство)
    Serialized LOB (Сериализованный LOB)
    Single Table Inheritance (Наследование с единой таблицей)
    Class Table Inheritance (Наследование с таблицами классов)
    Concrete Table Inheritance (Наследование с таблицами конечных классов)
    Inherritance Mappers (Наследуемые распределители)


# [Architecture](https://martinfowler.com/architecture/)

... Eric Evans’s Domain-Driven Design or Martin Fowler’s Patterns of Enterprise Application Architecture

## Architectural Styles

### Messaging

**Puplish-Subscribe**

```rust unimplemented!```

**Event-Driven**

```rust unimplemented!```

### Distributed

**Client-Server**

```rust unimplemented!```

**Peer-to-Peer**

```rust unimplemented!```

Архитектура "peer-to-peer" (P2P) представляет собой распределенную архитектурную модель, в которой каждый участник сети (пир) может одновременно выполнять роль как клиента, так и сервера. В такой сети нет центрального управляющего узла или сервера. Вместо этого каждый узел взаимодействует непосредственно с другими узлами, обмениваясь ресурсами, информацией или услугами.

Основные характеристики архитектуры P2P включают:

Децентрализация: Отсутствие единого центрального сервера, который координирует все действия в сети. Все узлы равноправны и могут взаимодействовать между собой напрямую.

Равноправность (Peerness): Все участники сети называются "пирами". Каждый пир имеет равные права и обязанности.

Самоорганизация: Узлы сети самостоятельно принимают решения и координируют свою работу с другими узлами, обеспечивая эффективность и стабильность сети.

Распределенная база данных: Данные часто распределены между разными узлами сети, и каждый узел может быть как потребителем, так и поставщиком данных.

Открытость: Сеть может быть открытой, что означает, что новые узлы могут присоединяться или покидать сеть в любое время без существенных изменений в архитектуре.

Примеры использования архитектуры P2P включают сети для обмена файлами (например, BitTorrent), блокчейн-технологии (например, Bitcoin), VoIP-телефонии (например, Skype) и децентрализованных приложений (DApps).

Архитектура P2P обеспечивает устойчивость к отказам, высокую масштабируемость и снижение зависимости от централизованных серверов, что делает ее привлекательной для различных приложений и сценариев.

## Architectural Pattern Model View Controller(MVC)

```rust unimplemented!```

## [Architectural Pattern Microservices](https://backendinterview.ru/architecture/microserices/index.html)

```rust unimplemented!```

## [Architectural Pattern Event Sourcing](https://backendinterview.ru/architecture/architecturesPatterns.html#event-sourcing)

```rust unimplemented!```

## Architectural Pattern CQRS

`CQRS` (Command Query Responsibility Segregation) — это шаблон проектирования, предлагающий разделение команд (изменяющих данные) и запросов (читающих данные) в приложении. 

Разделение команд-запросов (`CQS`) «Функции не должны вызывать абстрактные побочные эффекты... только команды (процедуры) могут вызывать побочные эффекты». - Бертран Мейер: Объектно-ориентированное создание программного обеспечения

`CQRS` — это стиль архитектуры, в котором операции чтения отделены от операций записи. 
 Подход сформулировал Грег Янг на основе принципа `CQS`, предложенного Бертраном Мейером. 
 Чаще всего (но не всегда) `CQRS` реализуется в ограниченных контекстах (bounded context) приложений, проектируемых на основе `DDD`.
 Одна из естественных причин развития `CQRS` — не симметричное распределение нагрузки и сложности бизнес-логики на `read` и `write` подсистем. 
 Большинство бизнес-правил и сложных проверок находится во `write` — подсистеме. 

Мспользование с паттерном `Command` дает увеличение производительности за счет хранения истории состояния.
Фиксируйте все изменения состояния приложения как последовательность событий.
`CQRS` подходит для сложных доменов, которые также выигрывают от дизайна `DDD`.
Наиболее очевидная вещь, которую мы получили, используя `Event Sourcing`, - это то, что теперь у нас есть журнал всех изменений.
В системе хранилища вместо отклонения запроса пользователя, поскольку система больше не может обрабатывать запросы, она принимает событие и обрабатывает его позднее.

`CQRS` хорошо подходит для моделей программирования на основе [событий](https://martinfowler.com/eaaDev/EventNarrative.html). 
Обычно `CQRS` разделяется на отдельные сервисы, взаимодействующие с [Event Collaboration](https://martinfowler.com/eaaDev/EventCollaboration.html)
Это позволяет этим службам легко использовать преимущества [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html)

`Query` - Для извлечения данных приложения следует использовать `Query`. «Запрос данных» всегда действует без сохранения состояния и не может каким-либо образом изменять состояние приложения.

Каждая обработка `Command` обрабатывается внутри одного` UnitOfWork` для обеспечить атомарность примененных изменений. 
Обработка нескольких `Command`s в последовательный способ (если требуется) может быть сделан с помощью `Saga`.
Выданная `Command` может быть проверена и перехвачена/обработана другими Агрегирование через `CommandValidators` и `CommandInterceptor`.


[CQRS](https://deviq.com/design-patterns/cqrs-pattern)

[DomainEvent martinfowler.com](https://martinfowler.com/eaaDev/DomainEvent.html)

[Patterns of Enterprise Application Architecture](https://martinfowler.com/eaaDev/)

[CQRS implementation](https://github.com/instrumentisto/cqrs-rs)

[CQRS implementation](https://github.com/cq-rs/cqrs)

[Command and Query Responsibility Segregation (CQRS)](https://backendinterview.ru/architecture/architecturesPatterns.html#command-and-query-responsibility-segregation-cqrs)
 
## Architectural Pattern Domain-Driven Design (DDD) 

```rust unimplemented!```

`DDD` означает "Domain-Driven Design" (Проектирование с учетом предметной области) и представляет собой методологию и набор принципов, разработанных Эриком Эвансом. 
`DDD` ориентировано на решение сложных задач в области проектирования программного обеспечения, особенно там, где ключевой упор делается на моделирование предметной области.

Вот некоторые ключевые концепции и принципы `DDD`:

**Предметная область (Domain):** Это область, которую охватывает ваше приложение или система. `DDD` предлагает сосредотачиваться на понимании и моделировании предметной области, чтобы лучше адаптировать приложение к бизнес-потребностям.

**Модель (Model):** В `DDD` модель предметной области становится центральным элементом. Модель представляет собой набор абстракций, описывающих ключевые концепции и сущности в предметной области. Она должна быть языком, понятным и близким к бизнес-понятиям.

**Ограниченные контексты (Bounded Contexts):** `DDD` предлагает разделять большие системы на ограниченные контексты, где каждый контекст имеет свою уникальную модель предметной области и границы, определенные для конкретных потребностей внутри этого контекста.

**Агрегаты и сущности (Aggregates and Entities):** Агрегаты представляют собой группу связанных сущностей, образующих единое целое. Сущности - это объекты с идентичностью, которые могут изменять свое состояние.

**Сервисы (Services):** В `DDD` сервисы представляют собой операции или функциональность, которые не принадлежат конкретной сущности или агрегату, но являются частью предметной области.

**Фабрики и репозитории (Factories and Repositories):** Фабрики отвечают за создание сложных объектов, а репозитории - за сохранение и получение объектов из хранилища.

`DDD` предоставляет набор инструментов и подходов, которые помогают разработчикам и бизнес-аналитикам совместно работать над сложными системами, обеспечивая лучшее соответствие кода предметной области требованиям бизнеса. Это особенно полезно в случаях, когда моделирование бизнес-логики является ключевым аспектом разработки.

[Domain model](https://www.cosmicpython.com/book/chapter_01_domain_model.html)

[Domain model](https://www.cosmicpython.com/book/preface.html#_a_brief_overview_of_what_youll_learn)

[DDD](https://backendinterview.ru/architecture/ddd.html)

## Layered architecture 

```rust unimplemented!```

Слоенная архитектура (Layered Architecture) - это структурный подход к организации кода, в котором приложение разделяется на логические слои (или уровни), 
каждый из которых выполняет определенные функции. 
Каждый слой зависит только от слоев, находящихся ниже, и предоставляет интерфейсы для взаимодействия с вышележащими слоями. 
Это помогает создать модульную и легко поддерживаемую структуру приложения.

#### [Ошибки применения Layered Architecture](https://medium.com/@stevebishop_89684/clean-architecture-is-not-a-project-structure-b158c9c4163f)

Что такое Common Closure Principle? Ну, CCP утверждает:

Классы внутри компонента должны быть сгруппированы вместе на основе того же типа изменений, к которым они восприимчивы. Когда изменение влияет на компонент, оно должно влиять на все классы внутри этого компонента и ни на какие другие компоненты.

Следует отметить, что компоненты — это классы, которые связаны с вариантом использования. Поэтому любые классы, которые могут быть затронуты изменениями в варианте использования, должны быть сгруппированы вместе. Но когда вы перемещаете эти классы в отдельные папки, проекты или пакеты, они больше не группируются вместе, и таким образом вы нарушаете CCP. Разделения кода не по его связности, а по уровню функциональности и всем тем печальным аспектам, которые с этим связаны. То, что должно быть простой, понятной кодовой базой, превращается в разбросанную кодовую базу с классами повсюду. В результате любой, кто сталкивается с такими кодовыми базами, слышит утверждения, что это пример Чистой Архитектуры, и приходит к выводу, что во всех гадостях виновата философия.

Ваша кодовая база должна быть организована по функциям, а не по слоям. Приложение растет горизонтально, а не вертикально. То есть вы постоянно добавляете больше функций в приложение, но почти никогда не добавляете больше слоев. Вам нужно спланировать это расширение, чтобы по мере роста вашего кода было не только легче находить классы, связанные с компонентом, но и вы могли легко разделить кодовую базу на отдельные пакеты и службы по мере ее роста. Вы должны упростить поиск швов в вашем коде, где код может быть разделен. Папки — очевидный способ создания этих швов.


Обычно выделяют следующие основные слои в слоенной архитектуре:

##### **Представление (Presentation Layer):**

Этот слой отвечает за отображение данных пользователю и обработку пользовательского ввода.
Включает в себя пользовательский интерфейс, отображение данных и обработку событий.
Зависит от слоя бизнес-логики.

##### **Бизнес-логика (Business Logic Layer или Service Layer):**

Здесь содержится основная бизнес-логика приложения.
Обрабатывает запросы от представления и координирует выполнение бизнес-правил и задач.
Не зависит напрямую от конкретной реализации базы данных или представления.

##### **Слой данных (Data Layer):**

Этот слой управляет доступом к данным и их хранением.
Включает в себя работу с базой данных, файлами, внешними API и т.д.
Зависит от бизнес-логики.
Преимущества слоенной архитектуры включают:

##### **Модульность:** 

Каждый слой предоставляет четко определенный интерфейс, что облегчает замену или модификацию отдельных компонентов без воздействия на другие части приложения.

##### **Понятность и управляемость:** 

Разделение приложения на логические слои делает код более читаемым и понятным. Разработчики могут сосредотачиваться на конкретной функциональности без необходимости вникать в детали других слоев.

##### **Повторное использование:** 

Компоненты внутри каждого слоя могут быть повторно использованы в различных частях приложения или даже в разных приложениях.

##### **Тестирование:** 

Каждый слой может быть легко протестирован независимо от других слоев.

##### **Изменение технологий:** 

Замена или обновление технологий в одном слое не должно существенно влиять на другие слои.

Напоминаю, что слоенная архитектура предоставляет общую концепцию, и конкретная реализация может варьироваться в зависимости от конкретных требований и характеристик проекта.

## Hexagonal Architecture (Шестиугольная архитектура)

```rust unimplemented!```

`Hexagonal Architecture` (Шестиугольная архитектура), также известная как Ports and Adapters (Порты и Адаптеры), это паттерн архитектуры, предложенный Алистером Кокберном. 
Он призван обеспечить легкость тестирования, гибкость и отделение бизнес-логики от деталей инфраструктуры.

`Hexagonal Architecture` поддерживает принципы чистой архитектуры (`SOLID`) и способствует созданию гибких, тестируемых и легко поддерживаемых приложений.

Основные идеи `Hexagonal Architecture`:

##### **Ядро (Core):** 

Это основа приложения, содержащая бизнес-логику и правила предметной области. В ядре не зависят от деталей реализации, таких как базы данных, фреймворки или пользовательский интерфейс.

##### **Порты (Ports):** 

Представляют собой интерфейсы или абстракции, которые определяют, как ядро взаимодействует с внешним миром. Это могут быть интерфейсы для работы с базой данных, веб-службы, пользовательские интерфейсы и т.д.

##### **Адаптеры (Adapters):** 

Это конкретные реализации портов, которые подключают ядро к конкретным технологиям или внешним системам. Они "адаптируют" внешний мир к интерфейсам, определенным в портах.

Преимущества `Hexagonal Architecture`:

##### **Тестируемость:** 

Ядро, не зависящее от инфраструктурных деталей, легко тестируется, так как тесты могут сфокусироваться на бизнес-логике, отделенной от внешних зависимостей.

##### **Гибкость:** 

Подход позволяет легко менять внешние компоненты (адаптеры) без изменения ядра приложения.

##### **Разделение ответственностей:** 

Бизнес-логика отделена от технических деталей, что упрощает понимание и обслуживание кода.


`Hexagonal Architecture` и слоенная архитектура (`Layered Architecture`) представляют собой два различных подхода к организации кода, но оба направлены на достижение подобных целей: улучшение тестируемости, гибкости и отделение бизнес-логики от деталей инфраструктуры. 

**Вот основные различия между ними:**

##### **Организация компонентов:**

**Hexagonal Architecture:** Организована вокруг идеи "портов" (ports) и "адаптеров" (adapters). Ядро (бизнес-логика) зависит от портов, представляющих интерфейсы для взаимодействия с внешним миром. Адаптеры реализуют эти порты и подключают ядро к конкретным технологиям или внешним системам.

**Слоенная архитектура:** Организована вокруг идеи разделения кода на логические слои (presentation layer, business logic layer, data access layer и т.д.), где каждый слой имеет определенные обязанности. Каждый слой может зависеть только от слоев, находящихся ниже.

##### **Фокус на внешние зависимости:**

**Hexagonal Architecture:** Акцент делается на отделении ядра от внешних зависимостей. Бизнес-логика предоставляет порты, и адаптеры реализуют эти порты для взаимодействия с инфраструктурой.

**Слоенная архитектура:** Слои организованы так, чтобы каждый слой знал только о слоях, находящихся ниже. Например, бизнес-логика может вызывать слой доступа к данным, но не наоборот.

##### **Структура кода:**

**Hexagonal Architecture:** Архитектура предполагает, что бизнес-логика находится в центре и зависит только от интерфейсов (портов). Адаптеры реализуются вне ядра, что способствует легкости тестирования и замене внешних компонентов.

**Слоенная архитектура:** Организована слоями, каждый из которых предоставляет определенные функциональности. Взаимодействие между слоями происходит в строго определенной последовательности.

##### **Обмен информацией:**

**Hexagonal Architecture:** Обмен информацией между внешним миром и ядром происходит через порты и адаптеры, что способствует легкости изменения внешних зависимостей.

**Слоенная архитектура:** Обмен информацией обычно происходит внутри слоев, и внешние зависимости передаются через слои.

Оба подхода ценятся за свои преимущества в отношении тестируемости, гибкости и четкого разделения ответственностей. Выбор между Hexagonal Architecture и слоенной архитектурой может зависеть от конкретных требований проекта, предпочтений разработчиков и характера приложения.

# Refactoring

```rust unimplemented!```

Рефакторинг — это процесс изменения программного кода с целью улучшения его структуры, читаемости, поддерживаемости и производительности, при этом без изменения его внешнего поведения. Рефакторинг позволяет разработчикам вносить изменения в программу, не нарушая функциональность и улучшая качество.

Важность рефакторинга становится очевидной по мере роста и развития программного проекта. Код, написанный в начале разработки, может быть неоптимальным и сложным. Накопление такого кода замедляет разработку и закладывает проблемы на будущее. Рефакторинг позволяет устранить этот технический долг, делая код более понятным, гибким и эффективным.

**Главная цель рефакторинга** — изменить структуру кода без изменения его внешнего поведения. Это вызывает потребность в тщательном тестировании после каждого рефакторинга, чтобы убедиться, что код все еще работает корректно.

[Refactoring](https://refactoring.com/)

[Refactoring](https://refactoring.guru/ru/refactoring)

[Refactoring](https://sourcemaking.com/refactoring)

[Что такое рефакторинг?](https://foxminded.ua/ru/refaktoring/)
 
## Sources
 
[Rust Design Patterns rust-unofficial](https://rust-unofficial.github.io/patterns/)

[Rust Design Patterns github.com/rust-unofficial](https://github.com/rust-unofficial/patterns)

[Rust Design Patterns refactoring.guru](https://refactoring.guru/ru/design-patterns/rust)

### OOP Patterns

[Rust Design Patterns](https://chercher.tech/rust/observer-design-pattern-rust)

[Паттерны проектирования](http://design-pattern.ru/patterns/)

[Каталог шаблонов архитектуры корпоративных приложений](https://martinfowler.com/eaaCatalog/index.html)

[Паттерны ООП в метафорах](https://habr.com/ru/articles/136766/)


 
