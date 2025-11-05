[![Hits-of-Code](https://hitsofcode.com/github/Jekahome/Patterns?branch=main)](https://hitsofcode.com/github/Jekahome/Patterns/view?branch=main)

# Topics

* [Что такое паттерны, зачем и почему?](https://github.com/Jekahome/Patterns#%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B-%D0%B7%D0%B0%D1%87%D0%B5%D0%BC-%D0%B8-%D0%BF%D0%BE%D1%87%D0%B5%D0%BC%D1%83)
* [Clean Code Principles](https://github.com/Jekahome/Patterns#clean-code-principles)
* [Семантическая и цикломатическая сложность кода](https://github.com/Jekahome/Patterns?tab=readme-ov-file#%D1%81%D0%B5%D0%BC%D0%B0%D0%BD%D1%82%D0%B8%D1%87%D0%B5%D1%81%D0%BA%D0%B0%D1%8F-%D0%B8-%D1%86%D0%B8%D0%BA%D0%BB%D0%BE%D0%BC%D0%B0%D1%82%D0%B8%D1%87%D0%B5%D1%81%D0%BA%D0%B0%D1%8F-%D1%81%D0%BB%D0%BE%D0%B6%D0%BD%D0%BE%D1%81%D1%82%D1%8C-%D0%BA%D0%BE%D0%B4%D0%B0)
* [Почему так тяжело понимать сложный код](https://github.com/Jekahome/Patterns?tab=readme-ov-file#%D0%BF%D0%BE%D1%87%D0%B5%D0%BC%D1%83-%D1%82%D0%B0%D0%BA-%D1%82%D1%8F%D0%B6%D0%B5%D0%BB%D0%BE-%D0%BF%D0%BE%D0%BD%D0%B8%D0%BC%D0%B0%D1%82%D1%8C-%D1%81%D0%BB%D0%BE%D0%B6%D0%BD%D1%8B%D0%B9-%D0%BA%D0%BE%D0%B4)
* [Программируйте в терминах проблемной области](https://github.com/Jekahome/Patterns/tree/main?tab=readme-ov-file#%D0%BF%D1%80%D0%BE%D0%B3%D1%80%D0%B0%D0%BC%D0%BC%D0%B8%D1%80%D1%83%D0%B9%D1%82%D0%B5-%D0%B2-%D1%82%D0%B5%D1%80%D0%BC%D0%B8%D0%BD%D0%B0%D1%85-%D0%BF%D1%80%D0%BE%D0%B1%D0%BB%D0%B5%D0%BC%D0%BD%D0%BE%D0%B9-%D0%BE%D0%B1%D0%BB%D0%B0%D1%81%D1%82%D0%B8)
* [Programming Paradigms](https://github.com/Jekahome/Patterns#programming-paradigms)
* [Подходят ли ООП паттерны для Rust?](https://github.com/Jekahome/Patterns#%D0%BF%D0%BE%D0%B4%D1%85%D0%BE%D0%B4%D1%8F%D1%82-%D0%BB%D0%B8-%D0%BE%D0%BE%D0%BF-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B-%D0%B4%D0%BB%D1%8F-rust)
   * [ООП vs Rust: Ключевые отличия](https://github.com/Jekahome/Patterns?tab=readme-ov-file#1-%D0%BE%D0%BE%D0%BF-vs-rust-%D0%BA%D0%BB%D1%8E%D1%87%D0%B5%D0%B2%D1%8B%D0%B5-%D0%BE%D1%82%D0%BB%D0%B8%D1%87%D0%B8%D1%8F)
   * [Когда ООП-паттерны не нужны в Rust](https://github.com/Jekahome/Patterns?tab=readme-ov-file#2-%D0%BA%D0%BE%D0%B3%D0%B4%D0%B0-%D0%BE%D0%BE%D0%BF-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B-%D0%BD%D0%B5-%D0%BD%D1%83%D0%B6%D0%BD%D1%8B-%D0%B2-rust)
   * [Когда ООП-паттерны полезны в Rust](https://github.com/Jekahome/Patterns?tab=readme-ov-file#3-%D0%BA%D0%BE%D0%B3%D0%B4%D0%B0-%D0%BE%D0%BE%D0%BF-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B-%D0%BF%D0%BE%D0%BB%D0%B5%D0%B7%D0%BD%D1%8B-%D0%B2-rust)
   * [Функциональные альтернативы в Rust](https://github.com/Jekahome/Patterns?tab=readme-ov-file#4-%D1%84%D1%83%D0%BD%D0%BA%D1%86%D0%B8%D0%BE%D0%BD%D0%B0%D0%BB%D1%8C%D0%BD%D1%8B%D0%B5-%D0%B0%D0%BB%D1%8C%D1%82%D0%B5%D1%80%D0%BD%D0%B0%D1%82%D0%B8%D0%B2%D1%8B-%D0%B2-rust)
   * [Виды полиморфизма](https://github.com/Jekahome/Patterns?tab=readme-ov-file#5-%D0%B2%D0%B8%D0%B4%D1%8B-%D0%BF%D0%BE%D0%BB%D0%B8%D0%BC%D0%BE%D1%80%D1%84%D0%B8%D0%B7%D0%BC%D0%B0)
* [Rust idioms](https://github.com/Jekahome/Patterns#rust-idioms)
* [Anti patterns](https://github.com/Jekahome/Patterns#anti-patterns)
* [Design principles](https://github.com/Jekahome/Patterns?tab=readme-ov-file#design-principles-solid-kiss-dry-yagni-grasp-lod-soc-sla): 
   * [SOLID](https://github.com/Jekahome/Patterns?tab=readme-ov-file#solid), 
   * [KISS](https://github.com/Jekahome/Patterns?tab=readme-ov-file#kiss), 
   * [DRY](https://github.com/Jekahome/Patterns?tab=readme-ov-file#dry), 
   * [YAGNI](https://github.com/Jekahome/Patterns?tab=readme-ov-file#yagni), 
   * [GRASP](https://github.com/Jekahome/Patterns?tab=readme-ov-file#grasp), 
   * [LoD](https://github.com/Jekahome/Patterns?tab=readme-ov-file#lod), 
   * [SoC](https://github.com/Jekahome/Patterns?tab=readme-ov-file#soc)
   * [Single Level of Abstraction (SLA)](https://github.com/Jekahome/Patterns?tab=readme-ov-file#single-level-of-abstraction-sla)
   * [Command-Query Separation (CQS)](https://github.com/Jekahome/Patterns?tab=readme-ov-file#command-query-separation-cqs)
* [Gangs of Four (GoF) Design Patterns](https://github.com/Jekahome/Patterns#gangs-of-four-gof-design-patterns)
   * [Порождающие паттерны](https://github.com/Jekahome/Patterns#%D0%BF%D0%BE%D1%80%D0%BE%D0%B6%D0%B4%D0%B0%D1%8E%D1%89%D0%B8%D0%B5-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B)
   * [Структурирующие паттерны](https://github.com/Jekahome/Patterns#%D1%81%D1%82%D1%80%D1%83%D0%BA%D1%82%D1%83%D1%80%D0%B8%D1%80%D1%83%D1%8E%D1%89%D0%B8%D0%B5-%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B)
   * [Паттерны поведения](https://github.com/Jekahome/Patterns#%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B-%D0%BF%D0%BE%D0%B2%D0%B5%D0%B4%D0%B5%D0%BD%D0%B8%D1%8F)
* [Other Design Patterns](https://github.com/Jekahome/Patterns#other-design-patterns) (...Dependency injection (DI))
* PoSA
* [Database Patterns](https://github.com/Jekahome/Patterns#database-patterns) ([Repository](https://github.com/Jekahome/Patterns#repository), [Unit of Work](https://github.com/Jekahome/Patterns#unit-of-work-uow), Lazy Load)
* [Architecture](https://github.com/Jekahome/Patterns#architecture) ([CQRS](https://github.com/Jekahome/Patterns#architectural-pattern-cqrs), [DDD](https://github.com/Jekahome/Patterns#architectural-pattern-domain-driven-design-ddd), [Layered architecture](https://github.com/Jekahome/Patterns#layered-architecture))
* [Refactoring](https://github.com/Jekahome/Patterns#refactoring)
* [Квадрант технического долга по Мартину Фаулеру](https://github.com/Jekahome/Patterns?tab=readme-ov-file#%D0%BA%D0%B2%D0%B0%D0%B4%D1%80%D0%B0%D0%BD%D1%82-%D1%82%D0%B5%D1%85%D0%BD%D0%B8%D1%87%D0%B5%D1%81%D0%BA%D0%BE%D0%B3%D0%BE-%D0%B4%D0%BE%D0%BB%D0%B3%D0%B0-%D0%BF%D0%BE-%D0%BC%D0%B0%D1%80%D1%82%D0%B8%D0%BD%D1%83-%D1%84%D0%B0%D1%83%D0%BB%D0%B5%D1%80%D1%83)
* [CAP-теорема](https://github.com/Jekahome/Patterns#cap-%D1%82%D0%B5%D0%BE%D1%80%D0%B5%D0%BC%D0%B0)

# Что такое паттерны, зачем и почему?
 
**Паттерны проектирования** — это проверенные способы решения часто встречающихся задач в разработке.
Они помогают сделать код **модульным**, **удобным в сопровождении** и **расширяемым**.

Помимо технической пользы, паттерны дают разработчикам предсказуемость кода - **общий язык**, что упрощает общение в команде и ускоряет принятие решений.

Хорошее понимание паттернов и алгоритмов развивает гибкость мышления, повышает качество конечного продукта и уменьшает вероятность появления «архитектурного мусора» (оставление кривых решений в итоговом продукте, которые видят и ощущают пользователи).

Но, паттерн — это **инструмент**, а не догма. Не всё надо «паттернизировать», иначе получится архитектурная перегрузка.

# Clean Code Principles

"Чистый код" - это концепция, предложенная Робертом Мартином в его книге "Clean Code: A Handbook of Agile Software Craftsmanship". 
Принципы чистого кода призывают к написанию программного кода, который легко читаем, понятен, поддается тестированию и легко поддается изменениям. Вот некоторые из основных принципов чистого кода:

<details>
<summary>...</summary>

<details>
<summary>Ясность (Clarity)</summary>

  Ясный код — это код, который невозможно понять неправильно.

  Код должен быть **максимально понятным и читаемым** для других разработчиков, включая вас самих в будущем.
  Отдавайте предпочтение **простым и очевидным решениям**, даже если они кажутся «излишне примитивными». Простота снижает когнитивную нагрузку и уменьшает вероятность ошибок.

  Ключевые моменты:

  * **Выразительные имена** переменных, функций, классов, модулей.
  * **Отсутствие скрытого поведения** — поведение должно быть очевидным из названия и структуры кода.
  * **Минимум умных трюков** — избегайте чрезмерно оптимизированных или изощрённых конструкций, если они ухудшают понимание.
  * **Предсказуемость** — одинаковый код должен вести себя одинаково в любых условиях.
  
##### Избегание магических чисел и строк:

  Избегайте использования "магических" (хардкодированных) чисел и строк. Используйте константы или переменные с понятными именами.

##### Амбигуальный код (ambiguous code)

  Амбигуальность в коде — это неоднозначность, когда программист не может однозначно понять, что именно произойдёт при его выполнении. Такой код сложнее сопровождать и он чаще становится источником ошибок.

  Примеры неоднозначности:

  1. **Неочевидные приоритеты операций**

  ```rust
  let result = a & b == 0; // Неочевидно: сравнение или побитовое И?
  ```
  2. **Перекрытие переменных**

  ```rust
  let count = 10;
  {
      let count = get_count(); // Затеняет внешнюю переменную
  }
  ```
  3. **Функции с разным поведением в зависимости от типа аргумента**
    (особенно в языках с динамической типизацией или сложным overloading).
  4. **Неявные преобразования типов**
    (часто в JavaScript, C++ и других языках с implicit casting).
  5. **Проблемы многопоточности**
    — когда результат зависит от порядка выполнения потоков, который не гарантирован.

</details>

<details>
<summary>Правила дизайна</summary>

1. **Держите конфигурацию на верхних уровнях**.
   Храните параметры, настройки и конфигурационные данные на высоких уровнях системы, чтобы менять поведение можно было без изменения низкоуровневого кода.
   - *Плюс*: гибкость и упрощённое сопровождение.

2. **Предпочитайте полиморфизм условным конструкциям**.
   Избегайте нагромождения `if/else` или `switch/case` для выбора поведения. Используйте полиморфизм, общий интерфейс или абстрактный базовый класс.
   - *Плюс*: добавление нового поведения не требует изменения старого кода (*Open/Closed Principle*).

3. **Изолируйте многопоточность**.
   Код, связанный с параллелизмом, должен быть отделён от бизнес-логики.
   - *Плюс*: снижение сложности, упрощение тестирования и уменьшение числа ошибок гонок данных.

4. **Избегайте чрезмерной конфигурируемости**.
   Делайте настраиваемым только то, что действительно нужно изменять. Чрезмерная гибкость усложняет систему и делает её менее предсказуемой.
   - *Баланс*: между гибкостью и устойчивостью.

5. **Используйте внедрение зависимостей (Dependency Injection)**.
   Объект должен получать зависимости извне, а не создавать их сам.
   - *Плюс*: тестируемость, модульность, упрощённая замена зависимостей.

6. **Следуйте Закону Деметры** (*Law of Demeter*).
   Общайтесь только со своими непосредственными зависимостями («не разговаривайте с незнакомцами»).
   - *Плюс*: снижение связанности и упрощение сопровождения.

</details>

<details>
<summary>Советы по понятности</summary>

Есть общее с [KISS](https://github.com/Jekahome/Patterns?tab=readme-ov-file#kiss)

1. **Будьте последовательны**.
   Делайте однотипные вещи одинаково. Последовательность повышает предсказуемость и снижает нагрузку на понимание.

   ```rust
   // ❌ — разный стиль имён функций
   fn get_user() { }
   fn fetch_order() { }
   fn retrieve_product() { }

   // ✅ — единообразие
   fn get_user() { }
   fn get_order() { }
   fn get_product() { }
   ```

2. **Используйте поясняющие переменные**.
   Явные и содержательные имена переменных делают код самодокументируемым.

   ```rust
   // ❌
   let s = "2024-07-20T12:34:56Z";
   let t = DateTime::parse_from_rfc3339(s).unwrap();

   // ✅
   let date_string = "2024-07-20T12:34:56Z";
   let parsed_date = DateTime::parse_from_rfc3339(date_string).unwrap();
   ```

3. **Инкапсулируйте граничные условия**.
   Храните проверку диапазонов и пределов в одном месте — это снижает дублирование и ошибки.

   ```rust
   // ❌
   fn is_valid_age(age: u32) -> bool {
       age >= 0 && age <= 120
   }

   // ✅
   const MIN_AGE: u32 = 0;
   const MAX_AGE: u32 = 120;

   fn is_valid_age(age: u32) -> bool {
       age >= MIN_AGE && age <= MAX_AGE
   }
   ```

4. **Предпочитайте специализированные объекты значений примитивам**.
   Используйте отдельные типы вместо "голых" примитивов, чтобы выразить смысл и избежать ошибок.

   ```rust
   struct Money { amount: f64 }

   impl Money {
       fn new(amount: f64) -> Self { Self { amount } }
       fn add(&self, other: Money) -> Money {
           Money::new(self.amount + other.amount)
       }
   }

   let price = Money::new(100.0);
   let tax = Money::new(20.0);
   let total = price.add(tax);
   ```

5. **Избегайте логической зависимости методов**.
   Метод не должен работать правильно только при условии, что перед этим вызван другой метод.

   ```rust
   // ❌
   fn print_total(&self) {
       println!("{}", self.total); // требует, чтобы calculate_total был вызван ранее
   }

   // ✅
   fn print_total(&self) {
       println!("{}", self.calculate_total());
   }
   ```

6. **Избегайте отрицательных условий**.
   Положительные проверки читаются легче, чем отрицательные.

   ```rust
   // ❌
   fn is_not_error(status: &str) -> bool {
       status != "error"
   }

   // ✅
   fn is_success(status: &str) -> bool {
       status == "success"
   }
   ```
</details>
 
<details>
<summary>Правила для функции</summary>

#### Правила для функции

1. **Делайте только одно**.
   Функция должна решать одну задачу и решать её полностью. Это повышает понятность, упрощает тестирование и повторное использование.

   ```rust
   // ❌ — функция делает всё сразу
   fn process_order(order: &Order) {
      /* 
        code validate order 
        ...
      */
      /*
        code save to database  
        ...
      */
      /* 
        code send confirmation email 
        ...
      */
   }

   // ✅ — каждая функция делает одно
   fn validate_order(order: &Order) -> bool { /* ... */ }
   fn save_to_database(order: &Order) { /* ... */ }
   fn send_confirmation_email(order: &Order) { /* ... */ }
   ```

2. **Ограничивайте количество аргументов**.
   Чем больше аргументов, тем сложнее понять и использовать функцию.
   Оптимально — до трёх аргументов. Если нужно больше — объедините их в структуру.

   ```rust
   // ❌
   fn create_user(name: &str, age: u32, email: &str, address: &str) { /* ... */ }

   // ✅ — используем объект для передачи данных
   struct User {
       name: String,
       age: u32,
       email: String,
       address: String,
   }
   fn create_user(user: User) { /* ... */ }
   ```

3. **Избегайте побочных эффектов**.
   Функция не должна менять состояние вне своего тела или модифицировать аргументы неожиданным образом.

   ```rust
   // ❌ — меняет глобальное состояние
   static mut GLOBAL_STATE: HashMap<String, String> = HashMap::new();
   fn update_state(key: String, value: String) {
       unsafe { GLOBAL_STATE.insert(key, value); }
   }

   // ✅ — передаём состояние явно
   fn update_state(state: &mut HashMap<String, String>, key: String, value: String) {
       state.insert(key, value);
   }
   ```

4. **Не используйте флаговые аргументы**.
   Если в функции есть флаг (`true/false`), это признак, что она делает больше одной задачи.
   Разделите её на две или более функций.

   ```rust
   // ❌
   fn set_user_status(user: &mut User, active: bool) {
       if active {
           user.status = "active".to_string();
       } else {
           user.status = "inactive".to_string();
       }
   }

   // ✅
   fn activate_user(user: &mut User) {
       user.status = "active".to_string();
   }
   fn deactivate_user(user: &mut User) {
       user.status = "inactive".to_string();
   }
   ```
</details>

<details>

<summary>Правила комментариев</summary>

Комментарии должны использоваться только там, где это действительно необходимо для понимания кода. Избегайте лишних или бессмысленных комментариев.

**Главный принцип** — комментарии нужны только тогда, когда смысл кода нельзя сделать очевидным самим кодом.

**Маркер `TODO`**

Используйте `TODO` для обозначения незавершённых задач, добавляя идентификатор задачи и ответственного. Это упрощает отслеживание.

```rust
// TODO [ISSUE-101] (Owner: Bob) Refactor this function
fn refactor_function() {
    // ...
}
```

1. **Объясняйте суть в коде, без необходимости обьяснять его в комментариях**.
   Пишите код так, чтобы он был самодокументируемым.

   ```rust
   // ❌
   // Функция для проверки, является ли пользователь активным
   fn check(user: &User) -> bool {
       user.status == "active"
   }

   // ✅
   fn is_user_active(user: &User) -> bool {
       user.status == "active"
   }
   ```

2. **Не дублируйте очевидное. Избегайте «шума»**.
   Комментарий, который повторяет код, бесполезен.

   ```rust
   // ❌
   let x = x + 1; // Увеличиваем x на 1

   // ✅
   let x = x + 1;
   ```
 
3. **Не храните закомментированный код**.
   Лишний код засоряет репозиторий. Если он не нужен — удалите его, история Git его сохранит.
   Если код "ценный" - просто сохраните в свое личное хранилище.

   ```rust
   // ❌
   // fn old_function() {
   //  // старый код
   // }

   // ✅
   fn new_function() {
       // новый код
   }
   ```

4. **Поясняйте намерения т.е. *почему*, а не *что* делает код**.

   ```rust
   // ❌
   let result = calculate(); // Вызываем calculate

   // ✅
   // Кэшируем результат, чтобы избежать повторного вызова тяжелой функции
   let result = calculate();
   ```

5. **Документируйте сложные участки**.
   Если логика нетривиальна — поясните её словами.

   ```rust
   // ❌
   let data = fetch_data();

   // ✅
   // Получаем данные из внешнего API и преобразуем их в формат JSON
   let data = fetch_data();
   ```

6. **Предупреждайте о последствиях**.
   Если операция имеет побочные эффекты — напишите об этом.

   ```rust
   // ❌
   delete_user(user_id);

   // ✅
   // Удаление пользователя приведёт к удалению всех связанных данных
   delete_user(user_id);
   ```

</details>

<details>

<summary>Структура исходного кода</summary>

#### Структура исходного кода

  1. **Разделяйте концепции вертикально**:

      Разные концепции, уровни абстракции или функциональные блоки должны находиться в отдельных частях файла или модуля. Это делает код более организованным и читабельным. Не смешивайте бизнес-логику, валидацию, вспомогательные функции и т.д.

  2. **Группировка кода**
     - Связанный код должен быть размещён вместе.
     - Зависимые функции должны быть близко, для облегчения понимания их взаимодействия.
     - Похожие функции должны быть близко.
 
  3. **Используйте пустые строки для разделения логической структуры, разделения слабо связанных частей**.
       
      ```rust
      // ❌ Плохо
      let a = 1;
      let b = 2;
      let c = 3;
      if a > b {
          println!("a is greater than b");
      }
      let result = a + b + c;
      println!("Result: {}", result);

      // ✅ Хорошо
      let a = 1;
      let b = 2;
      let c = 3;

      if a > b {
          println!("a is greater than b");
      }

      let result = a + b + c;
      println!("Result: {}", result);
      ```

      Фигурные скобки `{...}` для изоляции области видимости переменных, и визуального выделения логической части кода.

      ```rust
      let a = 1;
      let b = {
          // какая-то своя логика ...
          let b = 0;
          a + b
      }
 
      ```

  4. **Держите строки короткими**:

      Короткие строки облегчают чтение кода и предотвращают горизонтальный скроллинг.

      ```rust
      // ❌
      let very_long_variable_name = "This is a very long string that exceeds the recommended line length for readability";
      ```

      ```rust
      // ✅
      let very_long_variable_name = "This is a very long string \
                                  that exceeds the recommended \
                                  line length for readability";
      ```

  5. **Не используйте горизонтальное выравнивание и не нарушайте отступы**:

      Горизонтальное выравнивание переменных и комментариев затрудняет внесение изменений и поддержание кода.
        Когда вы выравниваете переменные и комментарии по горизонтали, изменение одного элемента может потребовать перестановки остальных элементов на той же линии. 
        И `rustfmt` всё равно его уберёт, а выравнивание вручную усложняет правки.

      ```rust
      // ❌
      let first_variable    = 1;    // Первый комментарий
      let second_variable   = 2;    // Второй комментарий

      // ✅
      let first_variable = 1;  // Первый комментарий
      let second_variable = 2; // Второй комментарий
      ```

</details>

<details>
<summary>Объекты и структуры данных</summary>

#### Объекты и структуры данных
 
1. **Упаковывайте внутреннее устройство в безопасный обьект**

Приватность особенностей внутренней структуры обьекта, помогает избежать зависимостей от конкретных деталей реализации и делает ваш код более устойчивым к будущим изменениям.

  ```rust
  // ❌ публичные поля структуры, позволяют использовать ее неверным способом  
  pub struct Order {
      pub items: Vec<Item>,
      pub total: f64,
  }

  // ✅ струтура предоставляет безопасный способ работы с ней
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

Предпочитайте простые структуры данных без избыточного состояния и кэшей, чтобы уменьшить сложность и повысить ясность кода. Логика/поведение должна быть отделена от хранения данных, чтобы облегчить сопровождение и повысить читаемость кода. Избегайте дублирования данных и логики внутри структуры, поддерживайте единую точку истины. Гибридные структуры, которые смешивают поведение и данные, могут быть трудны для понимания и сопровождения. Стремитесь к тому, чтобы структура данных и объекты имели ясное разделение.

✅ структура хранит данные, методы просто оперируют с этими данными (чистые вычисления или простые изменения).

❌ структура хранит данные и дублирует вычисления и состояние.


  ```rust
  pub struct Item {
      pub price: f64,
  }

  // ❌ хранение кэшированного состояния и его поддержка
  pub struct Order {
      items: Vec<Item>,
      total: f64,  // состояние, требующее обновления вручную
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
          self.total += item.price;  // риск рассинхронизации total и items
      }

      pub fn get_total(&self) -> f64 {
          self.total
      }
  }

  // ✅ хранение только данных, вычисление состояния на лету
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
 
3. **Объекты должны делать одно дело**

  Объекты и структуры данных должны быть сфокусированы на выполнении одной задачи.

 

4. **Базовый класс не должен знать о своих производных**

  В объектно-ориентированном программировании, базовый класс не должен иметь зависимости от своих производных классов. 
  
  В Rust вместо базовых классов и наследников используют трейты для описания поведения. Трейт не знает ничего о типах, которые его реализуют — это уже реализаторы (типы) знают про трейт, а не наоборот. Поэтому в Rust эта проблема (базовый класс, знающий о производных) просто не возникает.
 
  Однако, есть случаи, которые можно рассматривать как «нарушение духа» этого принципа, например:

  - 1. **Зависимость от конкретных типов в трейте через ассоциированные типы**

  Если в трейте есть ассоциированные типы (associated types) или связанные константы, и мы заставляем реализацию использовать определённые типы, то в некотором смысле трейт начинает иметь представление о типах, связанных с реализациями.

  ```rust
  pub trait Animal {
      type Food;

      fn eat(&self, food: Self::Food);
  }
  ```

  Но даже здесь трейт просто определяет интерфейс с «параметром», а не конкретно зависит от реализации.

  - 2. **Типажи с вызовом конкретных реализаций (в редких случаях)**

  Если в трейте сделать метод с дефолтной реализацией, который зависит от конкретного типа через downcasting или unsafe — это плохая практика и похожа на нарушение принципа. Впрочем, в стандартном Rust таких вещей не делают.

  - 3. **Связь через замыкания, где трейт «знает» о конкретных типах через параметры**

  Иногда, когда трейт имеет метод, принимающий конкретный тип или замыкание с конкретным захватом, это создаёт более сильную связь, но опять же это касается конкретных вызовов, а не самого трейта.

 5. **Лучше иметь много функций, чем передавать код в функцию для выбора поведения**

  Разделение кода на множество функций, позволяет лучше организовать логику и избежать сложных конструкций, таких как флаговые аргументы.

  ```rust
  // ❌ использование флага для выбора поведения
  pub fn handle_order(order: &Order, use_discount: bool) {
      if use_discount {
          // Применение скидки
      } else {
          // Без скидки
      }
  }

  // ✅ использование отдельных функций, избежание флагов повышает читаемость и упрощает тестирование.
  pub fn handle_order_with_discount(order: &Order) {
      // Применение скидки
  }

  pub fn handle_order_without_discount(order: &Order) {
      // Без скидки
  }
  ```

6. **Предпочитайте нестатические методы статическим**

  Нестатические методы (`&self/&mut self`) позволяют работать с состоянием экземпляра, обеспечивают полиморфизм и расширяемость, в то время как статические методы не имеют доступа к состоянию и чаще используются как конструкторы или вспомогательные функции. В Rust это не строгий догмат, а рекомендация, чтобы делать код более идиоматичным и понятным.
 
</details>

<details>
<summary>Code smells</summary>

#### Code smells

  Code smells — это признаки того, что код может содержать потенциальные проблемы или области для улучшения. Увеличивают когнитивную нагрузку на программиста.

  1. **Rigidity (Жесткость)**

  Программное обеспечение становится трудным для изменения, поскольку небольшое изменение вызывает каскад последующих изменений. Это происходит из-за сильных зависимостей между компонентами системы.

  ```rust
  // ❌ Order напрямую зависит от структуры Item, и добавление новых стратегий скидок, жёстко связано с ним.
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

  ✅ **Как исправить:**

  Используйте абстракции, такие как трейты и интерфейсы, чтобы уменьшить взаимные зависимости.

  ```rust
  #[derive(Debug)]
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

  pub struct Order<T: DiscountStrategy> {
      items: Vec<Item>,
      discount_strategy: T,
  }

  impl<T: DiscountStrategy> Order<T> {
      pub fn new(discount_strategy: T) -> Self {
          Self {
              items: Vec::new(),
              discount_strategy,
          }
      }

      pub fn add_item(&mut self, item: Item) {
          self.items.push(item);
      }

      pub fn apply_discount(&self) -> f64 {
          let total: f64 = self.items.iter().map(|item| item.price).sum();
          self.discount_strategy.apply_discount(total)
      }
  }

  ```

  2. **Fragility (Хрупкость)**

  Программное обеспечение ломается в нескольких местах из-за одного изменения. Это часто вызвано слишком сильной связью между компонентами.

  ```rust
    // ❌ хрупкий код, у Order есть метод update_inventory, что связывает логику заказа с инвентарём.
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

  ✅ **Как исправить:**

  Разделите ответственность между компонентами и используйте принципы инкапсуляции и декомпозиции.
  Чтобы избежать копирования Inventory при клонировании Order, стоит хранить ссылку или использовать Arc (в зависимости от контекста). Здесь для простоты — владение.

  ```rust
    // Inventory как отдельный компонент
    pub struct Inventory;  

    impl Inventory {
        pub fn update(&self) {
            // Логика обновления инвентаря
        }
    }

    pub struct Order {
        items: Vec<Item>,
        inventory: Inventory,
    }

    impl Order {
        pub fn new(inventory: Inventory) -> Self {
            Self {
                items: Vec::new(),
                inventory,
            }
        }

        pub fn add_item(&mut self, item: Item) {
            self.items.push(item);
            self.inventory.update();
        }
    }

  ```

  3. **Immobility (Нет мобильности)**
 
  Невозможно повторно использовать части кода в других проектах из-за связанных рисков и высоких затрат.
  - В коде `apply_discount` жёстко зашит фиксированный способ скидки — 10% (магическое число 0.1).
  - Невозможно легко изменить логику скидки без правки самой структуры `Order`.
  - Из-за такой жёсткой связи сложно переиспользовать логику расчёта скидок в других местах или проектах — каждый раз придется копировать и менять код.

  ```rust
  // ❌ сложно повторно использовать
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

  ✅ **Как исправить:**

  Сделайте ваш код более модульным и независимым, выделяя общие компоненты в библиотеки. 
  - Логика применения скидки вынесена в отдельный трейт `DiscountStrategy`, который можно реализовать любым способом.
  - `Order` не знает деталей, как именно считать скидку, а просто вызывает метод `apply_discount` из переданной стратегии.
  - Благодаря передаче стратегии через ссылку (`&'a T`) можно использовать разные варианты скидок без изменения структуры `Order`.
  - Такой код проще расширять, тестировать и переиспользовать в разных проектах — просто реализуй новую стратегию скидки, не меняя код заказа.
  - Это уменьшает связанность (`coupling`) и улучшает модульность.

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

    // Используем жизненный цикл 'a, чтобы Order не владел стратегией скидки,
    // а только ссылался на нее — так можно использовать одну стратегию для разных заказов.
    pub struct Order<'a, T: DiscountStrategy> {
        pub items: Vec<Item>,
        discount_strategy: &'a T,
    }

    impl<'a, T: DiscountStrategy> Order<'a, T> {
        pub fn new(discount_strategy: &'a T) -> Self {
            Self {
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
 
  Код содержит ненужные усложнения, которые делают его трудным для понимания и поддержки.
 

  ```rust
  // ❌ избыточная сложность
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

  ✅ **Как исправить:**

  Разделите код на более простые и понятные части. Разбил метод `calculate` на `subtotal` и `total` — так понятнее и проще тестировать.

  ```rust
    pub struct Order {
        items: Vec<Item>,
    }

    impl Order {
        pub fn subtotal(&self) -> f64 {
            self.items.iter().map(|item| item.price).sum()
        }

        pub fn total(&self, tax: f64, discount: f64) -> f64 {
            let subtotal = self.subtotal();
            let taxed = subtotal * (1.0 + tax);
            taxed - discount
        }
    }

  ```

  5. **Needless Repetition (Избыточное повторение)**
 
  Код содержит повторяющиеся фрагменты, что делает его сложным для поддержки и увеличивает вероятность ошибок.
 

  ```rust
  // ❌ избыточное повторение
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

  ✅ **Как исправить:**

  Используйте общие функции и абстракции для устранения повторений. Упростил подсчёт цены, избавившись от цикла с мутабельной переменной.

  ```rust
    pub fn calculate_price(item: &Item, quantity: u32) -> f64 {
        item.price * quantity as f64
    }

    pub fn calculate_total(order: &Order) -> f64 {
        order.items.iter().map(|item| calculate_price(item, 1)).sum()
    }

  ```

  6. **Opacity (Непрозрачность)**
 
  Код трудно понять из-за неясных или запутанных конструкций, что затрудняет его поддержку.
 

  ```rust
  // ❌ непрозрачный код
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

  ✅ **Как исправить:**

  Упрощайте код и добавляйте комментарии, чтобы сделать его более понятным.

  ```rust
    pub fn process_data(data: &str) -> String {
        let mut result = String::new();

        for c in data.chars() {
            if c.is_alphabetic() {
                result.push(c.to_ascii_uppercase());
            } else if c.is_numeric() {
                result.push('0'); // Заменяем цифры на символ '0'
            }
            // Другие символы игнорируются
        }

        result
    }

  ```

</details>


<details>
<summary>Code smells по Фаулеру</summary>

Этот список — не догма, а скорее набор ориентиров, которые помогают находить слабые места в коде и понимать, какие рефакторинги можно применить для их исправления.

### **Признаки, связанные с методами (функциями)**

1.  **Длинный метод (Long Method)**
    *   **Что это:** Метод, который слишком длинный и делает слишком много вещей.
    *   **Почему это плохо:** Снижает читаемость, усложняет понимание и повторное использование.

2.  **Большой класс (Large Class)**
    *   **Что это:** Класс, который пытается делать слишком много и содержит множество полей, методов и ответственностей.
    *   **Почему это плохо:** Нарушает принцип единственной ответственности (SRP), становится "богом-объектом".

3.  **Цепочка вызовов (Message Chains)**
    *   **Что это:** Длинные последовательности вызовов вида `a.getB().getC().getD().doSomething()`.
    *   **Почему это плохо:** Создает сильную связь между клиентом и структурой объектов. Изменение в одном звене цепи сломает весь код.

4.  **Завышенные права (Feature Envy)**
    *   **Что это:** Метод одного класса чрезмерно часто использует данные и методы другого класса.
    *   **Почему это плохо:** Показывает, что метод, скорее всего, должен находиться в том классе, к данным которого он так часто обращается.

5.  **Интимные отношения (Inappropriate Intimacy)**
    *   **Что это:** Классы слишком много знают о внутреннем устройстве друг друга (часто через доступ к приватным полям/методам).
    *   **Почему это плохо:** Сильная связность, изменение одного класса почти всегда ведет к изменению другого.

6.  **Расходящиеся модификации (Divergent Change)**
    *   **Что это:** Когда один класс приходится часто менять по разным причинам (например, при изменении в БД и при изменении в формате отчета).
    *   **Почему это плохо:** Нарушение SRP. Класс должен меняться только по одной причине.

7.  **Стрельба дробью (Shotgun Surgery)**
    *   **Что это:** Противоположность "Расходящимся модификациям". Одно изменение в системе требует внесения множества мелких правок в множество классов.
    *   **Почему это плохо:** Сложно отследить все места для изменений, легко что-то упустить.

8.  **Парализующий страх перед добавлением метода (Parallel Inheritance Hierarchies)**
    *   **Что это:** Частный случай "Стрельбы дробью". При добавлении подкласса в одну иерархию, вам приходится добавлять подкласс и в другую, связанную с ней иерархию.

 

### **Признаки, связанные с данными**

9.  **Длинный список параметров (Long Parameter List)**
    *   **Что это:** Метод с большим количеством параметров.
    *   **Почему это плохо:** Трудно понять и использовать, легко перепутать порядок аргументов.

10. **Группа данных (Data Clumps)**
    *   **Что это:** Несколько полей (например, `String country`, `String city`, `String street`) или параметров, которые всегда передаются вместе.
    *   **Почему это плохо:** Лучше объединить их в отдельный класс-структуру (например, `Address`).

11. **Примитивная мания (Primitive Obsession)**
    *   **Что это:** Использование примитивных типов (строк, чисел) вместо маленьких объектов для представления доменных понятий (например, `String phone` вместо класса `PhoneNumber`).
    *   **Почему это плохо:** Упускается возможность поместить поведение, связанное с этими данными, в один класс.

12. **Отклоняющийся код (Speculative Generality)**
    *   **Что это:** Наличие классов, методов или полей, которые "на всякий случай" добавлены для будущей функциональности, которая еще не нужна (YAGNI - You Ain't Gonna Need It).
    *   **Почему это плохо:** Бесполезно усложняет код.

 
### **Признаки, связанные с наследованием**

13. **Отказ от наследства (Refused Bequest)**
    *   **Что это:** Подкласс использует только часть унаследованных методов и свойств родителя.
    *   **Почему это плохо:** Нарушает принцип подстановки Барбары Лисков (LSP). Часто лучше использовать композицию вместо наследования.

14. **Дублирование кода (Duplicate Code)**
    *   **Что это:** Один и тот же или очень похожий код встречается в нескольких местах.
    *   **Почему это плохо:** Главный враг сопровождения. Исправление ошибки требует изменений в нескольких местах.

15. **Временное поле (Temporary Field)**
    *   **Что это:** Поле в классе, которое заполняется и используется только в определенных scenarios (часто в одном методе), а в остальное время равно `null` или не имеет смысла.
    *   **Почему это плохо:** Сбивает с толку, так как ожидается, что поля объекта всегда имеют осмысленные значения.

 

### **Общие признаки и запахи дизайна**

16. **Комментарии (Comments)**
    *   **Важно:** Фаулер не против комментариев вообще. "Запахом" являются комментарии, которые *маскируют* плохой код. Часто лучше отрефакторить код, чтобы он стал самодокументируемым, чем писать поясняющий комментарий.

17. **Ленивый класс (Lazy Class)**
    *   **Что это:** Класс, который слишком мал и не несет достаточной пользы, чтобы оправдать свое существование (например, после рефакторинга он стал почти пустым).
    *   **Почему это плохо:** Бесполезная сложность.

18. **Данные-классы (Data Class)**
    *   **Что это:** Класс, который содержит только поля и геттеры/сеттеры, без какой-либо бизнес-логики.
    *   **Почями это плохо:** Поведение, связанное с этими данными, размазано по другим классам, что приводит к "Завышенным правам".

19. **Свистопляска с операторами switch (Switch Statements)**
    *   **Что это:** Длинные или повторяющиеся операторы `switch` (или `if/else if`).
    *   **Почему это плохо:** Часто нарушают OCP (Принцип открытости/закрытости). Добавление нового case требует модификации существующего кода. Решение — полиморфизм.

20. **Игнорирование исключений (Ignored Exceptions)**
    *   **Что это:** Пустой блок `catch`.
    *   **Почему это плохо:** Ошибка подавляется, программа продолжает работать в неконсистентном состоянии, что приводит к непредсказуемым последствиям.

21. **Альтернативные классы с разными интерфейсами (Alternative Classes with Different Interfaces)**
    *   **Что это:** Два класса выполняют схожую функцию, но имеют разные имена методов или сигнатуры.
    *   **Почему это плохо:** Запутывает и усложняет выбор правильного класса.

22. **Недостоверные имена (Mysterious Name)**
    *   **Что это:** Имена переменных, методов или классов, которые неясно отражают их назначение.
    *   **Почему это плохо:** Код становится нечитаемым. Хорошее именование — одна из самых важных вещей в программировании.


---

</details>

* #### Минимизация повторений (DRY - Don't Repeat Yourself):

    Избегайте дублирования кода. Если у вас есть повторяющийся код, вынесите его в отдельную функцию, метод или класс.

* #### Маленькие функции (Small Functions):

    Функции должны быть небольшими и выполнять одну четко определенную задачу.
    Если функция становится слишком большой, разделите ее на более мелкие функции с понятными именами.

* #### Принцип единственной ответственности (Single Responsibility Principle - SRP):

    Каждый класс или функция должны быть ответственными только за одну вещь. Это облегчает понимание и изменение кода.

* #### Принцип открытости/закрытости (Open/Closed Principle - OCP):

    Код должен быть открыт для расширения, но закрыт для модификации. Это достигается путем использования абстракций и полиморфизма.

* #### Тестирование:

    Пишите тесты для вашего кода, чтобы обеспечить его корректность и устойчивость к изменениям.
    Следуйте принципу "Тестирование приводит к чистому коду".
    Тесты заставляют думать о дизайне — чтобы код было легко тестировать, он обычно становится более модульным, с хорошо отделёнными зонами ответственности.

  - Раннее обнаружение ошибок — тесты выявляют баги сразу, предотвращая накопление сложных проблем.
  - Документация поведения — тесты служат живым описанием того, как должен работать код.
  - Упрощение рефакторинга — с тестами можно смело менять структуру кода, не боясь сломать функциональность.
  - Меньше избыточной сложности и дублирования — чтобы тесты были простыми, код обычно становится проще и чище.


* #### Если нашли место для рефакторинга, следует это рефакторить:

    При **внесении изменений** в код, следите за тем, чтобы код оставался чистым или становился чище, чем был до ваших изменений.

    Правило бойскаута - "Оставьте место стоянки чище, чем оно было до вас". 
    Правило бойскаута помогает избежать накопления технического долга и поддерживать кодовую базу в хорошем состоянии без лишних усилий.


<details>
<summary>Стив Макконнелл. «Совершенный код». Глава 24. Рефакторинг</summary>

[Steve McConnell](https://stevemcconnell.com/books/) - (Code Complete)

Факторинг - это декомпозиция на составные части.

Рефакторинг - это изменение работоспособного кода, не влияющее на поведение программы, если рефакторить не рабочий код то это называется хакерство.

Причины рефакторинга:
- код повторяется
- метод слишком велик или принимает много параметров
- большая вложенность цикла
- класс с большой связанностью
- интерфейс класса не формирует согласованную абстракцию
- класс имеет две и более областей ответственности из-за чего его части изменяются независимо от других
- одно изменение требует множественного изменения в других классах
- зависимость от иерархии параллельного наследования
- родственные елементы данных используемые вместе не организованны вместе
- метод использует больше елементов другого класса чем своих
- один класс слишком много знает о другом классе
- данные-члены сделаны открытыми, что стирает грань между интерфейсов класса и реализацией
- подкласс использует лишь малую часть методов своих предков, измените отношение классов с "яыляется" на "содержит" (композиция,агрегация) и реализуйте интерфейс с этой малой долей методов
- программа содержит код который когда-то понадобится
 

</details>

---

</details>



# Семантическая и цикломатическая сложность кода

<details>
<summary>...</summary>

#### 1. Семантическая сложность

**Семантическая сложность** — это мера того, насколько сложно понять смысл и логику кода, включая намерения разработчика и взаимодействие кода с другими частями системы. В отличие от цикломатической, эта метрика не формализована и часто субъективна, так как сильно зависит от опыта и контекста восприятия программиста.

Факторы, влияющие на семантическую сложность:

* **Читаемость кода:** Хорошие имена переменных и функций, комментарии, логичная структура кода помогают легче понять, что он делает.
* **Уровень абстракции:** Использование паттернов и уровней абстракции может либо упростить, либо усложнить понимание — зависит от того, насколько они понятны команде.
* **Сложность алгоритмов:** Многочисленные вложенные условия и ветвления затрудняют понимание логики.
* **Зависимости и взаимодействия:** Сложные связи между модулями и компонентами повышают общую сложность восприятия.
 

#### 2. Цикломатическая сложность

Цикломатическая сложность измеряет количество независимых путей выполнения в программе, отражая её структурную сложность. Основана на теории графов: программа представляется как граф потока управления.

Формула расчёта цикломатической сложности `M`:

Цикломатическая сложность \( `M` \) для графа потока управления программы рассчитывается по формуле:

$$ M = E - N + 2\cdot P  $$

где:
- E — количество рёбер в графе.
- N — количество узлов в графе.
- P — количество связанных компонентов (для большинства программ это обычно 1).
 
**Пример**:

```python
def example(a, b):
    if a > b:
        return a - b
    elif a < b:
        return b - a
    else:
        return a + b
```

* Узлы (N): 6 (условия и возвраты)
* Рёбра (E): 7 (ветвления условий и переходы)
* Связанные компоненты (P): 1

Вычисляем:

$$ M = E - N + 2\cdot P = 7 - 6 + 2 \cdot 1 = 3 $$

Значит, у программы 3 независимых пути.
 

Интерпретация значений цикломатической сложности:

* **1–10:** Низкая, простой и понятный код
* **11–20:** Средняя, возможно стоит подумать о рефакторинге
* **21–50:** Высокая, сложный для поддержки и тестирования код
* **50+:** Очень высокая, необходима декомпозиция на более простые части
 
### Итог

* **Семантическая сложность** — субъективна, связана с восприятием и пониманием логики кода человеком.
* **Цикломатическая сложность** — объективна, даёт количественную оценку структуры и количества путей в коде.

Обе метрики важны для поддержания качества, удобочитаемости и тестируемости кода.

---

</details>

# Почему так тяжело понимать сложный код

> Книга "Ум программиста. Как понять и осмыслить любой код." Автор - Фелин Хермане.

**Когнитивная нагрузка** — это предел того, что может обработать рабочая память. **Рабочая память** это про вычисление, работает с того момента, как мы начинаем думать о коде.
Если вы испытываете большую когнитивную нагрузку, то вы не сможете пра­ вильно обработать код.
Когнитивная нагрузка возникает в тот момент, когда ваша рабочая память перегружается и мозг становится не в состоянии обрабатывать поступающую информацию.

<details>
<summary>...</summary>

## 4.1. Почему так тяжело понимать сложный код

Как и кратковременная память, рабочая память способна обрабатывать **от двух до шести элементов одновременно**. 
В контексте рабочей памяти эта способность больше известна как когнитивная нагрузка. 
Попытка решить задачу, содержащую множество отдельных элементов, которые нельзя эффективно разделить на чанки (паттерны), приведет к «перегрузке» рабочей памяти.

### Типы когнитивной нагрузки:
1. Внутренняя нагрузка - Насколько задача сложна сама по себе.
   Задача требующая определенных знаний в этой узкой области и нет других способов решить ее или упростить. Так ее называется внтуренней сложностью.
2. Внешняя нагрузка  -  Какие отвлекающие факторы усугубляют задачу
   Часто случайно, как результат отсутствия рефакторинга, требуется запоминать не нужные данные не влияющие на саму задачу, плохое оформление что мешает понять саму задачу и делает ее сложнее чем это есть на самом деле.
3. Соответствующая нагрузка  - Когнитивная нагрузка, созданная необходимостью хранить все мысли в долговременной памяти

 
### Способы снижения когнитивной нагрузки - Рефакторинг. Замена незнакомых языковых конструкций. 
Это рефакторинг кода до понятной и знакомой формы.
Рефакторинг— это перепроектирование кода с целью улучшения его внутренней структуры, но без изменения его интерфейсов, для облегчения сопровождения кода в дальнейшем. Например убрать дублирование и разделить обьемный код на части. Однако код, который будет легок в сопровождении, не всегда может быть удобен при чтении. Поэтому иногда вам захочется сделать рефакторинг кода, чтобы упростить его чте­ние в данный период времени, а не для того, чтобы упростить сопровождение кода в долгосрочной перспективе. Такой рефакторинг называется когнитивным. Цель когнитивного рефакторинга за­ключается в том, чтобы сделать код более читаемым для данного читателя в дан­ный момент времени. Когнитивный рефакторинг иногда может включать в себя обратный рефакторинг, который снижает удобство эксплуатации, например встраивание кода — реализа­ция метода и копирование тела функции в место вызова. Во многих случаях когнитив­ный рефакторинг носит лишь временный характер и предназначен для того, чтобы человек мог понять код. Затем, как только человек начинает понимать его, когни­тивный рефакторинг не используется.


Когда ваша рабочая память будет на пределе, вы можете воспользоваться специальными приемами, которые помогут вам сосредоточиться на нужных частях кода. Создание графа зависимостей для вашего кода может помочь вам понять логику и
прочитать код, следуя логической последовательности.

В моем понимании, проблема нехватки рабочей памяти (сложная запутанная архитектура или сложное вычисление), например если код состоит из множества подмодулей с изменением общего состояния в них, тут без знания особенностей всех возможностей модулей не понять поведение кода, придется делать **карту поведения**, внешне может выглядеть как **карта знания** со стрелками на понятные куски кода и общую динамику **timeline** изменения состояния с комментариями. Процесс мысленного выполнения кода называется трассировкой или когнитивной компиляцией.

Имена переменных. Если вы не знаете, что означает та или иная переменная, то код вызовет у вас определенные сложности. Вот почему понятные имена переменных могут помочь нам углубленно понять код. 
Например прикладная Венгерская нотация, в ней у префиксов есть конкретное значе­ние, а не тип переменной (от типа в названии отказались когда появилась поддерка отображения типа в редакторах). `lCustomers` это length customers т.е. для понятия длины массивы пользователей, а `cCoiors` count colors для подсчета количества цветов. 


**Боритесь со сложностью** ("[Steve McConnell](https://stevemcconnell.com/books/) - (Code Complete)"):
- Разделите систему на подсистемы на уровне архитектуры, чтобы концентрироваться в каждый конкретный момент времени на меньшей части системы.
- Тщательно определяйте интерфейсы классов, чтобы можно было игнорировать внутреннее устройство классов. Поддерживайте абстракцию, формируемую интерфейсом класса, чтобы не запоминать ненужных деталей.
- Избегайте глобальных данных, потому что их использование значительно увеличивает процент кода, который нужно удерживать в уме в любой момент времени. Глобальные данные вносят в код неопределенность.
- Избегайте глубокой вложенности циклов и условных операторов, поскольку их можно заменить на более простые управляющие структуры, позволяющие бережнее расходовать умственные ресурсы.
- Избегайте операторов goto, так как они вносят в программу нелинейность, за которой большинству людей трудно следовать.
- Используйте ясные, очевидные имена переменных, чтобы не вспоминать детали вроде «i — это индекс счета, а j — индекс клиента или наоборот?».
- Присваивайте промежуточным переменным промежуточные результаты вычислений с целью документирования этих результатов.

Программирование развивается преимущественно за счет **повышения абстрактности** программных компонентов.
Фред Брукс утверждает, что самым крупным достижением в компьютерных науках можно считать переход от машинного языка к высокоуровневым языкам: он освободил программистов от забот об особенностях отдельных устройств и позволил сосредоточиться на самом программировании (Brooks, 1995).

Функциональное именование переменных, отвечающее на вопрос «что?» уровня проблемы, а не «как?» уровня реализации, повышает уровень абстракции. Если вы говорите: «Я выталкиваю элемент из стека, получая данные о самом последнем сотруднике», — абстракция может избавить вас от выполнения умственного этапа «Я выталкиваю элемент из стека». Вы просто говорите: «Я получаю данные о самом последнем сотруднике». Эта выгода невелика, но если вы пытаетесь сократить диапазон сложности, простирающийся от 1 до 109, важен каждый шаг к цели.

---

</details>

# Программируйте в терминах проблемной области
 
[Steve McConnell](https://stevemcconnell.com/books/) - (Code Complete)

Одним из методов борьбы со сложностью, является работа на максимально высоком уровне абстракции. Один способ достижения этой цели заключается в работе в терминах проблемы программирования, а не ее компьютерного решения.

Высокоуровневый код не должен включать подробных сведений о файлах, стеках, очередях, массивах, символах и подобных объектах, имеющих имена вроде `i`, `j` и `k`. Высокоуровневый код должен описывать решаемую проблему. Он должен быть наполнен описательными именами классов и вызовами методов, ясно характеризующими выполняемые действия, а не подробными сведениями о том, что файл открывается в режиме «только для чтения». Высокоуровневый код не должен быть загроможден комментариями, гласящими, что «здесь переменная `i` представляет индекс записи из файла о сотрудниках, а чуть позже она используется для индексации файла счетов клиентов». 

Это неуклюжая методика программирования. На самом высоком уровне программы не нужно знать, что данные о сотрудниках представлены в виде записей или хранятся в файле. Информацию, относящуюся к этому уровню детальности, надо скрыть. На самом высоком уровне вы не должны иметь понятия о том, как хранятся данные. Вы не должны читать комментарии, объясняющие роль переменной `i` и то, что она используется с двойной целью. Вместо этого вы должны видеть две переменные с выразительными именами, такими как `employeeIndex` и `clientIndex`.

<details>
<summary>Разделение программы на уровни абстракции</summary>

Очевидно, что на некотором уровне надо работать и в терминах реализации, но вы можете изолировать эти части программы от частей, разработанных в терминах проблемной области. 

Проектируя программу, обдумайте уровни абстракции:

|уровни абстракции| 
|:------:| 
|4. высокоуровневые элементы | 
|3. низкоуровневые элементы | 
|2. низкоуровневые структуры реализации | 
|1. структуры и средства языка программирования | 
|0. возможности операционной системы и машинные команды | 

Программа может быть разделена на несколько уровней абстракции. Удачное проектирование позволяет программистам проводить значительную часть времени, сосредоточившись только на верхних уровнях, игнорируя более низкие уровни.

**Уровень 0**: возможности операционной системы и машинные команды. Если вы программируете на высокоуровневом языке, можете не беспокоиться о самом низком уровне: язык позаботится об этом автоматически. Если же вы используете низкоуровневый язык, попробуйте создать ради своего удобства более высокие уровни, хотя многие программисты этого не делают.

**Уровень 1**: структуры и средства языка программирования. Структуры языка программирования — это **элементарные типы данных, управляющие структуры** и т. д. Кроме того, большинство популярных языков снабжено дополнительными библиотеками, предоставляют доступ к вызовам ОС и т. д. Вы используете эти структуры и средства естественным образом, так как программировать без них невозможно. Многие программисты никогда не поднимаются выше этого уровня абстракции, чем значительно осложняют себе жизнь.

**Уровень 2**: низкоуровневые структуры реализации. Низкоуровневые структуры реализации относятся к чуть более высокому уровню, чем структуры, предоставляемые самим языком. В большинстве своем это операции и типы данных, которые вы изучали в вузе: **стеки**, **очереди**, **связные списки**, **деревья**, индексированные файлы, последовательные файлы, алгоритмы сортировки, поиска и т. д. Если вы будете писать программу полностью на этом уровне, вам придется работать со слишком большим числом деталей, чтобы победить в битве со сложностью.
 
**Уровень 3**: низкоуровневые элементы проблемной области. На этом уровне вы имеете **дело с примитивами, нужными для работы в терминах проблемной области**. Это **клей**, скрепляющий нижележащие структуры компьютерных наук и высокоуровневый код проблемной области. Чтобы писать код на этом уровне, вы должны определить словарь проблемной области и создать **строительные блоки**, годные для решения поставленной задачи. Во многих приложениях этим уровнем является **уровень бизнесобъектов или уровень сервисов**.

В качестве элементов словаря и строительных блоков данного уровня выступают **классы**. Возможно, эти классы слишком примитивны, чтобы их можно было задействовать для решения проблемы непосредственно на этом уровне, однако они формируют каркас, на основе которого можно решить проблему, используя классы более высокого уровня.


**Уровень 4**: высокоуровневые элементы проблемной области. Этот уровень формирует абстракцию, позволяющую работать с проблемой в ее собственных терминах. Код, написанный на этом уровне, должен быть частично понятен даже людям, далеким от программирования — возможно, и вашим заказчикам. **Он будет слабо зависеть от специфических аспектов языка программирования, потому что вы будете использовать для работы над проблемой собственный набор средств. Так что на этом уровне ваш код больше зависит от средств, созданных вами на уровне 3, чем от возможностей языка**.

Детали реализации уже должны быть скрыты на два уровня ниже — на уровне структур компьютерных наук, чтобы **изменения оборудования или ОС совсем не влияли на этот уровень**. Выразите в программе на этом уровне пользовательское представление о мире, потому что когда программа изменяется, она изменяется в терминах пользователя. Изменения проблемной области будут сильно влиять на этот уровень, но **вы сможете легко адаптировать к ним программу, создавая новую версию на основе строительных блоков предыдущего уровня**.

Многие программисты находят полезным дополнение этих концептуальных уровней другими, перпендикулярными «уровнями». Например, типичная трехуровневая архитектура пересекает описанные выше уровни, предоставляя дополнительные средства интеллектуального управления аспектами проектирования и кодом.

---

</details>

# Programming Paradigms

**Парадигма** — это стиль программирования и набор принципов использования средств языка. Многие языки поддерживают несколько парадигм (мультипарадигмальные).

<details>
<summary>Основные парадигмы</summary>
 
### **Императивное программирование (Imperative Programming)**.

Похоже на то, что делают компьютеры, - это последовательность **команд**, которые выполняются последовательно.
Фокус на том, **как** решать задачу.
Почти все языки обладают императивной природой.  

Год появления: \~1940-е (машинный код)


**Пример на Python:**

```python
total = 0
for i in range(5):
    total += i
print(total)
```
т.е.  мы буквально говорим **как** выполнять код каждый шаг - вот переменная `total`, теперь пройдись в цикле от 0 до 5 и суммируй в `total` порядковый номер итерации там образом... 


**Проблема императивного кода для ООП**

ООП строится на других принципах:
* **Инкапсуляция**: объект сам управляет своим состоянием.
* **Сообщения**: мы общаемся с объектами через вызовы методов, не влезаем внутрь.
* **Полиморфизм**: мы подменяем реализацию объекта, но внешний код от этого не ломается.

А что делает императивный код?
* Он разрушает инкапсуляцию: вместо того, чтобы сказать объекту "сделай X", мы берём данные и сами решаем, как менять.
* Он увеличивает связность (coupling): внешний код начинает зависеть от структуры данных, а не от интерфейса объекта.
* Он ломает декларативность: вместо "что", мы начинаем описывать "как". В итоге класс превращается в набор тупых полей (DTO), а логика оказывается в статических функциях/процедурах.

---

### **Процедурное программирование (Procedural Programming)**.

Фокус — последовательное выполнение команд.
Указывает компьютеру **что делать шаг за шагом**. Пример: язык Си.
Используются: процедуры, функции, циклы, условные операторы (`if`).

Процедурное программирование это подмножество императивного.

Для уменьшения дублирования используют процедуры (функции без возвращаемого значения) и функции (с возвратом результата).
Практически все языки поддерживают процедурный стиль.

Год появления: \~1950-е


**Пример на C:**

```c
#include <stdio.h>

void greet() {
    printf("Hello, world!\n");
}

int main() {
    // Сначала вызываем функцию greet
    greet();

    // Затем выполняем завершение программы
    return 0;
}
```

---

### **Структурное программирование (Structured Programming)**.

Структурное программирование — это подстиль императивного программирования, возникший в 60–70-х годах как реакция на «спагетти-код» с goto и сложностью управления сложностью кода, появилась необходимость в абстрациях.

Его основные принципы:
* Три базовых управляющих конструкции:
    * последовательность (инструкции выполняются по порядку),
    * ветвление (`if/else`, `switch`),
    * цикл (`while`, `for`).
    (всё остальное можно выразить через них).
* Разбиение программы на функции/подпрограммы для повышения читаемости.
* Запрет на goto (или сильное ограничение его применения).

Пример языков: C, Go, C#.

Год появления: 1966 (Эдгар Дейкстра)

Избегает использования goto, использует последовательности, ветвления и циклы для упорядочивания кода.

**Пример на C:**

```c
int factorial(int n) {
    int result = 1;
    for(int i = 1; i <= n; i++) {
        result *= i;
    }
    return result;
}
```

---

### **Декларативное программирование (Declarative Programming)**.

Декларативное про­граммирование ближе к естественному образу мышления, в ко­тором у нас есть сущности и **отношения** между ними.
Акцент на описании **чего** мы хотим достичь, а не **как**.
Пример — язык запросов SQL, где описывается запрос, а не алгоритм его выполнения.
Обычно декларативные языки минимизируют использование состояний и переменных.
Функциональное программирование — частный случай декларативного.

Год появления: \~1970-е (SQL и логическое программирование)

**Пример SQL:**

```sql
SELECT name FROM users WHERE age > 18;
```

```rust
println!("{}", (1..11).fold(0, |a, b| a + b));
```

т.е. мы говорим **что** нам надо с минимум деталей в отличии от императивного подхода описания каждого шага. 

```rust
// Императивный
let mut sum = 0;
for x in &arr {
    sum += x;
}

// Декларативный
let sum: i32 = arr.iter().sum();
```

Пример Java функция интервал в императивном стиле:
```java
public static int between(int 1, int r, int х) {
    return Math.min(Math.max(l, х), r);
}
int у = Math.between(S, 9, 13);// сразу вычисляется
```

Пример Java функция интервал в декларативном стиле:
```java
ciass Between implements Number {
    private final Number num;
    Between(Number left, Number right, Number х) {
        this.num ~ new Min(new Max(left, х), right);
    }
    @Override
    public int intValue() {
        return this.num.intValue();
    }
}
Number у = new Between(S, 9, 13); // еще не вычисляется!
```

Такой стиль будет декларативным, поскольку я не указываю процессору, что вы­числения нужно выполнить сразу. 
Я просто определил, что это такое, и оставил на усмотрение пользователя решение о том, когда (и нужно ли вообще) вычислять переменную у методом intValue(). Я еще не дал никакой работы процессору. Как указано в определении, выразил логику, не описывая процесс.

Смешивание стилей ведёт к “разрастанию” императивного кода: как только появляются мутации (значит, что мы будем менять состояние пошагово - накапливать, обновлять, мутировать коллекцию и т.п.) и шаги **как сделать**, код становится менее предсказуемым и тянет за собой похожие решения. В Rust это особенно видно: стоит начать с mut и циклов — и легко скатиться в "обычный C-подобный код"

---

### **Функциональное программирование (Functional Programming)**.

Функциональное программирование (ФП) — это подмножество декларативного подхода.

Акцент на построении программ из чистых функций и композиции функций (`fnc().func().func()`), избегающих побочных эффектов. В Rust аналог — цепочки итераторов.
В функциональном программировании функции должны быть **чистыми** — не изменять состояние, а лишь принимать аргументы и возвращать результат. 

Языки: Haskell, Lisp, Erlang, Clojure, Scheme и др.

Чистые функции - результат таких функций зависит лишь от входных параметров. Не надо ломать голову «хм, а что будет, если перед вызовом этой функции я запущу вот эту».

[Правила фп](https://senior.ua/articles/prostye-funkcionalnye-metody-programmirovaniya-na-rust): 
- Нет мутаций данных: это означает, что объект данных не должен быть изменен после его создания.
- Неявное состояние: скрытого / неявного состояния следует избегать. В функциональном программировании состояние не исключается, вместо этого оно делается видимым и явным.

<details>
<summary>...</summary>

Это означает:
- Отсутствие побочных эффектов: функция или операция не должны изменять любое состояние за пределами своей функциональной области. Т.е. функция должна только возвращать значение вызывающему и не должна влиять на любое внешнее состояние. Это означает, что программы легче понять.
- Только чистые функции: функциональный код идемпотентен. Функция должна возвращать значения только на основе переданных аргументов и не должна влиять (побочный эффект) или зависеть от глобального состояния. Такие функции всегда дают один и тот же результат для одних и тех же аргументов.

Год появления: 1958 (с появлением Lisp) 

В  нулевых годах начали массово распространяться многоядерные и многопроцессорные системы. Возникла потребность в распределенных вычислениях, а чуть позже в вычислениях на графических процессорах. Оказалось, что ООП справляется с такими задачами значительно хуже, чем функциональные программы.

**Пример на Rust (итераторы):**

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let sum_of_squares: i32 = numbers.iter()
        .map(|x| x * x)
        .sum();
}
```

Функциональная парадигма - это подвид декларативной парадигмы программирования, которая строится на функциях, что удобно для параллельной и распределенной разработки. Программам, написанным с использованием данной парадигмы, свойственны такие свойства, как высокая степень параллелизации вычислений, повышенные требования к производительности и надежности. Не меняет внешнего состояния и не зависит от глобальных данных только от того что передали. Сразу готова к многопоточности и нет необходимости в блокировке данных. Все состояния хранится в стеке функции в виде аргументов.

1. Неизменяемость данных

В функциональном программировании данные неизменяемы. Это означает, что при изменении данных создается новый объект, а старый остается неизменным. Это позволяет избежать ошибок, связанных с изменением данных во время выполнения программы.

2. Чистые функции

Чистые функции - это функции, которые не имеют побочных эффектов и всегда возвращают одинаковый результат при одинаковых входных данных. Такие функции легко тестировать и сопровождать.

```rust
fn sum(a: usize, b: usize) -> usize {
    return a + b;
}
```

3. Композиция функций

Композиция функций - это процесс объединения нескольких функций в одну. Это позволяет создавать более сложные функции и повторно использовать код. Кроме того, композиция функций улучшает читаемость кода и позволяет избежать дублирования кода.

```rust
fn multiply_by_two(x: i32) -> i32 {
    x * 2
}

fn add_three(x: i32) -> i32 {
    x + 3
}

// Функция-композитор: принимает две функции и возвращает новую
fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    move |x| f(g(x))
}

fn main() {
    // Создаём композицию: сначала multiply_by_two, потом add_three
    let multiply_then_add = compose(add_three, multiply_by_two);

    let value = 5;
    println!("Результат: {}", multiply_then_add(value)); // (5 * 2) + 3 = 13
}
```

4. Функции высшего порядка

Функции высшего порядка - это функции, которые принимают другие функции в качестве аргументов или возвращают функции в качестве результата. Это позволяет создавать более абстрактный код и повышать его гибкость.

Ленивая оценка или нестрогая оценка - это процесс задержки оценки выражения до тех пор, пока он не понадобится. В общем, Rust проводит строгую / энергичную оценку. Мы можем использовать функции высшего порядка, замыкания и методы запоминания для выполнения ленивых вычислений.

```rust
fn add(x: usize) -> usize {
    // Это выводится, так как функции оцениваются в первую очередь
    println!("executing add"); 
    return x + x;
}

fn multiply(x: usize) -> usize {
    // Это выводится, так как функции оцениваются в первую очередь
    println!("executing multiply"); 
    return x * x;
}

fn add_or_multiply(add: bool, on_add: usize, on_multiply: usize) -> usize {
    if add {
        on_add
    } else {
        on_multiply
    }
}
fn main() {
    println!("{}", add_or_multiply(true, add(4), multiply(4))); // 8
    println!("{}", add_or_multiply(false, add(4), multiply(4))); // 16
}
```

Это даст следующий вывод, и мы видим, что обе функции выполняются всегда

```
executing add
executing multiply
8
executing add
executing multiply
16
```

Мы можем использовать функции **высшего порядка**, чтобы переписать это в лениво оцененную версию

```rust
fn add(x: usize) -> usize {
    // это печатается, поскольку функции оцениваются первыми
    println!("executing add"); 
    return x + x;
}
fn multiply(x: usize) -> usize {
    // это печатается, поскольку функции оцениваются первыми
    println!("executing multiply"); 
    return x * x;
}
type FnType = fn(t: usize) -> usize;

// Теперь это функция высшего порядка, поэтому оценка функций задерживается в if-else
fn add_or_multiply(add: bool, on_add: FnType, on_multiply: FnType, t: usize) -> usize {
    if add {
        on_add(t)
    } else {
        on_multiply(t)
    }
}
fn main() {
    println!("{}", add_or_multiply(true, add, multiply, 4)); // 8
    println!("{}", add_or_multiply(false, add, multiply, 4)); // 16
}
```

Это выводит то, что ниже, и мы можем видеть, что были выполнены только необходимые функции!

```
executing add
8
executing multiply
16
```

Вы также можете использовать методы запоминания / кэширования, чтобы избежать нежелательных оценок в чистых и ссылочных прозрачных функциях, как показано ниже

```rust
use std::collections::HashMap;

fn main() {
    let mut cached_added = HashMap::new();

    let mut add = |x: usize| -> usize {
        return match cached_added.get(&x) {
            Some(&val) => val,
            _ => {
                println!("{}", "executing add");
                let out = x + x;
                cached_added.insert(x, out);
                out
            }
        };
    };

    let mut cached_multiplied = HashMap::new();

    let mut multiply = |x: usize| -> usize {
        return match cached_multiplied.get(&x) {
            Some(&val) => val,
            _ => {
                println!("executing multiply");
                let out = x * x;
                cached_multiplied.insert(x, out);
                out
            }
        };
    };

    fn add_or_multiply(add: bool, on_add: usize, on_multiply: usize) -> usize {
        if add {
            on_add
        } else {
            on_multiply
        }
    }

    println!("{}", add_or_multiply(true, add(4), multiply(4))); // 8
    println!("{}", add_or_multiply(false, add(4), multiply(4))); // 16
}
```

5. Рекурсия

Рекурсия - это процесс, при котором функция вызывает саму себя. В функциональном программировании рекурсия является основным способом повторения операций. Она позволяет создавать более элегантный и краткий код.

Функциональное программирование способствует рекурсии по циклу. Давайте посмотрим пример для вычисления факториала числа.

В традиционном итеративном подходе:

```rust
fn main() {
    fn factorial(mut num: usize) -> usize {
        let mut result = 1;
        while num > 0 {
            result *= num;
            num = num - 1;
        }
        return result;
    }

    println!("{}", factorial(20)); // 2432902008176640000
}
```

То же самое можно сделать с помощью рекурсии:

```rust
fn main() {
    fn factorial(num: usize) -> usize {
        return match num {
            0 => 1,
            _ => num * factorial(num - 1),
        };
    }

    println!("{}", factorial(20)); // 2432902008176640000
}
```

Недостатком рекурсивного подхода является то, что он будет медленнее по сравнению с итеративным подходом в большинстве случаев и может привести к ошибкам переполнения стека, поскольку каждый вызов функции должен быть сохранен как кадр в стек. Избегать этой хвостовой рекурсии предпочтительнее, особенно когда рекурсия выполняется слишком много раз. 

В хвостовой рекурсии рекурсивный вызов - это последнее, что выполняется функцией, и, следовательно, кадр стека функций не должен сохраняться компилятором. Большинство компиляторов могут оптимизировать код хвостовой рекурсии таким же образом, как оптимизируется итеративный код, что позволяет избежать потери производительности. Но, к сожалению, Rust пока не поддерживает это.

**Карринг** (англ. currying) — это приём в функциональном программировании, при котором функция с несколькими аргументами преобразуется в последовательность функций, каждая из которых принимает только один аргумент и возвращает новую функцию, ожидающую следующий аргумент.

Зачем нужен карринг
- Частичное применение функций — можно заранее зафиксировать часть аргументов, а остальные передавать позже.

- Повышение переиспользуемости кода — одна универсальная функция превращается в набор более специализированных.

- Чистота и читаемость — особенно полезно в цепочках функционального стиля.

```rust
fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

let add5 = add(5); // функция, которая прибавляет 5
println!("{}", add5(3)); // 8
```

</details>


---

### **Объектно-ориентированное программирование (ООП)**.

Из-за сложности программ, появилась необходимость в абстрациях. Оперирует понятиями класса и обьекта, а также инкапсуляция (упаковка, делегировании ответственности), наследование (расширение), полиморфизм (повторное использование). Дает гибкость в структурировании программ. Это архитектура приложения, способ моделирования предметной области. При котором используется разделение на сущности по принципу - недопустить возможность ввести сущность в некорректное состояния.

Основные принципы: 
* **Инкапсуляция** — упаковка данных и методов или герметичность данных, скрывающая внутреннее устройство объекта и предоставляющая безопасный интерфейс для его использования. Цель — защита инвариантов и предотвращение некорректных изменений состояния. Суть инкапсуляции - в делегировании ответственности объекту. Таким образом объ­ект получает право управлять своими (и не только) данными удобным для себя способом.
* **Наследование** — механизм, позволяющий одному классу/типу расширять или переопределять функциональность другого. Цель — повторное использование кода и расширяемость.
* **Полиморфизм** — способность использовать единый интерфейс для разных реализаций. Цель — уменьшение дублирования и гибкая подмена поведения в зависимости от конкретного типа.

ООП помогает создавать гибкие, легко расширяемые приложения, следя за корректностью состояний объектов.

Год появления: 1960-е (Simula), активно популяризировано в 1980-х.

**Пример на Python:**

```python
class Animal:
    def speak(self):
        pass

class Dog(Animal):
    def speak(self):
        return "Woof!"

dog = Dog()
print(dog.speak())
```

<details>
<summary>Почему стал популярен подход ООП.</summary>

Так как до ООП использовался процедурный/императивный подход, который не мог справиться с возрастающей сложностью требований к программам по таким причинам:

1. **Управление сложностью** (Managing Complexity):
По мере роста программ, в процедурном подходе возникала проблема "спагетти-кода". Все данные были доступны глобально, и функции могли изменять их в любом месте, что затрудняло отслеживание изменений и предсказание поведения программы.
ООП предложил способ модуляризации кода. Вместо набора функций, оперирующих глобальными данными, появились "объекты", которые инкапсулируют (объединяют) данные и функции, работающие с этими данными. Это делает части программы более независимыми и легче управляемыми.

1. **Повторное использование кода** (Code Reusability):
В процедурном программировании, если вам нужна была похожая функциональность для разных типов данных, часто приходилось дублировать код.
ООП ввел концепцию наследования. Вы можете создать базовый класс с общими характеристиками и поведением, а затем создать производные классы, которые наследуют эти характеристики и могут добавлять свои собственные или изменять унаследованные. Это значительно сокращает дублирование кода и упрощает его поддержку.

1. **Инкапсуляция** (Encapsulation):
В процедурном программировании данные часто были доступны напрямую, что позволяло любой части программы изменять их. Это могло приводить к непредсказуемым ошибкам.
ООП позволяет инкапсулировать данные внутри объекта, делая их доступными только через определенные методы (интерфейс объекта). Это называется упаковкой информации. Таким образом, внутреннее состояние объекта защищено от некорректных изменений извне, что повышает надежность и упрощает отладку.

1. **Реальное моделирование** (Modeling Real-World Entities):
В реальном мире мы мыслим объектами: машина, человек, счет в банке. У каждого объекта есть свои характеристики (данные) и действия, которые он может выполнять (поведение).
ООП позволяет напрямую моделировать эти реальные сущности в коде, делая программу более интуитивной и легкой для понимания. Объект "Машина" может иметь свойства "цвет", "скорость" и методы "завести", "остановить".

1. **Полиморфизм** (Polymorphism):
ООП позволяет объектам разных классов реагировать по-разному на одно и то же сообщение (вызов метода). Например, у вас может быть базовый класс "Фигура" и производные классы "Круг" и "Квадрат". У всех них может быть метод "нарисовать()", но каждый будет реализовывать его по-своему. Это делает код более гибким и расширяемым.

</details>

---

### **Entity-Component System (ECS)**

Критика Кейси Муратори ООП подхода Роберта Мартина в основном направлена на высокопроизводительные системы, где каждая наносекунда на счету (например, игровые движки, низкоуровневые симуляции). Для большинства бизнес-приложений, где производительность не является критическим фактором, преимущества ООП в плане управляемости сложности, модульности и повторного использования кода могут перевешивать его недостатки.

Год появления: 2000-е, популяризирована в разработке игр

Паттерн архитектуры, где объекты (Entities) состоят из компонентов (Components) с данными и систем (Systems), которые управляют поведением.

Основной недостаток ООП, который решается в **ESR (Entity–State–React)**, — это **чрезмерная сложность и негибкость, вызванная жёсткой привязкой поведения к данным** (т.е. к объектам).  

<details>
<summary>...</summary>

Конкретная проблема ООП, решаемая в ESR:

**Жёсткая связь между состоянием и поведением**  

В классическом ООП поведение (методы) жёстко привязано к данным (полям класса), что приводит к:  
1. Раздутым классам – объекты берут на себя слишком много ответственностей.  
2. Сложности изменения поведения – если нужно изменить логику, приходится модифицировать классы или создавать сложные иерархии наследования/композиции.  
3. Проблемам с тестированием – из-за мутаций состояния и скрытых зависимостей.  

**Как ESR решает эту проблему?**  

ESR разделяет:  
- Сущности (Entity) – просто контейнеры для данных (без логики).  
- Состояние (State) – данные, связанные с сущностью.  
- Реакции (React) – отдельные функции, обрабатывающие состояние.  

Это даёт:  
- **Гибкость** – поведение можно менять, не трогая структуру данных.  
- **Простота тестирования** – реакции (чистые или детерминированные функции) легко тестируются.  
- **Меньше boilerplate-кода** – не нужны сложные паттерны вроде Visitor или Strategy.  

Пример сравнения:  

**❌ В ООП (проблема):**  
```java
class User {
    private String name;
    private int balance;

    // Жёстко привязанное поведение
    public void addMoney(int amount) {
        this.balance += amount;
    }
}
```  
Если нужно изменить логику начисления, придётся менять класс `User`.  

**✅ В ESR (решение):**  
```javascript
// Данные
const user = { name: "Alice", balance: 100 };

// Отдельная функция-реакция
function addMoney(user, amount) {
    return { ...user, balance: user.balance + amount };
}
```  

Логика изменяется без модификации структуры `user`.  

**Когда ESR нужен**:
- Для сложных предметных областей, где поведение зависит от многих факторов
- Когда нужно гибко комбинировать компоненты (как в ECS в gamedev)
- Когда поведение должно работать с множеством разных сущностей

**Вывод:** 

ESR устраняет **главный недостаток ООП — жёсткую связь данных и поведения**, предлагая более декомпозированную и гибкую модель. Это особенно полезно в сложных или часто меняющихся системах.
</details>

---

### **Мультипарадигма**

Rust — мультипарадигмальный язык программирования.
Он сочетает императивный стиль с элементами функционального, процедурного подходов и объектной системы (через структуры, методы и трейты).

Функциональная парадигма - последовательный вызов ф-ции с передачей контекста, методы итераторов и векторов 

```rust  
fn main(){   
    let value:[i32;5] = [1,2,3,4,5];
    let res:i32 = value.iter().filter(|&x| x % 2 == 0).map(|v|v*2).sum();
    println!("Функциональная парадигма {}",res);
}
```  

Процедурная парадигма - вызовы ф-ций с сохранением состояния между вызовами, через присваивание.  

Вариант с итераторами:

```rust     
type Filter<'a> = core::iter::Filter<std::slice::IterMut<'a, i32>,for<'r> fn(&'r &mut i32) -> bool>;
fn filter(iter:&mut [i32])->Filter{
    iter.into_iter().filter(|x| **x % 2 == 0)
}
fn map(filter:Filter)->i32{
    filter.map(|v|*v*2).sum()
}
fn main(){
    let mut value:[i32;5] = [1,2,3,4,5];
    let value_filter = filter(&mut value);
    let res:i32 = map(value_filter);
    println!("Процедурная парадигма {:?}",res);  
}   
``` 

Вариант с классическими циклами:

```rust
fn filter(v:&mut Vec<i32>){
    let mut indexes:Vec<usize>=vec![];
    for (pos,value) in v.iter().enumerate(){
        if *value%2!=0{
            indexes.push(pos);
        }
    }
    indexes.sort();
    let mut correct_pos = 0;
    for pos in indexes{
            v.remove(pos-correct_pos);
            correct_pos+=1;
    }
}
fn map(filter:Vec<i32>)->i32{
    let mut res = 0;
    
    for v in filter{
        res+=v*2;
    }
    res
}
fn main(){
    let mut value:Vec<i32> = vec![1,2,3,4,5];
    filter(&mut value);
    let res:i32 = map(value);
    println!("Процедурная парадигма {:?}",res); 
}
``` 

Разница с классическим ООП: нет наследования состояния, но есть композиция и полиморфизм через трейты.

```rust
use oop::Value;
mod oop{
    pub struct Value{
        value:Vec<i32>,
        pub sum:Option<i32>
    }
    impl Value{
        pub fn new(value:Vec<i32>)->Self{
            Self{value:value,sum:None}
        }
        pub fn calculate(&mut self){
            self.sum = Some(self.value.iter().filter(|&x| x%2 ==0).map(|v|v*2).sum())
        }
    }        
}
fn main(){
    let mut value:Value = Value::new(vec![1,2,3,4,5]);
    value.calculate();
    println!("ООП парадигма {:?}",value.sum.unwrap());
}
``` 

---

</details>


# Подходят ли ООП паттерны для Rust?

Rust — это статически типизированный язык системного программирования, сочетающий элементы **функционального**, **процедурного** и **ограниченно ООП-стилей**. Хотя Rust не является классическим ООП-языком (нет наследования, виртуальных методов по умолчанию), он позволяет применять **принципы ООП** через:
- **Инкапсуляцию** (модули `mod`, `pub`-видимость).
- **Полиморфизм** (трейты, generics, `dyn`). В Rust полиморфизм реализуется несколько иначе, чем в классических ООП-языках (например, Java/C++), но основные виды поддерживаются с помощью **трэйтов, обобщённых типов (generics) и системы владения**. 
- **Композицию** (вместо наследования).

Но важнее не "можно ли", а **"нужно ли"** — Rust предлагает свои идиомы, часто более эффективные, чем классические ООП-паттерны.
 
<details>
<summary>...</summary>

## 1. ООП vs Rust: Ключевые отличия

### Наследование → Трейты + Композиция
В Rust **нет наследования реализации**, только:
- **Наследование интерфейсов** через `super-trait`:
  ```rust
  trait Animal: Display {  // Требует реализации Display
      fn speak(&self);
  }
  ```
- **Композиция** (встраивание структур):
  ```rust
  struct Engine { power: u32 }
  struct Car { engine: Engine }  // Вместо наследования
  ```

### Виртуальные методы → `dyn Trait`

Динамическая диспетчеризация возможна, но требует явного указания:

```rust
trait Draw { fn draw(&self); }
impl Draw for Circle { ... }

let shapes: Vec<Box<dyn Draw>> = vec![Box::new(Circle)];  // Явное указание `dyn`
```
 
## 2. Когда ООП-паттерны **не нужны** в Rust

### Стратегия (Strategy)

Вместо отдельного класса стратегии — просто трейт:

```rust
trait SortStrategy { fn sort(&self, data: &mut [i32]); }

struct QuickSort;
impl SortStrategy for QuickSort { ... }

struct Sorter<T: SortStrategy> { strategy: T }  // Статическая диспетчеризация
```

### Декоратор (Decorator)

Композиция + трейты:

```rust
trait Coffee { fn cost(&self) -> f64; }
struct SimpleCoffee;
impl Coffee for SimpleCoffee { ... }

struct MilkDecorator<T: Coffee> { inner: T }  // Обёртка
impl<T: Coffee> Coffee for MilkDecorator<T> {
    fn cost(&self) -> f64 { self.inner.cost() + 0.5 }
}
```
 

## 3. Когда ООП-паттерны **полезны** в Rust

### Строитель (Builder)
Идеален для Rust из-за неизменяемости по умолчанию:

```rust
#[derive(Default)]
struct ConfigBuilder { timeout: Option<u32> }

impl ConfigBuilder {
    fn timeout(mut self, secs: u32) -> Self {
        self.timeout = Some(secs);
        self
    }
}
```

### Посетитель (Visitor)

Реализуется через трейты и `match`:
```rust
trait Visitor {
    fn visit_circle(&self, c: &Circle);
    fn visit_rect(&self, r: &Rect);
}
```
 

## 4. Функциональные альтернативы в Rust

### Итераторы и замыкания

Вместо ООП-шаблонов часто хватает комбинаторов:
```rust
let sum: i32 = (1..=10).filter(|x| x % 2 == 0).sum();  // Декларативный стиль
```

### Алгебраические типы

`enum` + `match` заменяют полиморфизм подтипов:

```rust
enum Shape { Circle(f64), Rect(f64, f64) }

fn area(s: Shape) -> f64 {
    match s {
        Shape::Circle(r) => r * r * PI,
        Shape::Rect(w, h) => w * h,
    }
}
```
 

Вывод: Rust ≠ ООП, но гибок

| Паттерн       | Альтернатива в Rust               | Когда использовать?           |
|---------------|-----------------------------------|-------------------------------|
| Наследование  | Трейты + композиция              | Всегда                        |
| Стратегия     | Generics + трейты                | Если поведение статично       |
| Декоратор     | Композиция + трейты              | Для расширения функциональности |
| Строитель     | Method chaining + `Default`      | Для сложных объектов          |
| Фабрика       | Функции + `impl Trait`           | Когда нужна динамическая генерация |

**Главное правило**: в Rust сначала смотрите на **функциональные возможности** (итераторы, `match`, трейты), и только если они не подходят — применяйте ООП-паттерны в адаптированном виде.
 
---

 
Также благодаря `static dispatch` мы можем вынести потенциальные ошибки использования типов на этап компиляции кода.  
Система типов Rust может превратить многие виды проблем программирования в проблемы `static dispatch`. 
Это одно из самых больших преимуществ при выборе функционального языка, и оно имеет решающее значение для многих гарантий времени компиляции Rust.
В Rust параметр универсального типа создает то, что в функциональных языках называется «**ограничением класса типа**».
Это называется **мономорфизацией**, когда разные типы создаются из полиморфного кода.


## 5. Виды полиморфизма

Паттерны: Null Object, Strategy, Adapter, Decorator, Composite, Proxy, State — все это просто разновидности применения полиморфизма.
  
- **Null Object** — избегание null проверок
- **Strategy** — смена алгоритмов
- **Adapter** — совместимость интерфейсов  
- **Decorator** — динамическое расширение функциональности
- **Composite** — работа с иерархиями
- **Proxy** — контроль доступа и ленивая инициализация
- **State** — изменение поведения объекта


<details>
<summary>1. Null Object — полиморфизм для отсутствия поведения</summary>

```rust
trait Logger {
    fn log(&self, message: &str);
}

// Реальный логгер
struct RealLogger;
impl Logger for RealLogger {
    fn log(&self, message: &str) {
        println!("LOG: {}", message);
    }
}

// Null-объект
struct NullLogger;
impl Logger for NullLogger {
    fn log(&self, _message: &str) {
        // Ничего не делаем
    }
}

// Использование через полиморфизм
fn process(logger: &impl Logger) {
    logger.log("processing...");
}
```

---

</details>


<details>
<summary>2. Strategy — полиморфизм для замены алгоритмов</summary>

```rust
trait SortingStrategy {
    fn sort(&self, data: &mut [i32]);
}

struct QuickSort;
impl SortingStrategy for QuickSort {
    fn sort(&self, data: &mut [i32]) {
        // Быстрая сортировка
    }
}

struct BubbleSort;
impl SortingStrategy for BubbleSort {
    fn sort(&self, data: &mut [i32]) {
        // Пузырьковая сортировка
    }
}

struct Sorter {
    strategy: Box<dyn SortingStrategy>,
}
```

---

</details>


<details>
<summary>3. Adapter — полиморфизм для совместимости интерфейсов</summary>

```rust
// Старый интерфейс
trait OldService {
    fn old_method(&self);
}

// Новый интерфейс  
trait NewService {
    fn new_method(&self);
}

// Адаптер - реализует новый интерфейс через старый
struct Adapter {
    old_service: Box<dyn OldService>,
}

impl NewService for Adapter {
    fn new_method(&self) {
        self.old_service.old_method(); // Адаптация
    }
}
```

---

</details>


<details>
<summary>4. Decorator — полиморфизм для добавления функциональности</summary>

```rust
trait Coffee {
    fn cost(&self) -> f64;
}

struct SimpleCoffee;
impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        2.0
    }
}

struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.5 // Добавляем функциональность
    }
}
```

---

</details>


<details>
<summary>5. Composite — полиморфизм для древовидных структур</summary>

```rust
trait Graphic {
    fn draw(&self);
}

// Лист
struct Circle;
impl Graphic for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}

// Композит - содержит другие Graphic
struct Picture {
    children: Vec<Box<dyn Graphic>>,
}

impl Graphic for Picture {
    fn draw(&self) {
        for child in &self.children {
            child.draw(); // Рекурсивный вызов
        }
    }
}
```

---

</details>


<details>
<summary>6. Proxy — полиморфизм для контроля доступа</summary>

```rust
trait Database {
    fn query(&self, sql: &str) -> Result<(), String>;
}

struct RealDatabase;
impl Database for RealDatabase {
    fn query(&self, sql: &str) -> Result<(), String> {
        println!("Executing: {}", sql);
        Ok(())
    }
}

struct DatabaseProxy {
    database: RealDatabase,
    has_access: bool,
}

impl Database for DatabaseProxy {
    fn query(&self, sql: &str) -> Result<(), String> {
        if !self.has_access {
            return Err("Access denied".to_string());
        }
        self.database.query(sql) // Делегирование
    }
}
```

---

</details>


<details>
<summary>7. State — полиморфизм для изменения поведения</summary>
 
```rust
trait State {
    fn handle(&self, context: &mut Context);
}

struct StateA;
impl State for StateA {
    fn handle(&self, context: &mut Context) {
        println!("State A handling");
        context.set_state(Box::new(StateB));
    }
}

struct StateB; 
impl State for StateB {
    fn handle(&self, context: &mut Context) {
        println!("State B handling");
        context.set_state(Box::new(StateA));
    }
}

struct Context {
    state: Box<dyn State>,
}
```

---

</details>



### 1. Параметрический полиморфизм (Generics)  
**Есть**  
- Rust поддерживает обобщённое программирование через **Generics**.  
- Позволяет писать код, работающий с разными типами.  
- Этот код создаёт monomorphization, компилятор создаёт **отдельную версию функции `max` для каждого типа**, с которым мы используем функцию, что раздувает бинарный файл.

```rust
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

let m1 = max(5, 10);       // Работает с i32
let m2 = max(3.14, 2.71);  // Работает с f64
```


### 2. Полиморфизм подтипов (Subtyping / Inheritance)  
**Нет в классическом виде**  
- В Rust **нет наследования** (как в Java/C++).  
- Вместо этого используется **композиция и трейты**.
- Этот код создаёт monomorphization, компилятор создаёт **отдельную версию функции `animal_sound` для каждого типа**, с которым мы используем функцию, что раздувает бинарный файл.  

**Альтернатива в Rust:**
```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) { println!("Woof!"); }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) { println!("Meow!"); }
}

fn animal_sound(animal: &impl Animal) {
    animal.make_sound();
}

let dog = Dog;
let cat = Cat;
animal_sound(&dog);  // Woof!
animal_sound(&cat);  // Meow!
```
(Это **полиморфизм через трейты**, а не классический subtyping.)


### 3. Ad-hoc полиморфизм (перегрузка функций/операторов)  
**Частично**  
- Rust **не поддерживает перегрузку функций** (как в C++/Java).  
- Но можно **перегружать операторы** через трейты (`std::ops`).  

**Пример перегрузки оператора:**
```rust
use std::ops::Add;

struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };
let p3 = p1 + p2;  // Point { x: 4, y: 6 }
```

### 4. Полиморфизм через приведение (Coercion Polymorphism)  
**Ограниченно**  
- Rust **не допускает неявных преобразований** (в отличие от C++/Java).  
- Но есть **явные приведения** (`as`) и трейты типа `From`/`Into`.  

```rust
let x: i32 = 5;
let y: f64 = x as f64;  // Явное приведение

// Через трейт From
let z: f64 = f64::from(x);
```

### 5. Полиморфизм времени выполнения (Dynamic Dispatch / Runtime Polymorphism)  
**Есть (через `dyn Trait`)**  
- Rust поддерживает динамическую диспетчеризацию через **трейт-объекты (`dyn Trait`)**.  
- Но требует явного указания (`Box<dyn Trait>`, `&dyn Trait`).  
- Динамическая диспетчеризация - есть накладные расходы (обычно 1 индиректный вызов).

**Что происходит**

В `Box<dyn Animal>` хранится: 
- Указатель на данные (Dog или Cat).
- Указатель на vtable (таблицу методов для конкретного типа). 

Вызов `animal.make_sound()` - идёт через виртуальную таблицу vtable.

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog { fn make_sound(&self) { println!("Woof!"); } }

struct Cat;
impl Animal for Cat { fn make_sound(&self) { println!("Meow!"); } }

let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];

for animal in animals {
    animal.make_sound();  // Woof! Meow!
}
```


### 6. Полиморфизм через интерфейсы (Interface Polymorphism)
**Есть (через трейты)**  
- В Rust **нет интерфейсов**, но их роль выполняют **трейты**.  
- Можно реализовывать для разных структур.  
- Этот код создаёт monomorphization, компилятор создаёт **отдельную версию функции `render` для каждого типа**, с которым мы используем функцию, что раздувает бинарный файл.  

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
impl Drawable for Circle { fn draw(&self) { println!("Drawing Circle"); } }

struct Square;
impl Drawable for Square { fn draw(&self) { println!("Drawing Square"); } }

fn render(item: &impl Drawable) {
    item.draw();
}

let circle = Circle;
let square = Square;
render(&circle);  // Drawing Circle
render(&square);  // Drawing Square
```


Вывод: Какие виды полиморфизма есть в Rust? 
| **Вид полиморфизма**          | **Поддержка в Rust** | **Как реализуется?**          |
|-------------------------------|----------------------|-------------------------------|
| **Параметрический (Generics)** | ✅ Да                | `fn foo<T>(x: T)`             |
| **Subtyping (наследование)**   | ❌ Нет               | Нет классов, только трейты    |
| **Ad-hoc (перегрузка)**        | ⚠️ Частично         | Только операторы (`std::ops`) |
| **Приведение типов**           | ⚠️ Ограниченно      | `as`, `From`/`Into`           |
| **Динамический (`dyn`)**       | ✅ Да                | `Box<dyn Trait>`, `&dyn Trait`|
| **Интерфейсный (трейты)**      | ✅ Да                | `trait Drawable { ... }`      |

Что уникально в Rust?
- **Нет наследования** → вместо него **композиция + трейты**.  
- **Нет неявных приведений** → только явные (`as`, `From`/`Into`).  
- **Динамический полиморфизм (`dyn Trait`)** требует явного указания.  
- **Перегрузка функций отсутствует**, но есть перегрузка операторов.  

Rust делает упор на **статический полиморфизм (Generics + трейты)**, но при необходимости позволяет и **динамическую диспетчеризацию (`dyn`)**.

---

</details>

# Rust idioms

Идиомы — это широко используемые стили, рекомендации и шаблоны, в значительной степени согласованные сообществом. Написание идиоматического кода позволяет другим разработчикам лучше понимать, что происходит.

<details>
<summary>Rust idioms</summary>
 
Общая рекомендация, начинать проектировать данные с **ограниченной областью видимости**. Как только элемент crate становится публичным, его нельзя снова сделать приватным, не сломав код, который использует crate, что требует изменение MAJOR версии. Если вы раскрываете внутренние детали реализации структуры данных, предполагаемое будущее изменение для использования более эффективного алгоритма становится критическим изменением. Если вы раскрываете внутренние вспомогательные функции, это неизбежно, что некоторый внешний код станет зависеть от точных деталей этих функций.

## [Rust idioms examples](https://github.com/Jekahome/Patterns/tree/main/idioms):

* ### Type safety (Newtype, Typestate) 
  
  "Безопасный тип" - это способ ограничить функционал программы конкретным типом, 
  что бы компилятор фильтровал некорректные способы использования. 
  Покрывая код "безопасными типами" мы увеличиваем шансы корректного использования.

  -  [Newtype](https://github.com/Jekahome/Patterns/tree/main/idioms/1.Type_safety/1.1.Newtype)

  Предотвращает недопустимое использование данных.
  Завернув обший тип в свою оболочку мы получаем свой собственный тип данных,
  который неполучится спутать с его внутренным типом и подставить в неверное место,
  так же вы можете обеспечить требуемые инварианты для значений типа, ограничив или расширив их.
  Также newtype ближе к предметной области и следовательно лучше документировать и понять его применение.
  Newtype не несет дополнительных накладных расходов во время выполнения (абстракция с нулевой стоимостью),
  но для полноценного использования newtype требуется реализовать трейты (`Clone/Copy,From/Into,AsRef​​/AsMut...`)

  - [Typestate](https://github.com/Jekahome/Patterns/tree/main/idioms/1.Type_safety/1.2.Typestates)

  Подход "состояние типа" позволяет контролировать последовательность смены состояний при 
  выполнении программы еще на этапе компиляции. Мы можем описать поведение типа, 
  какие состояния он может принимать и в какой последовательности.

* ### [Mem replace](https://github.com/Jekahome/Patterns/tree/main/idioms/2.Mem_replace) (hook lifetime)

  Поскольку Rust подразумевает семантику перемещения по умолчанию и довольно строгие правила заимствования, часто возникают ситуации (особенно с большими structs и enums), когда изменение значения на месте или замена значений не могут быть разрешены средством проверки заимствований, что довольно сбивает с толку и приводит к к созданию ненужных клонов (что приводит к избыточным затратам на производительность).

* ### [Bound impl](https://github.com/Jekahome/Patterns/tree/main/idioms/3.Bound_impl) (ограничение поведения)

  Описывайте bond сигнатурой не данные, а поведение т.е. методы.
  Размещение границ признаков на impl блоках, методах и функциях, а не на типах, уменьшает загрязнение границ признаков, уменьшает связанность частей кода и делает общий код более чистым, простым и эргономичным.
  Структуры данных не должны дублировать производные границы признаков.
  Вы должны попытаться максимально поднять границы признаков (особенно в коде библиотеки), поскольку это расширяет возможности использования типа.


* ### [Generic in type out](https://github.com/Jekahome/Patterns/tree/main/idioms/4.Generic_in_type_out) (абстракция аргументов)

  Абстрагируйтесь от конкретного типа входных данных строки, что бы увеличить гибкость API, принимая как можно больше различных типов. Типажи в помощь `AsRef,Borrow,Into,Cow`
  Обратной стороной этой идиомы является то, что компилятор генерирует больше кода из-за мономорфизации, что потенциально приводит к раздуванию кода. Каноническое решение этой проблемы — исключить внутренний метод, содержащий весь код за вычетом универсальных преобразований, и оставить внешний метод в качестве оболочки.

* ### [Exhaustivity](https://github.com/Jekahome/Patterns/tree/main/idioms/5.Exhaustivity) (проверка на полноту вариантов)

  Проверка на полноту вариантов.
  Расширяя кодовую базу уже имеющихся перечислений и структур, есть шанс не учесть новые поля в коде, что может привести к ошибке во время выполнения. Для того чтобы отловить эту ошибку на этапе компиляции следует придерживаться следующих рекомендаций. Использовать полный список вариантов перечислений и пользоваться полями созданной струтурой,а не отдельными ее полями.

* ### [Sealing](https://github.com/Jekahome/Patterns/tree/main/idioms/6.Sealing) (запечатывание реализации)

  Запечатывание в программировании обычно означает, что некоторые API (в основном общедоступные) не могут быть унаследованы, расширены или реализованы за пределами места их определения.
  Основная цель запечатывания признака — это перспективность API. Так как мы уверены в том что только мы используем все имплементации запечатанного трейта то мы можем его менять.
  Sealed трейт — общедоступный трейт, который не может быть реализован вне места его определения ( модуля или крейта , в зависимости от видимости этого трейта).

* ### Конструктор с помощью ф-ции `new` и конструктор по умолчанию `Default`

В Rust нет конструкторов как языковых конструкций. Вместо этого принято использовать ассоциированную функцию new для создания объекта

<details>
<summary>...</summary>

```rust
/// Time in seconds.
///
/// # Example
///
/// ```
/// let s = Second::new(42);
/// assert_eq!(42, s.value());
/// ```
pub struct Second {
    value: u64
}

impl Second {
    // Constructs a new instance of [`Second`].
    // Note this is an associated function - no self.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}
```

**Конструктор по умолчанию**

```rust
/// Time in seconds.
///
/// # Example
///
/// ```
/// let s = Second::default();
/// assert_eq!(0, s.value());
/// ```
// или #[derive(Default)]
pub struct Second {
    value: u64
}

impl Second {
    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Default for Second {
    fn default() -> Self {
        Self { value: 0 }
    }
}

```
</details>

* ### Трейт `Default` в Rust

`Default` — это трейт, который определяет метод `default()`, возвращающий "значение по умолчанию" для типа.
Это полезно, когда мы хотим создать **объект без указания всех значений вручную**.

Rust не умеет абстрагироваться по признаку "у типа есть `new()`", но зато может абстрагироваться по признаку "у типа есть `Default`".

<details>
<summary>...</summary>

Когда полезно:
* Для **генериков** — можно создавать значение типа без знания конкретных полей.
* Для **структур** с множеством полей — не нужно явно инициализировать всё.
* Для **контейнеров** (`Vec`, `String`, `Option`) — у них уже есть свои дефолтные состояния.
* Можно использовать в функциях, например `unwrap_or_default()`.
 
**Пример с авто-derive**

Если все поля структуры реализуют `Default`, можно просто написать:

```rust
#[derive(Default, Debug)]
struct Config {
    // Option — по умолчанию None
    output: Option<String>,
    // Vec — по умолчанию пустой вектор
    search_path: Vec<String>,
    // bool — по умолчанию false
    check: bool,
}

fn main() {
    // Создание с дефолтными значениями
    let mut conf = Config::default();
    println!("{conf:?}");

    // Частичная инициализация
    let conf2 = Config {
        check: true,
        ..Default::default()
    };
    println!("{conf2:?}");
}
```

**Реализация вручную**

Если поля не имеют `Default`, можно реализовать самому:

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    let p = Point::default();
    println!("({}, {})", p.x, p.y);
}
```

**Использование с generics**

`Default` полезен, когда функция не знает конкретный тип:

```rust
fn make_default<T: Default>() -> T {
    T::default()
}

fn main() {
    let v: Vec<i32> = make_default(); // пустой вектор
    let n: i32 = make_default();      // 0
    println!("{v:?}, {n}");
}
```

Плюсы:
* Не нужно вручную инициализировать все поля.
* Можно легко создавать дефолтные значения в generics.
* Удобно для тестов и шаблонов настроек.

Минусы:
* Только один `Default` на тип (нельзя сделать несколько "разных дефолтов").
* Не подходит, если нужна сложная логика инициализации с параметрами.

</details>


* ### Динамическая диспетчеризация на стеке

Динамическая диспетчеризация на стеке — это способ эффективно и без аллокации, работать с разными типами, реализующими один интерфейс, без необходимости создавать дополнительные структуры.

В отличие от дженериков, где компилятор создаёт отдельный код для каждого типа (статическая диспетчеризация), динамическая диспетчеризация на стеке позволяет писать единый код.

Это снижает размер бинарника и ускоряет компиляцию.

<details>
<summary>...</summary>

Чтобы увеличить время жизни объекта при необходимости, можно использовать отложенную условную инициализацию, как показано ниже:

```rust
use std::io;
use std::fs;

// Каждая переменная содержит значения только одного типа. 
// В нашем примере stdin имеет тип Stdin, 
// file имеет тип File и readable имеет тип &mut dyn Read
let (mut stdin_read, mut file_read);

let readable: &mut dyn io::Read = if arg == "-" {
    stdin_read = io::stdin();
    &mut stdin_read
} else {
    file_read = fs::File::open(arg)?;
    &mut file_read
};
```

**Преимущества**

Нам не нужно ничего размещать в куче. 
Нам также не нужно инициализировать что-то, что мы не будем использовать позже, и нам не  нужно мономорфизировать весь следующий код для работы с обоими File или Stdin.

Версия аналогичного кода но с выделением памяти в куче:

```rust
let readable: Box<dyn io::Read> = if arg == "-" {
    Box::new(io::stdin())
} else {
    Box::new(fs::File::open(arg)?)
};
```
</details>

* ### Передача переменных в замыкание 

(используйте перепривязку переменных в отдельной области)

По умолчанию замыкания захватывают свое окружение путем заимствования. 
Или вы можете использовать move-closure для перемещения всей среды. 
Однако часто требуется переместить в замыкание только некоторые переменные, передать ему копию некоторых данных, передать их по ссылке или выполнить какое-либо другое преобразование.

Для этого используйте перепривязку переменных в отдельной области.

<details>
<summary>...</summary>

```rust
use std::rc::Rc;

let num1 = Rc::new(1);
let num2 = Rc::new(2);
let num3 = Rc::new(3);

// Хорошо
let closure = {
    // `num1` is moved
    let num2 = num2.clone();  // `num2` is cloned
    let num3 = num3.as_ref();  // `num3` is borrowed
    move || {
        *num1 + *num2 + *num3;
    }
};

// Плохо
let num2_cloned = num2.clone();
let num3_borrowed = num3.as_ref();
let closure = move || {
    *num1 + *num2_cloned + *num3_borrowed;
};

```

**Преимущества**

Скопированные данные группируются вместе с определением замыкания, поэтому их назначение становится более ясным, и они будут немедленно удалены, даже если они не будут использованы замыканием.

Замыкание использует те же имена переменных, что и окружающий код, независимо от того, копируются или перемещаются данные.

</details>

* ### Временная мутабельность

(заменить мутабельную переменню на не мутабельную путем переопределения переменной)

Часто необходимо подготовить и обработать некоторые данные, но после этого данные только проверяются и никогда не изменяются. Намерение можно сделать явным, переопределив изменяемую переменную как неизменяемую.

Это можно сделать либо путем обработки данных внутри вложенного блока, либо путем переопределения переменной.

<details>
<summary>...</summary>


Скажем, вектор перед использованием необходимо отсортировать.

**Использование вложенного блока:**
```rust
let data = {
    let mut data = get_vec();
    data.sort();
    data
};

// Here `data` is immutable.
```

**Использование перепривязки переменных:**
```rust
let mut data = get_vec();
data.sort();
let data = data;

// Here `data` is immutable.
```
</details>

* ### Возвращать использованный аргумент при ошибке

Если функция потребляет (перемещает) аргумент, верните этот аргумент обратно внутри ошибки.
Что бы не засталять пользователей клонировать эти данные в случае необходимости использовать как-то повторно эти данные.

<details>
<summary>...</summary>

```rust
pub struct SendError(String);

pub fn send(value: String) -> Result<(), SendError> {
    println!("using {value} in a meaningful way");
    // Simulate non-deterministic fallible action.
    use std::time::SystemTime;
    let period = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    if period.subsec_nanos() % 2 == 1 {
        Ok(())
    } else {
        Err(SendError(value))
    }
}
```
</details>

* ### Используйте заимствованные (borrow) типы для аргументов


**Коэрция аргументов (Argument Coercion) в Rust**
 
**Коэрция аргументов** — это механизм, благодаря которому компилятор Rust автоматически преобразует переданный тип в ожидаемый тип функции, когда это возможно. Обычно это происходит с помощью `Deref`-коэрции, позволяющей, например, ссылке на `String` (`&String`) автоматически приводиться к `&str`.
 
Например, `&String` может автоматически превратиться в `&str`, `&Vec<T>` в `&[T]`, а `&Box<T>` в `&T`.

Но преобразование в обратную сторону не всегда возможно (например, `&str → &String` не сработает).

Поэтому, когда вы пишете функции, всегда лучше принимать заимствованный тип (`&str` вместо `&String`, `&[T]` вместо `&Vec<T>`, `&T` вместо `&Box<T>`)

<details>
<summary>...</summary>


Зачем это нужно:

* **Повышает гибкость API** — функция сможет принимать больше видов входных данных (литералы строк, срезы, строки в куче и т. д.).
* **Снижает количество уровней косвенных ссылок** — например, `String` уже хранит данные в куче (слой косвенности), а `&String` добавляет ещё один слой. При `&str` этого лишнего слоя нет.
* **Нет лишних аллокаций** — преобразование `String → &str` дешёвое (не требует клонирования/выделения памяти), а `&str → &String` требует выделения памяти.

Итоговое правило:
* Принимайте минимально необходимый тип, который описывает ваши требования.
* Если нужна только возможность чтения строки — берите `&str`, а не `&String`.
* То же самое с `Vec<T>` к (`&[T]`) и `Box<T>` к (`&T`).

**Пример использования аргумента ф-ции как `&str` или `String`**:

```rust
// Если нужна только ссылка (чтение)
// Используйте AsRef<str> — функция будет принимать всё, 
// что можно превратить в &str (String, &String, &str, Arc<str>, и т. д.):
fn take_a_str(some: impl AsRef<str>) { // или fn take_a_str<S: AsRef<str>>(some: S){...
    let some = some.as_ref();
    println!("{some}");
}

// Если нужно владение (String внутри функции)
// Используйте Into<String> — функция примет и String, и &str (второе будет скопировано):
use core::fmt::Debug;
fn take_a_str_into(some: impl Into<String>+Debug){//или fn take_a_str_into<S: Into<String>+Debug>(some: S){..
    println!("{:?}",some);
}
fn main() {
    take_a_str("str");
    take_a_str("String".to_string());
    
    // also `&String` is supported:
    let string_ref = "StringRef".to_string();
    take_a_str(&string_ref);

    take_a_str_into("str");
    take_a_str_into("String".to_string());
    let string_ref = "StringRef".to_string();
    take_a_str_into(&string_ref);
}
```

**Обобщенный вариант для заимствования `&Vec<T>` и `&[T]`**

```rust
fn process<S, T>(data: S)
where
    S: AsRef<[T]>,
    T: std::fmt::Debug,
{
    for item in data.as_ref() {
        println!("{:?}", item);
    }
}

fn main() {
    let v = vec![1, 2, 3];
    let arr = [4, 5, 6];

    process(&v);   // &Vec<i32>
    process(&arr); // &[i32]
}
```

**Обобщенный вариант для владения (чтобы функция принимала и потребляла данные, а не только заимствовала)**

```rust
fn process<S, T>(data: S)
where
    S: Into<Vec<T>>,
    T: std::fmt::Debug,
{
    let owned_vec: Vec<T> = data.into();
    for item in owned_vec {
        println!("{:?}", item);
    }
}

fn main() {
    let v = vec![1, 2, 3];
    let arr = [4, 5, 6];
    let boxed: Box<[i32]> = vec![7, 8, 9].into_boxed_slice();

    process(v);       // Vec<i32> — перемещаем
    process(arr);     // [i32; N] — копирует элементы
    process(boxed);   // Box<[i32]> — без копирования
}
```
</details>

* ### Объединение строк с помощью `format!`

<details>
<summary>...</summary>

Строки можно создавать: 
- используя методы `push` и `push_str` для изменяемого объекта `String`. Это наиболее эффективно но неудобно.  
  ```rust
    let mut result = "Hello ".to_owned();
    result.push_str(name);
    result.push('!');
    ```
- или с помощью оператора + у `String`
  ```rust
    let hello = String::from("Hello");
    let world = String::from("world");
    let rust = " in Rust";

    let sentence = hello + " " + &world + rust;
  ```

- однако часто удобнее использовать [`format!`](https://doc.rust-lang.org/std/macro.format.html)
  ```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s:String = format!("{}-{s2}-{s4}", s1, s4=s3); // tic-tac-toe
  ```
</details>

* ### Коллекции — это умные указатели

В Rust коллекции, такие как `Vec<T>` или `String`, **владеют** своими данными, а срезы (`&[T]` или `&str`) — это **заимствованные** (borrowed) представления этих данных.

Чтобы удобно работать с коллекциями и их заимствованными версиями, в Rust используется трейт `Deref`. Он позволяет автоматически преобразовывать ссылку на коллекцию в ссылку на срез — это называется **авто-доступом через deref**.

<details>
<summary>...</summary>

Пример с `Vec` и `Deref`:

```rust
use std::ops::Deref;

struct Vec<T> {
    data: RawVec<T>,
    // ...
}

impl<T> Deref for Vec<T> {
    type Target = [T]; // целевой тип — срез

    fn deref(&self) -> &[T] {
        // возвращаем ссылку на внутренний срез
    }
}
```

* `Vec<T>` — владеет данными
* `&[T]` — это срез (borrowed view) на те же данные

Благодаря `Deref` можно вызывать методы срезов на `&Vec<T>` напрямую, например:

```rust
let v = vec![1, 2, 3];
let slice: &[i32] = &v; // благодаря Deref это работает
println!("Длина: {}", slice.len()); // метод среза вызван через Vec
```

Аналогично для `String` и `&str`:

* `String` — владеет строкой
* `&str` — срез строки (borrowed view)

Реализация `Deref<Target=str>` позволяет использовать методы `&str` напрямую на `&String`.


Зачем это нужно?

* **Удобство API**: большинство методов реализованы для заимствованных типов (`&[T]`, `&str`), и благодаря `Deref` мы можем использовать эти методы и на владеющих типах (`Vec<T>`, `String`).
* **Гибкость**: позволяет работать либо с владением данных, либо с их заимствованием, не переписывая код.
* **Избежание дублирования кода**: методы пишутся один раз для срезов, но работают и для коллекций.

Преимущества:
* Можно писать методы только для заимствованных типов (срезов), и они автоматически доступны для коллекций.
* Клиенты API могут выбирать — владеть данными или просто заимствовать их.

Недостатки:
* При дженериках и ограничениях по трейтам методы, доступные только через `Deref`, могут не учитываться при проверке ограничений.
* Это усложняет некоторую обобщённую программную логику (есть трейты `Borrow` и `AsRef` для решения таких задач).


Итог:

Коллекции в Rust — это как умные указатели, которые владеют данными, а трейт `Deref` позволяет обращаться к этим данным через удобные заимствованные представления. Это одна из мощных особенностей Rust, делающая API гибким и удобным без лишних накладных расходов.

</details>

* ### Финализация в деструкторах

В Rust **нет** `finally` как в JavaScript, Java или Python — блока кода, который всегда выполняется при выходе из функции (вне зависимости от того, вернулись мы через `return`, `?` или случился `panic!`).

Вместо этого можно использовать **деструкторы** (`Drop`) — код, который выполняется, когда объект выходит из области видимости (**scope**).

<details>
<summary>...</summary>

```rust
fn baz() -> Result<(), ()> {
    // Здесь могла быть логика, но просто вернем ошибку
    Err(())
}

fn bar() -> Result<(), ()> {
    struct Finalizer;

    impl Drop for Finalizer {
        fn drop(&mut self) {
            println!("Функция bar завершена!");
        }
    }

    // Создаем объект, деструктор которого выполнится при выходе из функции
    let _finalizer = Finalizer;

    // Если baz() вернет Err, `?` сделает ранний выход — деструктор всё равно вызовется
    baz()?;

    println!("Завершаем bar() обычным образом");
    Ok(())
}
// Даже если `baz()` вернет ошибку и мы выйдем через `?`, деструктор сработает.
fn main() {
    let _ = bar();
    // output: Функция bar завершена!
}
```

Почему это полезно:

* Позволяет **гарантированно** (почти) выполнить очистку или логирование при **любом** выходе из функции.
* Удобно, если внутри много `return` или используется `?`.
* Можно применять для освобождения ресурсов: закрытия файлов, сокетов, снятия блокировок (mutex).

 
Ограничения:

* **Не 100% гарантия** — деструктор **не сработает**, если:

  * Программа застряла в бесконечном цикле.
  * Произошел `panic!` **во время** уже выполняющегося `panic!` (double panic).
  * Произошло аварийное завершение процесса.
* Деструктор не должен сам вызывать `panic!` — это приведет к аварийному завершению потока.
* Чтобы деструктор вызвался в конце функции, объект нужно хранить **в переменной**, а не создать и сразу выбросить.


**RAII (Resource Acquisition Is Initialization)**

Этот подход — частный случай RAII-паттерна:
* Ресурсы (файлы, блокировки, соединения) **захватываются** при создании объекта.
* Ресурсы **освобождаются** в `Drop`.

Пример с мьютексом: мьютекс разблокируется в деструкторе `MutexGuard`, даже если произойдет `panic!` в середине блока.

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let data_clone = Arc::clone(&data);
    thread::spawn(move || {
        let mut num = data_clone.lock().unwrap(); // lock захвачен
        *num += 1;
        // lock освободится автоматически при выходе из области видимости
    }).join().unwrap();

    println!("data = {:?}", *data.lock().unwrap());
}
```

**Перехват panic для освобождения ресурсов или записи логов**

В Rust освобождение ресурсов основано на **RAII** (Resource Acquisition Is Initialization): как только объект выходит из области видимости, вызывается его `Drop`.
При `panic!` поведение зависит от профиля компиляции и настройки `panic`:


```toml
Cargo.toml:
[profile.dev]
panic = "unwind"

[profile.release]
panic = 'abort' 
```

* **`panic = "unwind"`** (по умолчанию в `dev`):

  * Происходит **разворачивание стека** (stack unwinding).
  * Rust проходит по каждому фрейму стека в обратном порядке вызова.
  * Для всех живых объектов вызывается их `Drop`.
  * Можно перехватить панику с помощью `std::panic::catch_unwind`.

* **`panic = "abort"`** (по умолчанию в `release` в некоторых проектах):

  * Программа немедленно завершается.
  * **`Drop` не вызывается**.
  * Никакой очистки ресурсов не происходит.

Когда это полезно:
* Закрытие файлов или сетевых соединений.
* Освобождение памяти, выделенной не через Rust (например, через FFI).
* Освобождение мьютексов (чтобы избежать deadlock).
* Откат изменений в системах, где важно оставить корректное состояние.

```rust
use std::fs::File;
use std::io::{self, Write};

struct TempFile {
    path: String,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        println!("Удаляю временный файл: {}", self.path);
        let _ = std::fs::remove_file(&self.path);
    }
}

fn do_work() -> io::Result<()> {
    let _temp = TempFile { path: "temp.txt".into() };
    let mut file = File::create("temp.txt")?;
    writeln!(file, "Важные данные")?;

    // Симуляция ошибки
    panic!("Что-то пошло не так!");

    // _temp удалится даже при панике (только при panic = "unwind")
}

fn main() {
    let result = std::panic::catch_unwind(|| {
        do_work().unwrap();
    });

    if result.is_err() {
        println!("Поймали панику, ресурсы освобождены");
    }
}
```

Как сделать надёжно:

1. **Если ресурс критичен** — используй `catch_unwind` вокруг кода, чтобы гарантировать выполнение `Drop` даже при панике.
2. **В production-сборках**:

   * Если стоит `panic = "abort"`, освобождение ресурсов через `Drop` не сработает.
   * Для действительно критичных операций (закрытие транзакций в БД, сохранение данных) стоит явно вызывать очистку **до** паники.
3. Не паникуй внутри `Drop` — при панике во время `unwind` Rust **немедленно завершит программу**.


Сводка:

| Настройка `panic` | Drop вызывается? | Можно перехватить `panic`? |
| ----------------- | ---------------- | -------------------------- |
| `"unwind"`        | ✅ Да             | ✅ `catch_unwind`           |
| `"abort"`         | ❌ Нет            | ❌                          |

---

</details>


* ### `mem::{take(_), replace(_)}` для сохранения собственных значений в измененных enum'ов


В Rust мы **не можем просто так "забрать"** значение поля из `&mut`-ссылки на enum,
потому что:

* Компилятор требует, чтобы в поле *всегда* что-то лежало.
* Нельзя оставить "дырку" вместо значения.

<details>
<summary>...</summary>

```rust
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn bad(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        let taken_name = *name; // ❌ ошибка: нельзя "забрать" из &mut
    }
}
```


**Решение: заменить, а потом забрать**

Rust предлагает два стандартных способа:

**`mem::take`**

* Заменяет значение **на его `Default`** (например, `String` → пустая строка).
* Возвращает старое значение.

```rust
use std::mem;

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: mem::take(name), // заменили на пустую строку и забрали старую
        };
    }
}
```

Для `String` пустая строка не аллоцирует память — поэтому это быстрый способ.



**`mem::replace`**
* Заменяет значение на **то, что вы явно указали**.
* Не требует `Default`.

```rust
use std::mem;

fn a_to_b_with_replace(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: mem::replace(name, String::from("default_name")),
        };
    }
}
```

**Более сложный пример: перестановка вариантов**

```rust
use std::mem;

enum MultiEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}

fn swizzle(e: &mut MultiEnum) {
    use MultiEnum::*;
    *e = match e {
        A { name } => B { name: mem::take(name) },
        B { name } => A { name: mem::take(name) },
        C => D,
        D => C,
    };
}
```

Здесь `mem::take` позволяет переставить значения между вариантами **без `.clone()` и аллокаций**.


Когда использовать
* **Изменить enum на месте**, сохранив какое-то поле в новом варианте.
* Избежать лишнего `.clone()` и аллокаций.
* Работать с типами, у которых есть `Default` (для `take`) или вручную указать замену (для `replace`).


| Метод          | Чем заменяет?             | Требует `Default`? |
| -------------- | ------------------------- | ------------------ |
| `mem::take`    | `T::default()`            | ✅ Да               |
| `mem::replace` | Значение, переданное вами | ❌ Нет              |

</details>


* ### [Идиомы FFI](https://rust-unofficial.github.io/patterns/idioms/ffi/intro.html)

* ### Итерация по Option

В Rust `Option<T>` можно рассматривать как **контейнер, в котором либо 0, либо 1 элемент**.
Поскольку `Option` реализует трейты **`IntoIterator`** и **`Iterator` для ссылок**, с ним можно работать так же, как с векторами или другими коллекциями.

<details>
<summary>...</summary>

**`Option` в `.extend()`**

Если у вас есть коллекция, вы можете добавить в неё элемент из `Option` без `if let`:

```rust
let turing = Some("Turing");
let mut logicians = vec!["Curry", "Kleene", "Markov"];

// Добавим элемент, если он есть
logicians.extend(turing);

// Эквивалентно:
if let Some(inner) = turing {
    logicians.push(inner);
}
```

**`Option` в `.chain()`**

Если хотите добавить элемент в конец существующего итератора:

```rust
let turing = Some("Turing");
let logicians = vec!["Curry", "Kleene", "Markov"];

// Склеиваем итератор вектора с итератором Option
for name in logicians.iter().chain(turing.iter()) {
    println!("{name} is a logician");
}
```

Если элемент **всегда есть**, лучше использовать `std::iter::once`:

```rust
for name in logicians.iter().chain(std::iter::once(&"Turing")) {
    println!("{name} is a logician");
}
```

**`for` по `Option`**

Можно итерироваться по `Option` напрямую в цикле `for`:

```rust
let opt = Some("Rust");

for value in opt {
    println!("Value: {value}");
}

// Эквивалентно:
if let Some(value) = opt {
    println!("Value: {value}");
}
```

Хотя так можно, **`if let` обычно читается лучше**.

**Преобразование `Option` в итератор вручную**
* `option.into_iter()` — забирает значение из `Option`, потребляя её.
* `option.iter()` — даёт итератор по ссылке (`&T`).
* `option.iter_mut()` — даёт итератор по изменяемой ссылке (`&mut T`).


```rust
let mut opt = Some(10);

for x in opt.iter_mut() {
    *x += 5;
}

assert_eq!(opt, Some(15));
```
 
Дополнительно:
* **`std::iter::once`** — создаёт итератор с ровно одним элементом (удобно, если точно знаете, что он есть).
* **`Iterator::filter_map`** — фильтрует и преобразует элементы в `Option`, убирая `None`.
* **crate `ref_slice`** — умеет превращать `Option<&T>` в слайс длиной 0 или 1.

</details>

* ### [non_exhaustive] и приватные поля для расширяемости

Существует небольшой набор сценариев, в которых автор библиотеки может захотеть добавить открытые поля в открытую структуру или новые варианты в перечисление, не нарушая при этом обратную совместимость.

Rust предлагает два решения этой проблемы:
- Используется #[non_exhaustive]для structs, enums и enumвариантов. Подробную документацию по всем местам #[non_exhaustive]использования см. в документации .
- Вы можете добавить частное поле к структуре, чтобы предотвратить ее прямое создание или сопоставление с ней

<details>
<summary>...</summary>

Представьте, что ыы написали библиотеку с таким enum:

```rust
pub enum Shape {
    Circle(f64),
    Square(f64),
}
```

Кто-то использует ваш код и пишет:

```rust
match shape {
    Shape::Circle(r) => println!("radius {r}"),
    Shape::Square(s) => println!("side {s}"),
}
```

Если завтра вы добавите `Shape::Triangle(f64)`, этот код **сломается** при компиляции (не все варианты покрыты).

Но иногда вам **нужно** иметь возможность добавлять варианты/поля без мажорного релиза.
Вот тут и помогает `#[non_exhaustive]`

1. Для **enum**

```rust
#[non_exhaustive]
pub enum Shape {
    Circle(f64),
    Square(f64),
}
```

Теперь пользователь **обязан** писать `_` или `..` в `match`, чтобы обработать будущее расширение:

```rust
match shape {
    Shape::Circle(r) => println!("circle {r}"),
    Shape::Square(s) => println!("square {s}"),
    _ => println!("something new!"),
}
```

Пользователь не сможет исчерпывающе перечислить все варианты — это гарантия, что вы сможете добавить новые.

2. Для **struct**

```rust
#[non_exhaustive]
pub struct Config {
    pub name: String,
}
```

Теперь нельзя будет создать его напрямую:

```rust
// ❌ Ошибка — конструктор недоступен
let cfg = Config { name: "hi".into() };
```

Придётся использовать `..`:

```rust
let Config { name, .. } = cfg;
```

Это гарантирует, что при добавлении новых полей старый код не сломается.



**Альтернатива внутри одного крейта — приватное поле**

Если `#[non_exhaustive]` нужен только **внутри** текущего крейта, можно просто добавить приватное поле:

```rust
pub struct Config {
    pub name: String,
    _private: (),
}
```

Теперь:
* Пользователь **не может** распаковать все поля без `..`
* Не может сам создать `Config`
* Вы можете безопасно добавлять поля


Когда это уместно

✅ Хорошо:
* API, моделирующие **внешние ресурсы**, которые могут измениться (например, HTTP-ответы или JSON-схемы)
* Когда **важно** не ломать обратную совместимость и не выпускать major-версию

Осторожно:
* Пользователи будут вынуждены писать обработку "неизвестных" случаев, часто с `panic!()` или заглушками
* Это может сделать код менее удобным

Иногда проще просто выпустить **новую major-версию**, чем усложнять API `#[non_exhaustive]`.

---

</details>

* ### Простая инициализация примера в документации

В доках к методам вам нужно показать рабочий пример,
но для этого нужно **много кода инициализации**, который не относится к сути примера.

<details>
<summary>...</summary>

Например, чтобы вызвать метод `send_request`, вам приходится в каждой доке повторять:

```rust
struct Connection {
    name: String,
    stream: TcpStream,
}

impl Connection {
    /// Sends a request over the connection.
    ///
    /// # Example
    /// ```no_run
    /// # // Boilerplate are required to get an example working.
    /// # let stream = TcpStream::connect("127.0.0.1:34254");
    /// # let connection = Connection { name: "foo".to_owned(), stream };
    /// # let request = Request::new("RequestId", RequestType::Get, "payload");
    /// let response = connection.send_request(request);
    /// assert!(response.is_ok());
    /// ```
    fn send_request(&self, request: Request) -> Result<Status, SendErr> {
        // ...
    }

    /// Oh no, all that boilerplate needs to be repeated here!
    fn check_status(&self) -> Status {
        // ...
    }
}
```

Это:
* Делает пример **длинным**
* Отвлекает читателя от основной идеи метода
* Усложняет сопровождение (меняется инициализация — переписывай везде)


**Решение — обёрточная функция**

Вместо того, чтобы повторять код, выносим всю инициализацию в функцию-хелпер, а в примерах просто показываем вызов метода.

````rust
/// # Example
/// ```
/// # fn call_send(connection: Connection, request: Request) {
/// let response = connection.send_request(request);
/// assert!(response.is_ok());
/// # }
/// ```
fn send_request(&self, request: Request) { /* ... */ }
````

Тут `call_send` — это **не вызываемая** функция, которая скрывает весь подготовительный код.
Rustdoc проверит, что он компилируется, но тестировать его не будет, так как мы его не вызываем.


Плюсы
* Примеры становятся **короче** и чище.
* Нет копипаста — меняем инициализацию только в одном месте.
* Компиляция всё ещё проверяется (`cargo test` без запуска).

Минусы
* Тесты внутри такой функции не выполняются.
* Если нужно именно проверить поведение (assert), надо либо вызывать функцию в тестах, либо использовать другой подход.

 
**Альтернатива для тестируемых примеров**

Если нужно, чтобы код **выполнялся** при `cargo test`, можно:

* сделать `#[doc(hidden)] pub fn test_connection() -> Connection` внутри библиотеки;
* использовать его в доках как часть публичного API.

Пример:

```rust
#[doc(hidden)]
pub fn test_connection() -> Connection {
    Connection {
        name: "foo".into(),
        stream: TcpStream::connect("127.0.0.1:34254").unwrap(),
    }
}
```

И в доках:

````rust
/// ```
/// let conn = mycrate::test_connection();
/// let status = conn.check_status();
/// assert_eq!(status, Status::Ok);
/// ```
````

</details>


---

[Rust idioms rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms)

[Rust idioms rust-unofficial](https://rust-unofficial.github.io/patterns/idioms/index.html)

[Rust idioms rust-unofficial github](https://github.com/Jekahome/RustDesignPatterns/tree/main/src/idioms)

[Реализация конечного автомата (переходы между состояниями)](https://hoverbear.org/blog/rust-state-machine-pattern/)

[Typestate шаблон Состояние](https://doc.rust-lang.ru/book/ch17-03-oo-design-patterns.html)

---

</details>

# Anti patterns

Есть три большие области, в которых выделяют антипаттерны: разработка, архитектура и управление проектами.

Паттерны - это методы решения распространенных проблем при кодировании. Однако, хотя шаблоны проектирования дают нам преимущества, антишаблоны создают больше проблем.

Так же антипаттерны в части [code-smells](https://github.com/Jekahome/Patterns/tree/main?tab=readme-ov-file#code-smells).

Антипаттерн возникает без рефакторинга.
Рефакторинг программного обеспечения — это форма модификации кода, используемая для улучшения структуры программного обеспечения для поддержки последующего расширения и долгосрочного обслуживания. В большинстве случаев цель состоит в том, чтобы преобразовать код, не влияя на его корректность.

Разработка программного обеспечения на основе архитектуры — наиболее эффективный подход к созданию систем. 
Подходы, основанные на архитектуре, превосходят подходы, основанные на требованиях, документах и ​​методологии. Проекты часто достигают успеха вопреки методологии, а не благодаря ей.
 

<details>
<summary>...</summary>

 
#### Антипаттерны в Rust 
 
- Клонирование для проверки заимствований (borrow checker)

<details>
<summary>...</summary>

Если вы используете `.clone()` что-бы обойти правила заимствования Rust, то это анти-паттерн.
В общем, клонирование должно быть осознанным, с полным пониманием последствий. 

```
// define any variable
let mut x = 5;

// Borrow `x` -- but clone it first
let y = &mut (x.clone());

// without the x.clone() two lines prior, this line would fail on compile as
// x has been borrowed
// thanks to x.clone(), x was never borrowed, and this line will run.
println!("{}", x);

// perform some action on the borrow to prevent rust from optimizing this
//out of existence
*y += 1;

```

Rust предоставляет типажи `Rc,Arc` для случаев когда нам нужно несколько «владеющих» указателей на одни и те же данные.

Несмотря на то, что `.clone()` указывает на плохой шаблон, иногда можно написать неэффективный код, например, в следующих случаях:

- developer все еще является новым владельцем;

- код не имеет больших ограничений по скорости или памяти (как проекты хакатонов или прототипы);

- выполнить проверку заимствований действительно сложно, и вы предпочитаете оптимизировать читаемость, а не производительность;

Также обязательно всегда запускайте свой проект c `cargo clippy`, который обнаружит некоторые случаи, в которых нет необходимости в `.clone()`
 
---

</details>

- Использование подавление предупреждений компилятора при сборке

<details>
<summary>...</summary>

Использование `#![deny(warnings)]` является анти-паттерном, так как ведет к потенциальным ошибкам в будущем.

Иногда новые функции или старые недостатки требуют изменения в том, как что-то делается, поэтому `lints` записываются `warn` на определенный льготный период, прежде чем их переведут на `deny`.

Кроме того, иногда API устаревают, поэтому при их использовании будет выдаваться предупреждение там, где раньше его не было.

**Альтернативы**

Есть два способа решения этой проблемы: во-первых, мы можем отделить настройку сборки от кода, а во-вторых, мы можем явно назвать тесты, которые хотим запретить.

Следующая командная строка будет построена со всеми предупреждениями, установленными на deny:

`RUSTFLAGS="-D warnings" cargo build`

Это может сделать любой отдельный разработчик (или настроить в инструменте CI, таком как Travis, но помните, что это может привести к поломке сборки, если что-то изменится) без необходимости внесения изменений в код.

Альтернативно, мы можем указать нужные нам линты denyв коде. Вот список предупреждений, которые (надеюсь) можно безопасно отклонить (начиная с Rustc 1.48.0):

```
#![deny(bad_style,
       const_err,
       dead_code,
       improper_ctypes,
       non_shorthand_field_patterns,
       no_mangle_generic_items,
       overflowing_literals,
       path_statements,
       patterns_in_fns_without_body,
       private_in_public,
       unconditional_recursion,
       unused,
       unused_allocation,
       unused_comparisons,
       unused_parens,
       while_true)]
```
Кроме того, может быть хорошей идеей запретить следующие разрешенные линты:

```
#![deny(missing_debug_implementations,
       missing_docs,
       trivial_casts,
       trivial_numeric_casts,
       unused_extern_crates,
       unused_import_braces,
       unused_qualifications,
       unused_results)]
```
Некоторые, возможно, также захотят дополнить `missing-copy-implementations` в свой список.

Список линтов в вашей системе `rustc -W help`  

Общий список опций `rustc --help` 

[rust-clippy](https://github.com/Manishearth/rust-clippy) — это набор тестов для улучшения кода Rust

[Коллекция всех клипповых ворсинок](https://rust-lang.github.io/rust-clippy/master)
 
---

</details>

- Неправильное использование трейта `Deref`

<details>
<summary>...</summary>

Неправильно используйте трейт `Deref` для эмуляции наследования между структурами.

**Пример**
Иногда нам нужно эмулировать следующий общий шаблон объектно-ориентированных языков, таких как Java:
```java
class Foo {
    void m() { ... }
}

class Bar extends Foo {}

public static void main(String[] args) {
    Bar b = new Bar();
    b.m();
}

```

Для этого мы можем использовать антишаблон полиморфизма deref:
```rust
use std::ops::Deref;

struct Foo {}

impl Foo {
    fn m(&self) {
        //..
    }
}

struct Bar {
    f: Foo,
}

impl Deref for Bar {
    type Target = Foo;
    fn deref(&self) -> &Foo {
        &self.f
    }
}

fn main() {
    let b = Bar { f: Foo {} };
    b.m();
}

```
В Rust нет наследования структур. Вместо этого мы используем композицию и включаем экземпляр Foo в Bar.

```rust
impl Bar {
    fn m(&self) {
        self.f.m()
    }
}
```

**Недостатки**

Будущие программисты, читающие это в коде, не будут ожидать, что это произойдет.
Это потому, что мы неправильно используем `Deref`. Признак Deref предназначен для реализации пользовательских типов указателей. Цель состоит в том, чтобы он принимал указатель на T, а не выполнял преобразование между различными типами.

Это еще и потому, что механизм здесь совершенно неявный.

Наконец, этот шаблон поддерживает только одиночное наследование и не имеет понятия об интерфейсах, конфиденциальности на основе классов или других функциях, связанных с наследованием. Таким образом, это дает опыт, который будет слегка удивить программистов, привыкших к наследованию Java и т. д.

**Обсуждение**

Нет ни одной хорошей альтернативы. В зависимости от конкретных обстоятельств может быть лучше повторно реализовать использование признаков или написать фасадные методы для ручной диспетчирезации Foo. 
 
---

</details>

### Антипаттерны разработки ПО

Это ошибки на уровне **кода и реализации**, которые ухудшают читаемость, тестируемость и поддержку.
 
<details>
<summary>...</summary>  

* **1. Spaghetti Code (Спагетти-код)**
Спагетти-код - слабо структурированная и плохо спроектированная система, запутанная и очень сложная для понимания.
Код с множеством goto, копипастой, затрудняет расширение и оптимизацию кода. 
Пример из Linux SCSI driver.

Решение — частый рефакторинг может улучшить структуру и поддержку.
 
* **2. God Object / Blob (Божественный объект / Блоб)**
Гигантский класс, который делает всё и становится почти операционной системой. За таким "блобом" обычно скрываются десятки других плохих практик. Часто встречается в легаси-системах (например, банковских).

Такой объект берет на себя слишком много функций и/или хранит в себе практически все данные. В итоге мы имеем непереносимый код, в котором, к тому же, сложно разобраться. Так же, подобный код довольно сложно поддерживать, учитывая, что вся система зависит практически только от него. Причинами являются — некомпетентность разработчика, взятие одним разработчиком большой части работы (особенно, когда размер работы «превышает» уровень опыта этого разработчика).
 
* **3. Copy-Paste Programming / Копипаста**
Повторное использование программного кода методом вырезания и вставки, без переиспользования. Приводит к серьёзным проблемам сопровождения когда ошибка в коде скопированна в разных его частях, и программист забывает вносить требуемые изменения в скопированный код. Может быть как экономией времени, так и источником проблем.

Решение — создание общих решений и их использование. 
 
* **4. Premature Optimization (Преждевременная оптимизация)**
Оптимизация до того, как подтверждена реальная необходимость, ухудшающая читаемость и усложняющая поддержку.
 
* **5. Lava Flow (Поток лавы)**
Код копируется из проекта в проект с минимальными изменениями, с большим количеством закомментированных или устаревших функций и вообще недокументированы, или такому коду сопутствует комментарий вида "Не знаю, как оно работает, но оно работает. Не удалять и не менять!". В итоге образуются огромные блоки мёртвого кода, который боятся удалить. Особенно распространён в аутсорсинговых компаниях. Главными причинами возникновения потоков лавы являются — написание больших частей проекта одним программистом, отсутствие code review, ошибки в проектировании архитектуры.
 
* **6. Golden Hammer (Золотой молоток)**
Золотой молоток — уверенность в полной универсальности любого решения. На практике, это — применение одного решения (чаще всего какого-либо одного паттерна проектирования) для всех возможных и невозможных задач. 
Использование одной знакомой технологии или подхода для решения всех задач. Например, предпочтение IBM MQ вместо Apache Kafka. Золотой молоток — это уверенность в универсальности любого решения, что на практике приводит к неэффективным результатам.

Решение — для каждой задачи имеется не одно, а несколько, красивых и оптимальных решений — именно к поиску таких решений и сводится эффективная разработка. И только такая разработка позволит создать эффективную систему.
 
* **7. Boat Anchor (Лодочный якорь)**
Наследование большого количества бесполезного кода или зависимостей, которые не служат полезной цели и мешают развитию. Часто возникает после слияний компаний.

Этот анти-паттерн означает сохранение неиспользуемых частей системы, которые остались после оптимизации или рефакторинга. Часто, после рефакторинга когда, который является результатом анти-паттерна, некоторые части кода остаются в системе, хотя они уже больше не используются. Так же некоторые части кода могут быть оставлены «на будущее», авось придётся ещё их использовать. Такой код только усложняет системы, не неся абсолютно никакой практической ценности. 
 
* **8. Magic Numbers (Магические числа)**
Необъяснённые числовые значения без использования констант и комментариев. Числа затрудняют понимание кода и его рефакторинг. Главными причинами этой ошибки — спешка при разработке, отсутствие практики программирования. Данный анти-паттерн надо пресекать на корню, оговаривая использование числовых констант перед началом разработки.
 
* **9. Shotgun Surgery (Разбросанные изменения)**
При изменении логики нужно вносить правки в десятки разрозненных мест кода.
  
* **10. Poltergeists (Полтергейсты)**
Классы с очень коротким жизненным циклом, содержащие минимум данных и логики, существующие только для вызова методов других классов. Нарушают принципы ООП.
Пример: класс DatabaseConnector, который только создаётся, открывает соединение, вызывает метод и тут же умирает. Логичнее интегрировать функционал в основной класс.
 
* **11. Dead End (Тупик)**
Использование классов или API, которые не могут быть расширены или модифицированы, не предоставляют нужной функциональности и ведут архитектуру в тупик. Например, проприетарная библиотека, не поддерживаемая разработчиком.
 
* **12. Input Kludge (Кладж ввода)**
Костыльная обработка входных данных: система принимает данные в неудобном формате, логика парсинга разбросана по всему коду, нет чётких интерфейсов.
Пример: парсинг CSV в 15 разных местах по разным правилам.
 
* **13. Blind Faith (Слепая вера)**
Слепое доверие внешним API без проверки ответов, входным данным без валидации, сторонним библиотекам без понимания их работы.
Пример: использование результата API без проверки статуса ответа.

Этот анти-паттерн — недостаточная проверка корректности входных данных, исправления ошибки или результатов работы кода. Очень часто программист думает, что его код всегда будет в идеальных условиях, никогда не выдаст ошибки и не получит неверных входных данных или, ещё чего, данных неверного типа. Но все лгут, поэтому нельзя доверять никакому коду, даже собственному. Но и не следует доводить это недоверие до паранойи, то есть приходить к анти-паттерну ненужной сложности. Просто следует помнить про проверку входных данных и возможные проблемы у чужого кода, который используете вы.
 
* **14. Soft Code / Hard Code (Мягкое / Жёсткое кодирование)**
Отсутствие чёткого разделения, что должно быть настраиваемым, а что — жёстко заданным в коде. Жёсткое кодирование — внедрение различных данных об окружении в реализацию. Например — различные пути к файлам, имена процессов, устройств и так далее. Захардкодить — жёстко прописать значение каких-либо данных в коде. Главная опасность, исходящая от этого анти-паттерна — непереносимость. В системе разработчика код будет исправно работать до перемещения или переименования файлов, изменения конфигурации устройств. На любой другой системе код может вовсе не заработать сразу же. Как правило, программист практически сразу забывает где и что он захардкодил, даже если делает это в целях отладки кода.

Мягкое кодирование — параноидальная боязнь жёсткого кодирования. Это приводит к тому, что незахардкожено и настраивается абсолютно всё, что делает конфигурацию невероятно сложной и непрозрачной. Этот анти-паттерн является вторым концом палки о жёстком кодировании и поэтому тоже является опасным. Во-первых, при разработке много ресурсов уходит на реализацию возможности настроек абсолютно всего. Во-вторых, развёртывание такой системы повлечет так же дополнительные затраты. Перед началом решения определённой задачи следует определить, что должно быть настариваемым, а что является постоянным для различных систем или может быть настроено автоматически.
 
* **15. Functional Decomposition (Функциональная декомпозиция)**
Разделение классов не по функциональному уровню, а по частям бизнес-процесса. Приводит к смешиванию уровней (например, фронт в бэке, запросы к веб-сервисам внутри функций сложения чисел). Возникает из-за попыток реализовать процедурные подходы на ООП-языках.

* **16. Needless Complexity (Избыточная сложность)**
Код содержит ненужные усложнения, которые делают его трудным для понимания и поддержки. Простыми словами — это заумность решения. Ненужная сложность может быть внесена в решение любой задачи. Это приводит к усложнению понимания кода, понижению скорости работы. Причинами являются — отсутствие или некачественность рефакторинга, некомпетентность программиста.

* **17. Programming by permutation (Программирование перебором)**
Многие начинающие программисты пытаются решать некоторые задачи методом перебора — не брутфорсом решения, а именно подбором параметров, порядка вызова функций и так далее. Все эти игры с "А если i+1? Ну тогда может i-1?" к параметрам и подобные штучки устраняют только симптомы, и не дают понимания сути происходящего. А если программист не понимает происходящего, то он не сможет предусмотреть все варианты развития событий и обязательно о чём-то забудет. Он потратит время на подбор работающего для него решения и позднее потратит время для переделки этого решения. Все подобные подобранные решения вылазят боком и хорошо ещё — если в процессе разработки или отладки. К подобному ни в коем случае нельзя привыкать, достигая успеха на небольших задачках. Если программист не может решать задачи другим путём — он некомпетентен и ему не следует доверять разработку — вам же будет хуже.

* **18. Бездумное комментирование**
Результат «работы» данного анти-паттерна — большое количество лишних и неинформативных комментариев. Код не следует комментировать ради комментирования!
Ни в коем случае нельзя допускать диалога разработчиков в комментариях — лучше перенести данную функцию с комментариев на специализированные инструменты для code review, или на личное обсуждение.

</details>
 
### Антипаттерны архитектуры ПО

Ошибки на уровне **структуры системы** и **взаимодействия компонентов**.

<details>
<summary>...</summary>  

* **1. Big Ball of Mud (Большой ком грязи)**
Архитектура без чёткой структуры, всё перепутано и хаотично. Система сложна для понимания, поддержки и расширения.
 
* **2. Stovepipe System / Stovepipe Enterprise (Система дымоходов / Предприятие по производству дымоходов)**
Изолированные подсистемы без унифицированных интерфейсов, слабо связанные и плохо интегрированные, что мешает развитию и масштабированию системы. Отсутствие координации и планирования приводит к фрагментированным и трудноуправляемым архитектурам.
 
* **3. Reinventing the Wheel (Изобретение велосипеда заново)**
Написание собственных решений вместо использования готовых стандартных библиотек или фреймворков, что приводит к излишним затратам времени и ресурсов.

Смысл этого анти-паттерна в том, что программист разрабатывает собственное решение для задачи, для которой уже существуют решения, очень часто лучшие чем придуманное программистом. Разработчик считает себя наилучшим, поэтому для каждой задачи пытается придумать собственное решение, не смотря на опыт его предшественников. Чаще всего это приводит лишь к потере времени и понижению эффективности работы программиста — так как решение может быть найдено далеко неоптимальное или вообще ненайденное. Полностью же отбрасывать возможность самостоятельного решения нельзя, так как это прямой дорогой к приведет к программированию копипастом. Разработчик должен ориентироваться в задачах, которые могут предстать перед ним, чтобы грамотно их решить — используя готовые решение или изобретая собственные. Очень часто причиной этого анти-паттерна является банальная нехватка времени.
 
* **4. Vendor Lock-In (Привязка к поставщику)**
Полная зависимость от одного поставщика технологий или проприетарных архитектур, что затрудняет замену и обновление компонентов.
 
* **5. Jenga Architecture (Архитектура Дженга)**
Система, где любое изменение может сломать всё остальное, подобно игре в Дженгу. Отсутствие модульности и слабое разделение ответственности.
 
* **6. Anemic Domain Model (Анемичная модель)**
Объекты без логики, всё поведение вынесено в сервисы, что превращает модель в набор данных без бизнес-правил. Противоречит принципам ООП.

   [Анемичная модель (Anemic Domain Model)](https://www.martinfowler.com/bliki/AnemicDomainModel.html)  

    Предпочитайте нормальную модель бизнес логики (иногда называют Rich Domain Model), а не процедурную, анемичную модель с DTO вместо полноценных сущностей. Анемичная модель не имеет ничего общего с ООП и ее следует рассматривать как неудачный пример процедурного программирования. 

    Суть антипаттерна:
    * В **предметной области** (domain) объекты содержат **только данные** (поля) и **не содержат поведения** (методов с бизнес-логикой).
    * Вся логика вынесена в отдельные **сервисы**, которые оперируют «голыми» DTO.
    * Получается, что объекты системы не умеют ничего, кроме как хранить состояние.

    Симптомы:
    * Сущности выглядят как Java/Rust/С# классы/структуры с кучей `pub` или `getters/setters` и **ни одного метода с бизнес-логикой**.
    * Бизнес-правила сосредоточены в сервисах/утилитах, а не в самих объектах.
    * Чтобы изменить объект, всегда нужно звать отдельный сервис, который «знает, как с ним работать».
    * Код похож на процедурный, даже если он формально написан на ООП-языке.

    Почему это плохо:
    * Нарушает **принцип инкапсуляции** — данные и логика работы с ними разорваны.
    * Сложнее поддерживать код — изменения бизнес-правил требуют менять множество сервисов.
    * Усложняет тестирование — тесты нужно писать для сервисов, а не для самих объектов.
    * Порождает **god services** — сервисы, которые делают всё и знают всё.
    * Модель перестаёт быть самоописательной — бизнес-логика «размазана» по коду.

    Как правильно (Rich Domain Model).
    В **богатой модели** сущность сама:
    * знает свои инварианты,
    * выполняет бизнес-логику,
    * изменяет своё состояние **только через свои методы**.

    ```rust
    struct BankAccount {
        balance: i32,
    }

    impl BankAccount {
        fn deposit(&mut self, amount: i32) {
            assert!(amount > 0, "amount must be positive");
            self.balance += amount;
        }

        fn withdraw(&mut self, amount: i32) -> Result<(), &'static str> {
            if amount > self.balance {
                Err("Insufficient funds")
            } else {
                self.balance -= amount;
                Ok(())
            }
        }
    }
    ```

    Анемичный вариант выглядел бы так:

    ```rust
    struct BankAccount {
        balance: i32,
    }

    // Логика вынесена в отдельный сервис
    fn withdraw(account: &mut BankAccount, amount: i32) -> Result<(), &'static str> { ... }
    ```

    Если упростить, **анемичная модель** = «ООП без ООП» → процедурная архитектура под видом объектной.
    В DDD её считают особенно вредной, потому что она ломает ключевую идею — объединение данных и поведения в доменной модели.


* **7. Architecture by Implication (Архитектура по смыслу / случайная архитектура)**
Архитектура, которая возникла случайно, без явного планирования и документирования, что приводит к неустойчивой и непредсказуемой системе.
 
* **8. The Blob (Блоб)**
Центральный модуль, через который проходит вся логика системы, что создаёт сильную связанность и узкое место для всех изменений.
 
* **9. Connector Bloat (Избыточность интерфейсов)**
Чрезмерно сложные или многочисленные интерфейсы и точки интеграции, что затрудняет сопровождение и развитие системы.
 
* **10. Technology Obsolescence (Технология непрерывного устаревания)**
Использование устаревших технологий без осознанного выбора, что затрудняет поддержку и развитие системы. Быстрые изменения в экосистеме приводят к сложностям совместимости.
 
* **11. Architectural Smells (Архитектурные запахи)**
Общие признаки плохой архитектуры, включающие смешение слоёв, дублирование, чрезмерную связанность и др.
  
* **12. Swiss Army Knife (Швейцарский армейский нож)**
Слишком сложный и универсальный интерфейс или модуль, пытающийся удовлетворить все возможные потребности, что приводит к громоздкости и плохой поддерживаемости.
 
* **13. Wolf Ticket**
Продукт или компонент, который заявляет об открытости и соответствии стандартам, но на деле не соответствует им и имеет собственные интерфейсы.
 
* **14. Architecture by Committee (Проектирование комитетом)**
Архитектура, созданная большим количеством участников без чёткой ответственности, в результате чего получается сложная и неэффективная структура.
 
* **15. Bloatware / Feature Creep (Перегрузка функционалом)**
Накопление избыточного функционала, который усложняет архитектуру и снижает её гибкость.
 
* **16. Anti-integration (Антиинтеграция / Подсистемы дымоходной системы)**
Интеграция подсистем производится точечно и не стандартизирована, что затрудняет развитие системы в целом.
</details>

### Антипаттерны управления проектами

Ошибки в **менеджменте, процессах и взаимодействии команды**, которые приводят к снижению эффективности, демотивации и провалам проектов.

<details>
<summary>...</summary>

* **1. Death March (Смертельный марш)**
Проект с заведомо нереалистичными сроками, требованиями и ресурсами, где команда вынуждена работать в условиях постоянного кризиса и переработок.

* **2. Scope Creep / Featuritis / Feature Creep (Расползание области проекта / Перегрузка фичами)**
Постоянное добавление новых функций без пересмотра сроков и ресурсов, что ведёт к хаосу, росту стоимости и срыву дедлайнов.

* **3. Micromanagement (Микроменеджмент)**
Чрезмерный контроль и вмешательство менеджеров в мелкие детали работы разработчиков, что снижает их мотивацию и продуктивность.

* **4. Analysis Paralysis (Паралич анализа)**
Бесконечный сбор и анализ информации без принятия решений и перехода к реализации, что тормозит развитие проекта.

* **5. Throw It Over the Wall (Передать работу без обратной связи)**
Передача задач или готового продукта другой команде или отделу без должного взаимодействия и обратной связи, что вызывает недопонимания и ошибки.

* **6. Management by Numbers (Управление по цифрам)**
Фокус только на метриках и числах, игнорируя качество, человеческий фактор и контекст, что приводит к ложным выводам и неправильным решениям.

* **7. Smoke and Mirrors (Дым и зеркала)**
Демонстрация фальшивого прогресса, подделка отчетов и показных демонстраций вместо реального выполнения работы.

* **8. The Mythical Man-Month (Мифический человеко-месяц)**
Вера, что добавление большего числа разработчиков всегда ускоряет проект, что часто приводит к обратному эффекту из-за необходимости коммуникации и обучения.

* **9. Warm Bodies (Теплые тела)**
Назначение в проект большого количества людей без учёта их квалификации и вклада, ради достижения численности, что снижает эффективность.

* **10. Walking on Minefield (Прогулка по минному полю)**
Работа с технологиями или процессами, которые содержат множество скрытых рисков и проблем, приводящих к частым ошибкам и сбоям.

* **11. Mushroom Management (Грибовидное управление)**
Изоляция разработчиков от конечных пользователей и важных решений, при этом им даётся мало информации, как будто их «растят в темноте и кормят навозом».

* **12. Cover Your Assets (Прикрывайте свои активы)**
Создание чрезмерной документации и формальных требований с целью избежать ответственности и принятия решений, что тормозит процесс.

* **13. Committee Design (Проектирование комитетом)**
Решения принимаются группой без чёткой ответственности, что приводит к раздутым и непоследовательным архитектурам и решениям.

* **14. Blind Faith (Слепая вера)**
Менеджеры и команда слепо доверяют инструментам, процессам или внешним факторам без критического анализа и проверки.

* **15. Over-Commitment (Переобязательство)**
Обещание больше, чем реально возможно сделать, что приводит к постоянным срывам сроков и снижению качества.

* **16. Lack of Feedback (Отсутствие обратной связи)**
Отсутствие регулярной коммуникации между участниками проекта, что ведёт к накоплению ошибок и недопониманиям.

---

</details>

[Что такое анти-паттерны?](https://habr.com/ru/articles/59005/)

[Худшие практики разработки и архитектуры](https://habr.com/ru/companies/gazprombank/articles/742618/)
  
---

</details>

# Design principles: SOLID, KISS, DRY, YAGNI, GRASP, LoD, SoC, SLA


## SOLID

Принципы SOLID — это набор из пяти принципов проектирования, введенных [Робертом Сесилом Мартином (Дядя Боб)](http://cleancoder.com/products), [cleancoders.com](https://cleancoders.com/), призванных прояснить изначальные ограничения объектно-ориентированного программирования и сделать программы более гибкими и адаптируемыми.
<details>
<summary>...</summary>

### Принцип единой ответственности (SRP - Single Responsibility Principle)

Принцип единой ответственности (**SRP**): класс должен иметь только одну ответственность, то есть только изменения в одной части спецификации программного обеспечения должны иметь возможность повлиять на спецификацию класса.

SRP — это про “одну причину для изменения”, а не буквально “одну ответственность” в бытовом смысле.

Роберт Мартин (Uncle Bob) формулировал SRP так:

> «У модуля должна быть только одна причина для изменения».
> Это значит, что ответственность измеряется источником изменений, а не количеством функций в классе.

«Соберите вместе вещи, которые изменяются по одним и тем же причинам. Разделите те вещи, которые изменяются по разным причинам»

Это очень похоже на [SoC](https://github.com/Jekahome/Patterns?tab=readme-ov-file#soc), не правда ли? Разница между этими двумя принципами в том, что SRP - локальный принцип, обычно применяемый к классам, функциям, модулям, в то время как SoC — стратегический принцип, который работает на всех уровнях архитектуры: от микроскопического (функции) до макроскопического (сервисы, модули, слои).

Принцип единой ответственности обладает всеми преимуществами SoC, в частности, он способствует высокой связности и низкой связанности, а также позволяет избежать анти-шаблона [«божественного объекта (God object)»](https://github.com/Jekahome/Patterns?tab=readme-ov-file#%D0%B0%D0%BD%D1%82%D0%B8%D0%BF%D0%B0%D1%82%D1%82%D0%B5%D1%80%D0%BD%D1%8B-%D1%80%D0%B0%D0%B7%D1%80%D0%B0%D0%B1%D0%BE%D1%82%D0%BA%D0%B8-%D0%BF%D0%BE), который нарушает SRP, так как в нём сосредоточено слишком много разнотипных причин для изменения (UI, бизнес-логика, доступ к данным и т.д.).

---

### Принцип открытости/закрытости (Open-Closed Principle **OCP**)

OCP = Расширяем через добавление, а не через изменение.

Мы проектируем код так, чтобы его поведение можно было менять, не переписывая существующие строки.
То есть «закрыто» — не значит, что код вообще не трогаем, а что интерфейс стабилен и изменения в нём не ломают существующую работу клиентов.

<details>
<summary>...</summary>

**В функциональном программировании** — через высокоуровневые функции, передающие поведение в аргументах.

❌ Чтобы добавить новый способ обработки, нужно лезть внутрь `process_list` — нарушаем OCP:
```rust
fn process_list(nums: Vec<i32>, mode: &str) -> Vec<i32> {
    match mode {
        "square" => nums.iter().map(|x| x * x).collect(),
        "double" => nums.iter().map(|x| x * 2).collect(),
        _ => nums,
    }
}
```

✅ Теперь мы можем расширять поведение просто передавая новую функцию. Мы не меняем `process_list`, а расширяем её возможности:

```rust
fn process_list<F>(nums: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    nums.into_iter().map(f).collect()
}
let doubled = process_list(vec![1, 2, 3], |x| x * 2);
let squared = process_list(vec![1, 2, 3], |x| x * x);
```

**В композиционной архитектуре** — через DI (dependency injection), плагины, стратегии, обработчики событий. Поведение расширяется подключением новых модулей, а не изменением старых.

- Мы не «зашиваем» конкретную реализацию в класс.
- Класс зависит от абстракции, а не от конкретного типа.
- Чтобы расширить функциональность, мы просто создаём новую реализацию и подставляем её — менять код самого класса не нужно.
- Если DI нет, то мы почти всегда приходим к «лестнице if-else» или match с перечислением всех вариантов, и тогда любое новое поведение = модификация старого кода → нарушение OCP.

✅ Нужен новый способ оплаты? Просто создаём новый тип, реализующий `Payment`. И `OrderService` остаётся неизменным.
```rust
trait Payment {
    fn pay(&self, amount: f64);
}

struct OrderService<T: Payment> {
    payment: T,
}

impl<T: Payment> OrderService<T> {
    fn checkout(&self, amount: f64) {
        self.payment.pay(amount);
    }
}
```

**В микросервисах** — через расширяемые API. API проектируется так, чтобы его можно было расширять без поломки клиентов.

Например, у вас есть `REST API`:
```code
GET /orders/{id}
```
Вы добавляете новый эндпоин:
```code
GET /orders/{id}/status
```
или новое поле в JSON:
```json
{
  "id": 1,
  "status": "shipped",
  "tracking": "AB123456789"
}

```
✅ Клиенты, которые не знают про tracking, продолжают работать. API «закрыт» для поломок, но «открыт» для расширений.


Представьте себе классическое наследование ООП: вы создали родительский класс, а затем позже расширили его дочерним классом с дополнительной функциональностью. Затем по какой-то причине вы решили изменить внутреннюю структуру родительского класса (например, добавить новое поле или удалить какой-то метод), которая также доступна или напрямую влияет на производный класс. Делая это, вы нарушаете этот принцип, потому что теперь вам не только нужно изменить родительский класс, но и адаптировать дочерний класс для новых изменений. Это происходит из-за того, что сокрытие информации не применяется должным образом.  

Клиент должен зависеть от абстракций (интерфейсов, абстрактных классов, публичных контрактов), а не от деталей реализации.
Тогда код клиента «закрыт» от изменений, но «открыт» к новым возможностям, которые реализуют ту же абстракцию.

</details>

---

### Принцип замены Liskov (LSP - Liskov Substitution Principle)

«Объекты в программе должны быть заменены экземплярами их подтипов без изменения корректности этой программы».

<details>
<summary>...</summary>

**Принцип подстановки Лисков (LSP)**  

**Формальное определение:**  
> *«Если `S` — подтип `T`, то объекты типа `T` могут быть заменены объектами типа `S` без изменения корректности программы»*.  

**Суть принципа:**  
Подтипы должны быть **полностью взаимозаменяемы** с базовыми типами. Это означает, что код, работающий с типом `T`, должен продолжать работать корректно, если вместо `T` подставить его подтип `S`.  

 

**Ключевые концепции**  
1. **Подтип vs Подкласс**  
   - *Подтип* — это тип, совместимый с другим типом **по поведению** (например, `u32` — подтип `i64` в некоторых контекстах).  
   - *Подкласс* — это структурное наследование (актуально для ООП, но не для LSP в общем случае).  

2. **Контракт типа**  
   Подтип обязан соблюдать **контракт** базового типа:  
   - Те же **предусловия** (требования к входным данным).  
   - Те же или более строгие **постусловия** (гарантии результата).  
   - Те же **инварианты** (условия, истинные в любое время).  



**Правила LSP для типов**  
1. **Совместимость сигнатур**  
   - Методы подтипа должны принимать **те же или более широкие** входные типы.  
   - Возвращаемые значения должны быть **те же или более узкие**.  

2. **Исключения**  
   - Подтип не должен вводить **новые исключения**, которые не описаны у базового типа.  

3. **Инварианты**  
   - Подтип не должен нарушать **условия**, которые гарантирует базовый тип.  
   - Пример: если базовый тип требует `value > 0`, подтип не может допускать `value = 0`.  

4. **История (опционально)**  
   - Подтип не должен изменять **состояние системы** неожиданным для базового типа образом.  


**Пример нарушения LSP**  

Допустим, у нас есть:  
- Базовый тип `Транспорт` с методом `изменить_скорость(delta: i32)`.  
- Подтип `ГоночныйТранспорт`, который ускоряется в 2 раза быстрее: `изменить_скорость(delta: i32 * 2)`.  

**Проблема:**  
Код, ожидающий `Транспорт`, может сломаться, если получит `ГоночныйТранспорт`, потому что:  
- Ожидается линейное изменение скорости, но подтип даёт нелинейное.  
- Это **нарушает контракт** базового типа.  

**Как исправить:**  
Сделать `ГоночныйТранспорт` отдельным типом **без наследования** или переопределить контракт явно.  

 

**LSP в языках без ООП (например, Rust)** 
 
В Rust подтипизация реализуется через **трейты** и **типы-подтипы** (например, `&dyn Trait`).  

**Пример корректного LSP:**  
```rust
trait Transport {
    fn change_speed(&mut self, delta: i32);
}

struct Car;
impl Transport for Car {
    fn change_speed(&mut self, delta: i32) { ... } // Линейное изменение
}

struct RacingCar;
impl Transport for RacingCar {
    fn change_speed(&mut self, delta: i32) { ... } // Также линейное (контракт сохранён)
}

// Функция работает с любым `impl Transport`
fn test_drive(transport: &mut impl Transport) {
    transport.change_speed(10);
}

let mut car = Car;
let mut racer = RacingCar;
test_drive(&mut car);   // OK
test_drive(&mut racer); // OK, если контракт не нарушен
```  

**Нарушение LSP в Rust:**  
Если `RacingCar::change_speed` будет умножать `delta` на 2, это нарушит ожидания `test_drive`.  
 

**Итог**  
- **LSP — про поведенческую совместимость типов**, а не про наследование.  
- **Подтип** должен быть **прозрачной заменой** базового типа.  
- **Нарушение LSP** приводит к неожиданным ошибкам при подстановке типов.  
- **В Rust** принцип применяется через трейты и полиморфизм (`impl Trait`, `dyn Trait`).  

**Главное правило:**  
> *«Код, работающий с базовым типом, не должен знать, что ему подменили подтип»*.
 
</details>

---

### Принцип разделения интерфейсов (ISP - Interface Segregation Principle)

«Интерфейс отвечающий за одну область задач лучше, чем один интерфейс общего назначения».

Любой код не должен зависеть от методов, которые ему не нужны. Если клиент не использует какое-то поведение объекта, почему он должен быть вынужден зависеть от него? Аналогично, если клиент не использует какие-то методы, почему реализатор должен быть вынужден предоставлять эту функциональность?

Разбейте «толстые» интерфейсы на более конкретные. Если вы измените конкретный интерфейс, эти изменения не повлияют на не связанных с ним клиентов.

---

### Принцип инверсии зависимостей (DIP - Dependency Inversion Principle) 

1. Нужно «зависеть от абстракций, а не от конкретики».

2. Реализация «подключается» к абстракции, а не наоборот.

Вместо того чтобы высокоуровневый модуль (логика приложения) напрямую использовал низкоуровневый модуль (конкретная реализация), оба работают через абстракцию (интерфейс, протокол, контракт).

<details>
<summary>...</summary>

Инверсия зависимости состоит из двух основных утверждений:
1. Модули высокого уровня не должны зависеть от модулей низкого уровня. Оба должны зависеть от абстракций
2. Абстракции не должны зависеть от деталей. Детали должны зависеть от абстракций.

В Rust DIP чаще реализуют через:
- `trait` как абстракцию;
- `generic-параметры` с трейт-ограничениями (`<T: Logger>`);
- или динамическое распределение (`Box<dyn Logger>`), если нужна замена в runtime.


❌ `UserService` жёстко зависит от конкретного `FileLogger`.
Если мы захотим писать в БД или в консоль, придётся менять код `UserService` → нарушаем `OCP`
```rust
struct FileLogger;

impl FileLogger {
    fn log(&self, message: &str) {
        println!("Log to file: {message}");
    }
}

struct UserService {
    logger: FileLogger,
}

impl UserService {
    fn new() -> Self {
        Self { logger: FileLogger }
    }

    fn register_user(&self, name: &str) {
        self.logger.log(&format!("User registered: {name}"));
    }
}

fn main() {
    let service = UserService::new();
    service.register_user("Alice");
}
```

✅ Абстракция (`trait Logger`) — `UserService` зависит только от интерфейса. Реализации (`FileLogger, ConsoleLogger`) можно менять и добавлять без изменения `UserService → OCP` соблюдён. Инверсия зависимостей — теперь не `UserService` создаёт `FileLogger`, а внешний код внедряет зависимость.
```rust
// Абстракция (контракт)
trait Logger {
    fn log(&self, message: &str);
}

// Одна реализация
struct FileLogger;
impl Logger for FileLogger {
    fn log(&self, message: &str) {
        println!("Log to file: {message}");
    }
}

// Другая реализация
struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("Console log: {message}");
    }
}

// Высокоуровневый модуль зависит от абстракции
struct UserService<'a, L: Logger> {
    logger: &'a L,
}

impl<'a, L: Logger> UserService<'a, L> {
    fn new(logger: &'a L) -> Self {
        Self { logger }
    }

    fn register_user(&self, name: &str) {
        self.logger.log(&format!("User registered: {name}"));
    }
}

fn main() {
    let file_logger = FileLogger;
    let console_logger = ConsoleLogger;

    let service1 = UserService::new(&file_logger);
    service1.register_user("Alice");

    let service2 = UserService::new(&console_logger);
    service2.register_user("Bob");
}

```

### Основные моменты критики SOLID
SOLID полезен как чек-лист и набор рекомендаций, но не как строгий закон.

Проблема не в самих принципах, а в буквализме: следование им без учёта контекста и масштаба проекта. Хорошие разработчики используют SOLID избирательно, совмещая его с KISS, YAGNI и здравым смыслом.

1. Избыточность и усложнение. Следование принципам «в чистом виде» часто ведёт к созданию слишком большого количества абстракций, интерфейсов, классов и прослоек. Это нарушает [KISS](https://github.com/Jekahome/Patterns?tab=readme-ov-file#kiss) и иногда [YAGNI](https://github.com/Jekahome/Patterns?tab=readme-ov-file#yagni).
2. Не всегда применимо в функциональном или процедурном стиле. В Rust `DIP` часто реализуется просто через `generics` и `traits`, без выдумывания слоёв `DI-контейнеров`.
3. `S` - Single Responsibility Principle слишком расплывчат. «Одна ответственность» можно трактовать по-разному — модуль может менять код по сотне причин, но все они будут «логически одной зоной ответственности» (или наоборот — одну функцию можно раздробить на пять классов).
4. `O` - Open/Closed Principle сложно соблюдать в реальности. Часто проще поменять существующий код, чем плодить дополнительные расширения через наследование/композицию. В маленьких и средних проектах «открытость/закрытость» может создавать избыточные абстракции ради гипотетической возможности расширения.
5. `L` — Liskov Substitution Principle не всегда полезен. В реальном коде часто нарушается LSP ради оптимизации или особых случаев.
6. `I` — Interface Segregation Principle иногда вреден. Разделение интерфейсов может привести к слишком большому количеству мелких интерфейсов, что усложняет понимание кода.
7. `D` — Dependency Inversion Principle часто приводит к overengineering. В Rust тот же `DIP` почти всегда делается проще (через параметры и `generic-constraints`), а «контейнеры» нужны редко.

</details>

---

</details>

## [KISS](https://en.wikipedia.org/wiki/KISS_principle)

**KISS (Keep It Simple, Stupid)**
Большинство систем работают надёжнее и обслуживаются проще, если они остаются простыми, а не усложняются. Простота должна быть ключевой целью проектирования, а излишняя сложность — исключением, а не нормой.

<details>
<summary>...</summary>

**Ключевые идеи:**
1. **Простота кода — превыше всего**, потому что простой код легче понять, тестировать, сопровождать и изменять.
2. Использовать паттерн проектирования **только при наличии проблемы**, которую он реально решает. Применение паттернов "на всякий случай" нарушает KISS, добавляя лишнюю сложность.
3. Не использовать паттерн проектирования там, где он нужен, — тоже нарушение KISS, потому что это усложняет код за счёт ручных решений, уже решённых известным подходом.
4. Простота достигается не минимизацией кода любой ценой, а **минимизацией сложности** — явной и скрытой.

**Исторический контекст:**

Фраза KISS («будь проще, глупый») была введена авиаинженером Келли Джонсоном. Его команда должна была спроектировать реактивный самолёт так, чтобы его можно было обслуживать обычным механиком в полевых условиях с ограниченным набором инструментов.
Главная идея: проектируйте системы так, чтобы они были понятны, ремонтопригодны и работоспособны при минимуме необходимых средств.

Всвязи с тем, что представления разных людей о таком понятии как «простота» могут различаться, приобрели широкое распространение следующая заблуждения относительно KISS-a:
- Заблуждение 1. Если считать, что простой код – это такой код, который проще всего написать, то можно истолковать, что принцип KISS призывает писать первое что взбредёт в голову, вообще не задумываясь о проектировании.
- Заблуждение 2. Если считать, что простой код – это такой код, для написания которого требуется как можно меньше знаний, то можно истолковать, что принцип KISS призывает не использовать паттерны проектирования.

**Принцип наименьшего удивления** (POLA - Principle of least astonishment).
Компонент системы должен вести себя так, как ожидает большинство пользователей. Его поведение не должно вызывать удивления или удивления у пользователей.

[KISS — принцип проектирования, содержащий все остальные принципы проектирования](https://habr.com/ru/articles/249639/)

</details>

## DRY 

"**Не повторяйте себя**" (Don’t Repeat Yourself).

<details>
<summary>...</summary>

Следование этому принципу означает, что ваша цель — **сократить количество повторяющихся шаблонов, дублирования кода, логики, в пользу модульного кода, на который можно ссылаться т.е. использовать повторно**.

В книге «Программист-прагматик» мы можем увидеть такое определение DRY:
«Каждая часть знаний должна иметь единственное, однозначное и авторитетное представление в системе»

**Суть:**
* Избегайте дублирования кода, логики и знаний.
* Предпочитайте модульный и переиспользуемый код.
* Все изменения должны вноситься **в одном месте**.

**Почему это важно:**
1. **Легче поддерживать** — меняешь в одном месте, и всё обновляется.
2. **Меньше багов** — нет риска, что в одном месте обновили, а в другом забыли.
3. **Код компактнее и чище** — легче читать, понимать и тестировать.

**Опасности дублирования:**
* Разные версии "одного и того же" кода со временем расходятся.
* Исправление ошибки в одном месте не исправляет её в других.
* Рост сложности и размера кода без реальной пользы.

</details>

## YAGNI 

"**Тебе это не понадобится**" (You Aren’t Gonna Need It)

<details>
<summary>...</summary>

Это означает, что вам не следует реализовывать функциональность только потому, что вы думаете, что она вам когда-нибудь понадобится, а реализовывать ее только тогда, когда она вам действительно понадобится. Поступая так, вы избежите траты времени на реализации, которые даже не были необходимы и, возможно, никогда не будут использоваться.

Не реализовывай функциональность **"на всякий случай"**. Делай только то, что нужно **прямо сейчас**, на основе подтверждённых требований.

**Суть:**
* Не трать время на код, который *возможно* пригодится.
* Избегай преждевременной оптимизации и расширения.
* Реализуй только то, что необходимо для текущих задач.

**Почему это важно:**
1. **Экономия времени** — ты не пишешь лишний код.
2. **Меньше багов** — нет ненужных функций, которые могут сломаться.
3. **Меньше сложности** — легче поддерживать и понимать систему.
4. **Сочетается с KISS** — нет искусственного усложнения.
5. **Сочетается с DRY** — меньше шансов на дублирование "полуготовых" решений.

[Design principles](https://rust-unofficial.github.io/patterns/additional_resources/design-principles.html)

[dry-kiss-yagni-principles](https://henriquesd.medium.com/dry-kiss-yagni-principles-1ce09d9c601f)

</details>

## GRASP

Общие принципы распределения ответственности (GRASP) — это набор из девяти принципов, используемых в объектно-ориентированном проектировании, представленных Крейгом Ларманом в его книге «Применение UML и шаблонов».

<details>
<summary>...</summary>


1. **Information Expert** (Информационный эксперт) Шаблон определяет базовый принцип распределения обязанностей:

"Информационный эксперт" — это не всегда единственный владелец данных, а тот, кто имеет их в наибольшем объёме и может выполнить обязанность с минимальными зависимостями.

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
Создатель не только "создаёт", но и инициализирует объект в корректном состоянии, чтобы снизить риски неправильной конфигурации.

Обычно это объект, который:
* Использует создаваемый объект,
* Обладает агрегированными данными для создаваемого объекта,
* Является родительским для создаваемого объекта,
* Хранит или записывает экземпляры создаваемого объекта.
 
Альтернатива — шаблон «Фабрика» (создание объектов концентрируется в отдельном классе).

3. **Controller** (Контроллер)

Этот шаблон определяет объект, который принимает и координирует выполнение операций.

Контроллеры обычно представляют собой объекты, управляющие жизненным циклом других объектов. 
Контроллер в GRASP — не обязательно MVC-контроллер, а любой объект, обрабатывающий сценарии использования.

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

Низкая связанность и Высокая когезия, всегда рассматриваются вместе — иногда уменьшение связанности снижает когезию, и наоборот, поэтому баланс важен.

6. **Polymorphism** (Полиморфизм)

Речь идёт об интерфейсном полиморфизме (через абстракции), а не только о наследовании.
Использование полиморфизма позволяет заменить условные операторы (например, `if` или `switch`) вызовами методов, которые могут быть реализованы различными способами в разных классах. 
Это улучшает расширяемость и изменяемость кода.

Устройство и поведение системы:

* Определяется данными;
* Задано полиморфными операциями её интерфейса.

Пример: Адаптация коммерческой системы к многообразию систем учёта налогов может быть обеспечена через внешний интерфейс объектов-адаптеров (смотрите также: Шаблон «Адаптеры»).

7. **Pure Fabrication** (Чистая фабрикация)

Это создание класса, не являющегося частью реального мира, для обеспечения высокой когезии и низкой связанности. 
Например, это может быть класс, реализующий функциональность, которая не может быть естественно присвоена другим объектам. 
Класс создаётся специально ради архитектурных целей, даже если это искусственный объект.

Не относится к предметной области, но:
* Уменьшает зацепление;
* Повышает связность;
* Упрощает повторное использование.

«Pure Fabrication» отражает концепцию сервисов в модели проблемно-ориентированного проектирования.

Пример задачи: Не используя средства класса «А», внести его объекты в базу данных.

Решение: Создать класс «Б» для записи объектов класса «А» (смотрите также: «Data Access Object»).

8. **Indirection** (Косвенность)

Этот шаблон вводит посредника для управления взаимодействием между объектами, с целью достижения низкой связанности и высокой когезии. 
Косвенность может быть реализована не только через посредника, но и, например, через событийную шину или слой адаптеров.

Например, использование паттерна "Посредник" (Mediator) для управления коммуникацией между модулями.
 
Слабое зацепление между элементами системы (и возможность повторного использования) обеспечивается назначением промежуточного объекта их посредником.

Пример: В архитектуре Model-View-Controller, контроллер (англ. controller) ослабляет зацепление данных (англ. model) за их представление (англ. view).

9. **Protected Variations** (Устойчивость к изменениям)   

Принцип, при котором система разрабатывается таким образом, чтобы защитить части системы от влияния вариаций в других частях. 
Мы явно выделяем точки потенциальных изменений и защищаем остальную систему с помощью стабильных интерфейсов или контрактов.
Это может быть достигнуто с помощью интерфейсов, абстракций и других методов.
Шаблон защищает элементы от изменения другими элементами (объектами или подсистемами) с помощью вынесения взаимодействия в фиксированный интерфейс, через который (и только через который) возможно взаимодействие между элементами. Поведение может варьироваться лишь через создание другой реализации интерфейса.

---

</details>

## LoD

The Law of Demeter (LoD) [Закон Деметры](https://backendinterview.ru/architecture/principles.html#%D0%97%D0%B0%D0%BA%D0%BE%D0%BD-%D0%94%D0%B5%D0%BC%D0%B5%D1%82%D1%80%D1%8B)

Принцип Деметры — это принцип проектирования программного обеспечения, направленный на минимизацию связности между различными компонентами системы. Он также известен как "принцип наименьшего знания". Объект должен взаимодействовать только с ближайшими соседями (не ходить через цепочки вызовов).

**Суть принципа:**
Объект должен взаимодействовать только с теми объектами, которые он непосредственно знает и с которыми связан, а не с объектами, которые являются "посредниками" или находятся на уровне глубже.

**Простыми словами:**
Каждый объект должен знать о структуре других объектов как можно меньше. То есть объект не должен обращаться напрямую к внутренностям других объектов и передавать управление дальше по цепочке.

<details>
<summary>...</summary>

Говоря упрощённо, каждый программный модуль:
* должен обладать ограниченным знанием о других модулях: знать о модулях, которые имеют «непосредственное» отношение к этому модулю.
* должен взаимодействовать только с известными ему модулями «друзьями», не взаимодействовать с незнакомцами.
обращаться только к непосредственным «друзьям».

LoD напрямую помогает достигать "Low Coupling" (из [GRASP](https://github.com/Jekahome/Patterns?tab=readme-ov-file#grasp)) и облегчает [рефакторинг](https://github.com/Jekahome/Patterns#refactoring).

Аналогия из жизни: Если Вы хотите, чтобы собака побежала, глупо командовать её лапами, лучше отдать команду собаке, а она уже разберётся со своими лапами сама.

Основной идеей является то, что объект должен иметь как можно меньше представления о структуре и свойствах чего угодно (включая собственные подкомпоненты).

Общее описание правила: Объект `A` не должен иметь возможность получить непосредственный доступ к объекту `C`, если у объекта `A` есть доступ к объекту `B` и у объекта `B` есть доступ к объекту `C`.

Более формально, Закон Деметры для функций требует, что бы метод `М` у объекта `О` мог вызывать методы только следующих типов объектов:
* собственно самого `О`
* параметров `М`
* других объектов, созданных в рамках `М`
* прямых компонентных объектов `О`
* глобальных переменных, доступных `О`, в пределах `М`

Практически, объект-клиент должен избегать вызовов методов объектов, внутренних членов, возвращенных методом объекта-сервиса.

В общем случае можно сказать, что LoD не работает, когда к одному объекту применено знание особенностей устройства через другой обьект, например, `object.friend.stranger` вместо `object.friend` или такое нарушение принципа `String cityName = person.getAddress().getCity().getName();`

В Rust меньше риск нарушить LoD через прямой доступ, если поля `pub` не выставлены. Инкапсуляция через `pub(crate)` и приватные поля помогает соблюдать LoD естественным образом.

**❌ Нарушение LoD**

```rust
struct User {
    pub account: Account,
}

struct Account {
    pub balance: u64,
}

// ❌ нарушение LoD (прямой доступ к вложенному полю)
fn print_user_balance(user: &User) {
    println!("Balance: {}", user.account.balance); 
}
```

**✅ Соблюдение LoD**

```rust
impl User {
    pub fn get_balance(&self) -> u64 {
        self.account.get_balance()  
    }
}

impl Account {
    pub fn get_balance(&self) -> u64 {
        self.balance
    }
}

fn print_user_balance(user: &User) {
    println!("Balance: {}", user.get_balance()); // ✅
}
```

</details>

## SoC

**SoC (Separation of Concerns)** — это принцип разделения обязанностей или ответственности в программной инженерии, который предполагает, что разные части программы должны решать строго определённые задачи и быть независимыми друг от друга. 

**Суть принципа:**
Каждый компонент системы должен отвечать только за одну "зону ответственности" или "аспект". Эти зоны ответственности должны быть максимально разделены, чтобы изменения в одной зоне не затрагивали другие.

SoC тесно переплетается с [SRP (Single Responsibility Principle из SOLID)](https://github.com/Jekahome/Patterns?tab=readme-ov-file#%D0%BF%D1%80%D0%B8%D0%BD%D1%86%D0%B8%D0%BF-%D0%B5%D0%B4%D0%B8%D0%BD%D0%BE%D0%B9-%D0%BE%D1%82%D0%B2%D0%B5%D1%82%D1%81%D1%82%D0%B2%D0%B5%D0%BD%D0%BD%D0%BE%D1%81%D1%82%D0%B8-srp---single-responsibility-principle).
Разница:
- SRP — применим к отдельному классу/модулю: «один класс — одна причина для изменения»
- SoC — более общий архитектурный принцип: «разные аспекты системы — разные зоны ответственности».

<details>
<summary>...</summary>

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

</details>

## Single Level of Abstraction (SLA)

Каждая функция или метод должна работать только на одном уровне абстракции — либо высокоуровневом (бизнес-логика, сценарии), либо низкоуровневом (конкретные операции с данными, технические детали), но не смешивать их.

Почему это важно:
- Код становится проще читать, потому что мозгу не приходится переключаться между «что» (высокоуровневое) и «как» (низкоуровневое) в одном методе.
- Легче тестировать — можно отдельно тестировать высокоуровневую логику и низкоуровневые детали.
- Упрощается рефакторинг — изменения на одном уровне не задевают другой.

<details>
<summary>...</summary>

**Нарушение SLA**

```rust
fn process_data(data: &str) -> Result<(), String> {
    // Низкоуровневая проверка
    if data.is_empty() {
        return Err("Empty data".to_string());
    }

    // Высокоуровневая логика
    let parsed = parse_data(data)?;

    // Опять низкоуровневая работа
    save_to_db(&parsed).map_err(|e| e.to_string())
}
```

**Соблюдение SLA**

```rust
fn process_data(data: &str) -> Result<(), String> {
    validate_data(data)?;
    let parsed = parse_data(data)?;
    store_data(parsed)
}

// Высокоуровневые шаги
fn validate_data(data: &str) -> Result<(), String> { ... }
fn parse_data(data: &str) -> Result<ParsedData, String> { ... }
fn store_data(data: ParsedData) -> Result<(), String> { ... }
```

---

</details>

## Command-Query Separation (CQS)

CQS — это принцип проектирования методов, который можно и нужно использовать практически везде, предложенный Бертраном Мейером, который разделяет методы на две категории:
* Команды (Commands) — изменяют состояние, но не возвращают данные
* Запросы (Queries) — возвращают данные, но не изменяют состояние

**Метод должен быть либо командой, либо запросом, но не обоими одновременно.**

<details>
<summary>...</summary>

**Преимущества CQS**
* Предсказуемость кода (ясно когда изменяется состояние)
* Безопасность в многопоточности (запросы можно вызывать из нескольких потоков без блокировок)
* Упрощение тестирования

❌ Нарушение CQS

```rust
// ПЛОХО: метод и изменяет состояние, и возвращает значение
struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) -> f64 {
        self.balance -= amount;
        self.balance // Нарушение: и команда, и запрос
    }
}
```

✅ Соблюдение CQS


```rust
// ХОРОШО: разделение на команду и запрос
struct BankAccount {
    balance: f64,
}

impl BankAccount {
    // КОМАНДА: изменяет состояние, ничего не возвращает
    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }
    
    // ЗАПРОС: возвращает данные, не изменяет состояние
    fn get_balance(&self) -> f64 {
        self.balance
    }
}
```

**Исключения из правила**:
* Создание объектов (фабричные методы)

```rust
impl User {
    // Исключение: создание объекта обычно возвращает его
    fn new(name: &str) -> Self {
        Self {
            id: generate_id(),
            name: name.to_string(),
        }
    }
}
```

* 2. Операции с очевидными побочными эффектами


```rust
struct Logger;
impl Logger {
    // Запись в лог - и команда, и может возвращать статус
    fn log(&mut self, message: &str) -> Result<(), IoError> {
        println!("{}", message);
        Ok(())
    }
}
```

</details>



# Gangs of Four (GoF) Design Patterns

#### Методы решения распространенных проблем при кодировании.

Шаблоны проектирования `GoF` делятся на три категории:

- **Порождающие** паттерны связанны с созданием объекта. `Singleton, Builder, Factory, Fabric Method, Prototype, Fold`

- **Структурные** паттерны связаны со структурой классов, такой как наследование и композиция. `Adapter, Bridge, Composite, Decorator, Facade, Proxy, Flyweight`

- **Поведенческие** паттерны обеспечивают решение для лучшего взаимодействия между объектами, обеспечения потери связнности и гибкости для легкого расширения в будущем. `Chain of Responsibility, Command, Interpreter, Iterator, Mediator, Memento, Observer, State, Strategy, Template Method, Visitor`

## <ins>Порождающие паттерны</ins>

Паттерны которые создают новые объекты, или позволяют получить доступ к уже существующим. 
То есть те шаблоны, по которым можно создать новый автомобиль и как это лучше сделать.

<details>
<summary>...</summary>

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

---

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

---

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

---

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

---

- ## Singelton	

`Pattern Singleton` (Одиночка) применяется в том случае, когда какой-либо класс может иметь только один экземпляр (или не иметь ни одного) и легко доступен из глобальной видимости.
 
`Pattern Singleton` нарушает принцип единственной ответственности класса (SRP), так как он доступен глобально для различных частей программы следовательно есть вероятность изменения в одной из частей.

`Pattern Singleton` решает сразу две проблемы:

1. Гарантирует наличие единственного экземпляра класса. Чаще всего это полезно для доступа к какому-то общему ресурсу, например, базе данных.

Представьте, что вы создали объект, а через некоторое время пробуете создать ещё один. В этом случае хотелось бы получить старый объект, вместо создания нового.

Такое поведение невозможно реализовать с помощью обычного конструктора, так как конструктор класса всегда возвращает новый объект.

2. Предоставляет глобальную точку доступа. Это не просто глобальная переменная, через которую можно достучаться к определённому объекту.  

### Недостатки:

Паттерн **Синглтон** (Singleton) часто считается **антипаттерном** (anti-pattern) не потому, что он всегда плох, а из-за его **частого злоупотребления** и ряда серьезных недостатков, которые он привносит в код. По сути, он маскирует **глобальное состояние** и **скрытые зависимости**, что противоречит принципам хорошего объектно-ориентированного дизайна (ООП).

Вот ключевые причины, по которым Синглтон критикуют:

 

**1. Глобальное состояние и скрытые зависимости**

Синглтон обеспечивает **глобальную точку доступа** к своему единственному экземпляру.

* **Глобальное состояние:** Экземпляр Синглтона по сути ведет себя как **глобальная переменная**. Любая часть кода может изменить его внутреннее состояние, и это изменение отразится на всей системе. Это делает поведение программы **непредсказуемым** и **затрудняет отладку**, поскольку сложно определить, какая именно часть кода и когда изменила состояние.
* **Скрытые зависимости:** Классы, использующие Синглтон, не объявляют эту зависимость явно (например, через конструктор или параметры метода), а просто вызывают статический метод `getInstance()`. Это создает **скрытые зависимости**, которые не видны в публичном контракте класса, что усложняет понимание структуры кода и его модификацию.
 

**2. Проблемы с тестированием**

Синглтон критически усложняет **модульное (юнит) тестирование**.

* **Нарушение изоляции:** Юнит-тесты должны быть независимыми и запускаться в изоляции. Поскольку Синглтон поддерживает глобальное состояние, тесты одного класса могут влиять на состояние Синглтона, которое затем будет использовано в тестах другого класса. Это означает, что **порядок выполнения тестов имеет значение**, а результаты могут быть нестабильными.
* **Сложность подмены (Mocking):** Большинство тестовых фреймворков для создания тестовых заглушек (mock-объектов) полагаются на механизмы, требующие передачи зависимостей (например, через интерфейсы или конструкторы). Синглтон с его приватным конструктором и статическим доступом делает **подмену или имитацию** его поведения крайне сложной, если не невозможной, без сложных хаков.


**3. Нарушение принципов SOLID**

Синглтон часто нарушает основные принципы ООП, в частности:

* **Принцип единственной ответственности (SRP):** Класс Синглтона берет на себя **две** обязанности:
    1.  Непосредственная **бизнес-логика** класса.
    2.  Логика **управления своим жизненным циклом** (гарантия единственного экземпляра и предоставление глобального доступа).
    Это нарушает SRP, который гласит, что у класса должна быть только **одна причина для изменения**.
* **Принцип открытости/закрытости (OCP):** Класс Синглтона, как правило, тесно связан со своей реализацией, что **снижает гибкость**. Расширение функциональности или изменение поведения может потребовать модификации существующего кода во многих местах, где он используется.

 
**4. Проблемы многопоточности**

В многопоточной среде реализация Синглтона требует дополнительных усилий для обеспечения **потокобезопасности**.

* Если несколько потоков одновременно попытаются создать экземпляр Синглтона (в случае ленивой инициализации), может возникнуть **состояние гонки**, и в результате будет создано несколько экземпляров, что нарушит основную цель паттерна. Для предотвращения этого необходимо использовать механизмы синхронизации (например, блокировки), которые, в свою очередь, могут **снизить производительность**.


[Singleton refactoring.guru](https://refactoring.guru/design-patterns/singleton/rust/example#example-1)

---

</details>

## <ins>Структурирующие паттерны</ins>

Данные паттерны помогают внести порядок и научить разные объекты более правильно взаимодействовать друг с другом.

<details>
<summary>...</summary>

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

---

</details>

## <ins>Паттерны поведения</ins>

Эта группа паттернов позволяет структурировать подходы к обработке поведения и взаимодействия объектов. Проще говоря, как должны проходить процессы в которых существует несколько вариантов протекания событий.

<details>
<summary>...</summary>

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

---

</details>

[Rust Design Patterns](https://chercher.tech/rust/observer-design-pattern-rust)

[Паттерны проектирования](http://design-pattern.ru/patterns/)

[Каталог шаблонов архитектуры корпоративных приложений](https://martinfowler.com/eaaCatalog/index.html)

[Паттерны ООП в метафорах](https://habr.com/ru/articles/136766/)

# Other Design Patterns
- Object Pool Pattern
- Private Class Data	
- Specification
- Delegation
- Service Locator
- Dependency injection (DI)

<details>
<summary>...</summary>

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

`TODO: добавить больше информации, пример, пояснение`

Объект, вместо того чтобы выполнять одну из своих поставленных задач, поручает её связанному вспомогательному объекту.

[Delegation](https://snoekiede.medium.com/easy-delegation-in-rust-the-delegation-pattern-hacking-with-rust-9366f10bf7f2?source=user_profile---------19----------------------------)

- ## Service Locator	

`TODO: добавить больше информации, пример, пояснение`

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

---

</details>

# `PoSA` Архитектура программного обеспечения, ориентированная на шаблоны

[PoSA](https://en.wikipedia.org/wiki/Pattern-Oriented_Software_Architecture)

[PoSA github.com/ppizarro](https://github.com/ppizarro/coursera/blob/master/POSA/Books/Pattern-Oriented%20Software%20Architecture/Pattern-Oriented%20Software%20Architecture%20~%20Vol%203%20~%20Patterns%20for%20Resource%20Management%20(Wiley~2004.06).chm)

`TODO: добавить больше информации, пример, пояснение`

...

# Database Patterns

Шаблоны баз данных, сохранять и извлекать данные из баз данных и устанавливать соответствие между объектами базы данных и приложения.

- Active Record
- Identity Map
- Data Mapper
- Repository
- Unit of Work (UOW)
- Lazy Load

<details>
<summary>...</summary>

- ## Active Record	

`TODO: добавить больше информации, пример, пояснение`

Схема Active Record — это подход к доступу к данным в базе данных. Таблица базы данных или представление обёрнуты в классы. 
Таким образом, объектный экземпляр привязан к единственной строке в таблице. После создания объекта новая строка будет добавляться к таблице на сохранение. 
Любой загруженный объект получает свою информацию от базы данных. Когда объект обновлён, соответствующая строка в таблице также будет обновлена. 
Класс обёртки реализует методы средства доступа или свойства для каждого столбца в таблице или представлении.
Нарушает принцип единственной ответственности (SRP) или нет ?

- ## Identity Map	

(Identity Map => Data Mapper => Repository => Unit of Work)

`TODO: добавить больше информации, пример, пояснение`

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

`TODO: добавить больше информации, пример, пояснение`

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

`TODO: добавить больше информации, пример, пояснение`

Загрузка данных по мере необходимости. Объект, не содержит данных, но знает, где их взять.
При первом обращении или при свободном ресурсе, происходит загрузка, последующие обращения используют тот же обьект не загружая обьект из источника.

**Существует четыре основных варианта ленивой загрузки.**


| Подход              | Когда грузим                     | Как устроено                                 |
| ------------------- | -------------------------------- | -------------------------------------------- |
| Lazy Initialization | При первом доступе к **полю**    | Маркер `None` / `null`                       |
| Virtual Proxy       | При первом **методе интерфейса** | Прокси с тем же API, создаёт реальный объект |
| Value Holder        | При первом `getValue()`          | Обёртка с лямбдой-загрузчиком                |
| Ghost               | При первом методе                | Пустой объект, сразу грузит **все** данные   |

<details>
<summary>...</summary>

1. Lazy Initialization (Ленивая Инициализация) использует специальный макер (обычно null), чтобы пометить поле, как не загруженное. При каждом обращении к полю проверяется значение маркера и, если значение поля не загружено - оно загружается.

```rust
struct Data {
    value: Option<String>,
}

impl Data {
    fn new() -> Self {
        Self { value: None }
    }

    fn get_value(&mut self) -> &str {
        if self.value.is_none() {
            println!("Загружаем данные...");
            self.value = Some("Hello from DB".to_string());
        }
        self.value.as_ref().unwrap()
    }
}

fn main() {
    let mut data = Data::new();
    println!("Первый доступ: {}", data.get_value());
    println!("Второй доступ: {}", data.get_value()); // Уже без загрузки
}

```

2. Virtual Proxy (Виртуальный Прокси) - объект с таким же интерфейсом, как и настоящий объект. При первом обращении к методу объекта, виртуальный прокси загружает настоящий объект и перенаправляет выполнение.  Прокси реализует тот же интерфейс (Service), но создаёт настоящий объект только при первом вызове.

```rust
trait Service {
    fn process(&self);
}

struct RealService {
    payload: String,
}

impl RealService {
    fn new() -> Self {
        println!("Загружаем реальный объект...");
        Self { payload: "Real Data".into() }
    }
}

impl Service for RealService {
    fn process(&self) {
        println!("Обработка: {}", self.payload);
    }
}

struct VirtualProxy {
    real: Option<RealService>,
}

impl VirtualProxy {
    fn new() -> Self {
        Self { real: None }
    }
}

impl Service for VirtualProxy {
    fn process(&self) {
        if self.real.is_none() {
            // ❌ Ошибка: self не mut — значит надо менять структуру
            // решение: сделать RefCell для внутренней мутации
            println!("Придётся использовать RefCell для ленивой инициализации");
        }
    }
}

fn main() {
    // Чтобы Virtual Proxy работал без лишнего мута, используем RefCell
    use std::cell::RefCell;

    struct VirtualProxyCell {
        real: RefCell<Option<RealService>>,
    }

    impl VirtualProxyCell {
        fn new() -> Self {
            Self { real: RefCell::new(None) }
        }
    }

    impl Service for VirtualProxyCell {
        fn process(&self) {
            if self.real.borrow().is_none() {
                let mut r = self.real.borrow_mut();
                println!("Создаём реальный объект...");
                *r = Some(RealService::new());
            }
            self.real.borrow().as_ref().unwrap().process();
        }
    }

    let proxy = VirtualProxyCell::new();
    proxy.process(); // Создание и вызов
    proxy.process(); // Уже без создания
}

```

3. Value Holder (Контейнер значения) - объект с методом getValue. Клиент вызывает метод getValue, чтобы получить реальный объект. getValue вызывает загрузку. Хранится замыкание, которое знает, как получить данные, и само значение.

```rust
struct ValueHolder<T, F>
where
    F: Fn() -> T,
{
    value: Option<T>,
    loader: F,
}

impl<T, F> ValueHolder<T, F>
where
    F: Fn() -> T,
{
    fn new(loader: F) -> Self {
        Self { value: None, loader }
    }

    fn get_value(&mut self) -> &T {
        if self.value.is_none() {
            println!("Загружаем через ValueHolder...");
            self.value = Some((self.loader)());
        }
        self.value.as_ref().unwrap()
    }
}

fn main() {
    let mut holder = ValueHolder::new(|| {
        println!("Функция загрузки...");
        "Data from Loader".to_string()
    });

    println!("{}", holder.get_value()); // первая загрузка
    println!("{}", holder.get_value()); // уже кэшировано
}

```


4. Ghost (Призрак) - объект без каких-либо данных. При первом обращении к его методу, призрак загружает все данные сразу. Объект пустой при создании, но при первом использовании грузит всю необходимую информацию.

```rust
struct Ghost {
    loaded: bool,
    data: Option<String>,
}

impl Ghost {
    fn new() -> Self {
        Self { loaded: false, data: None }
    }

    fn process(&mut self) {
        if !self.loaded {
            println!("Ghost загружает ВСЕ данные сразу...");
            self.data = Some("Full dataset".into());
            self.loaded = true;
        }
        println!("Работаем с: {}", self.data.as_ref().unwrap());
    }
}

fn main() {
    let mut ghost = Ghost::new();
    ghost.process(); // Загружает сразу всё
    ghost.process(); // Уже использует готовые данные
}

```

</details>

P.S. В Rust'е итераторы ленивы, также `std::borrow::Cow` обладает свойствами бережного обращения к русурсам, и smart pointer `Rc/Arc`

[Lazy Load](http://design-pattern.ru/patterns/lazy-load.html)

---

</details>

## [Паттерны Объектно-Реляционного структурирования](http://design-pattern.ru/patterns/)

`TODO: добавить больше информации, пример, пояснение`

- Identity Field (Поле первичного ключа)
- Foreign Key Mapping (Разметка внешних ключей)
- Association Table Mapping (Разметка таблиц связей)
- Dependent Mapping (Управление распределением подчинённых сущностей)
- Embedded Value (Объединённое свойство)
- Serialized LOB (Сериализованный LOB)
- Single Table Inheritance (Наследование с единой таблицей)
- Class Table Inheritance (Наследование с таблицами классов)
- Concrete Table Inheritance (Наследование с таблицами конечных классов)
- Inherritance Mappers (Наследуемые распределители)





# [Architecture](https://martinfowler.com/architecture/)

... Eric Evans’s Domain-Driven Design or Martin Fowler’s Patterns of Enterprise Application Architecture

## Architectural Styles

### Monolithic

**Монолитная архитектура (Monolithic)**
* **Суть:** Вся система разрабатывается, развертывается и работает как единое целое. Компоненты тесно связаны и выполняются в одном процессе.
* **Эксперимент:** Сравнить классический многослойный монолит (Presentation-Business-Data layers) с модульным монолитом (Modular Monolith), где модули имеют строгие границы.

### Messaging

**Puplish-Subscribe**

`TODO: добавить больше информации, пример, пояснение`

**Event-Driven**

**Событийно-ориентированная архитектура (Event-Driven Architecture, EDA)**
* **Суть:** Компоненты системы общаются путем асинхронной рассылки и обработки событий. Они слабо связаны и не знают друг о друге.
* **Эксперимент:** Спроектировать одну и ту же систему, используя EDA и более традиционный запросно-ответный стиль (например, REST), и сравнить производительность, отказоустойчивость и сложность.

`TODO: добавить больше информации, пример, пояснение`

### Distributed

**Client-Server**

`TODO: добавить больше информации, пример, пояснение`

**Peer-to-Peer**

`TODO: добавить больше информации, пример, пояснение`

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

`TODO: добавить больше информации, пример, пояснение`

## [Architectural Pattern Microservices](https://backendinterview.ru/architecture/microserices/index.html)

**Микросервисная архитектура (Microservices)**
* **Суть:** Система разбивается на небольшие, слабо связанные сервисы, каждый из которых отвечает за свою бизнес-возможность. Сервисы развертываются независимо и общаются по сети.
* **Эксперимент:** Сравнить разные способы коммуникации (синхронный REST vs асинхронный messaging) или разные стратегии декомпозиции (по бизнес-возможностям vs по поддоменам DDD).

`TODO: добавить больше информации, пример, пояснение`

## [Architectural Pattern Event Sourcing](https://backendinterview.ru/architecture/architecturesPatterns.html#event-sourcing)

`TODO: добавить больше информации, пример, пояснение`

## Architectural Pattern CQRS

`CQRS` (Command Query Responsibility Segregation) — это шаблон проектирования, предлагающий разделение команд (изменяющих данные) и запросов (читающих данные) в приложении. 

Принцип **Command-Query Separation (CQS)** разделение команд-запросов «Функции не должны вызывать абстрактные побочные эффекты... только команды (процедуры) могут вызывать побочные эффекты». - Бертран Мейер: Объектно-ориентированное создание программного обеспечения.

CQS - "Метод должен либо изменять состояние объекта (команда), либо возвращать данные (запрос), но не делать оба действия одновременно."

CQS и CQRS — **это разные**, хотя и связанные концепции. CQS действует на уровне методов/обьектов для поддержания частоты кода и не нуждается в синхронизации данных так как все в одном объекте, а CQRS на архитектурном уровне всей системы для масштабируемости и требует явной синхронизации данных. CQS является основой для понимания CQRS, но они решают разные задачи на разных уровнях абстракции.

<details>
<summary>Архитектурный паттерн CQRS разделения на разные модели:</summary>


```rust
// CQRS - разные модели для команд и запросов

// МОДЕЛЬ КОМАНД (для записи)
struct BankAccountWrite {
    balance: f64,
}

impl BankAccountWrite {
    fn new() -> Self {
        Self { balance: 0.0 }
    }
    
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        // Здесь может быть сложная бизнес-логика
    }
    
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }
}

// МОДЕЛЬ ЗАПРОСОВ (для чтения)
struct BankAccountRead {
    account_id: u64,
    balance: f64,
    last_transactions: Vec<Transaction>,
}

impl BankAccountRead {
    fn get_balance(&self) -> f64 {
        self.balance
    }
    
    fn get_transaction_history(&self) -> &[Transaction] {
        &self.last_transactions
    }
    
    fn get_account_details(&self) -> AccountDetails {
        // Комплексные данные для UI
    }
}

// Синхронизация между моделями (через события)
struct AccountEventPublisher {
    // Отправка событий о изменениях
}

impl AccountEventPublisher {
    fn publish_deposit_event(&self, account_id: u64, amount: f64) {
        // Отправка в систему сообщений
    }
}
```

</details>


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
 
---

</details>

## Architectural Pattern Domain-Driven Design (DDD) 

`TODO: добавить больше информации, пример, пояснение`

`DDD` означает "Domain-Driven Design" (Проектирование с учетом предметной области) и представляет собой методологию и набор принципов, разработанных Эриком Эвансом. 
`DDD` ориентировано на решение сложных задач в области проектирования программного обеспечения, особенно там, где ключевой упор делается на моделирование предметной области.

<details>
<summary>...</summary>

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

---

</details>

## Layered architecture 

`TODO: добавить больше информации, пример, пояснение`

Слоенная архитектура (Layered Architecture) - это структурный подход к организации кода, в котором приложение разделяется на логические слои (или уровни), 
каждый из которых выполняет определенные функции. 
Каждый слой зависит только от слоев, находящихся ниже, и предоставляет интерфейсы для взаимодействия с вышележащими слоями. 
Это помогает создать модульную и легко поддерживаемую структуру приложения.

<details>
<summary>...</summary>

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

---

</details>

## Hexagonal Architecture (Шестиугольная архитектура)

`TODO: добавить больше информации, пример, пояснение`

`Hexagonal Architecture` (Шестиугольная архитектура), также известная как Ports and Adapters (Порты и Адаптеры), это паттерн архитектуры, предложенный Алистером Кокберном. 
Он призван обеспечить легкость тестирования, гибкость и отделение бизнес-логики от деталей инфраструктуры.

`Hexagonal Architecture` поддерживает принципы чистой архитектуры (`SOLID`) и способствует созданию гибких, тестируемых и легко поддерживаемых приложений.

<details>
<summary>...</summary>

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

---

</details>

# Refactoring

Рефакторинг — это процесс изменения программного кода с целью улучшения его структуры, читаемости, поддерживаемости и производительности, при этом без изменения его внешнего поведения, но с устранением [`code smells`](https://github.com/Jekahome/Patterns/tree/main?tab=readme-ov-file#code-smells) (плохих практик) и [`антипаттернов`](https://github.com/Jekahome/Patterns/tree/main?tab=readme-ov-file#anti-patterns) (неэффективных решений). Рефакторинг позволяет разработчикам вносить изменения в программу, не нарушая функциональность и улучшая качество.

<details>
<summary>...</summary>

Важность рефакторинга становится очевидной по мере роста и развития программного проекта. Код, написанный в начале разработки, может быть неоптимальным и сложным. Накопление такого кода замедляет разработку и закладывает проблемы на будущее. Рефакторинг позволяет устранить этот технический долг, делая код более понятным, гибким и эффективным.

**Главная цель рефакторинга** — изменить структуру кода без изменения его внешнего поведения. Это вызывает потребность в тщательном тестировании после каждого рефакторинга, чтобы убедиться, что код все еще работает корректно.

Основные этапы рефакторинга:
1. Выявление проблем (code smells и антипаттернов).
2. Применение техник рефакторинга для их устранения.
   1. Rename Method → Улучшение читаемости.
   2. Extract Class → Разделение ответственности.
   3. Replace Conditional with Polymorphism → Упрощение логики.
   4. Introduce Parameter Object → Уменьшение количества параметров.
3. Тестирование, чтобы убедиться, что функциональность не сломана.
 
[Refactoring](https://refactoring.com/)

[Refactoring](https://refactoring.guru/ru/refactoring)

[Refactoring](https://sourcemaking.com/refactoring)

[Что такое рефакторинг?](https://foxminded.ua/ru/refaktoring/)

---

</details>



# Квадрант технического долга по Мартину Фаулеру

Мартин Фаулер, известный эксперт в области разработки программного обеспечения, предложил модель квадранта технического долга, которая помогает классифицировать виды технического долга на основе двух критериев:

<details>
<summary>...</summary>

1. **Намеренный (Intentional) vs. Ненамеренный (Unintentional)**
2. **Безрассудный (Reckless) vs. Осторожный (Prudent)**

Эта модель разделяет технический долг на четыре квадранта:

### 1. **Намеренный и Осторожный (Prudent & Intentional Debt)**
Описание:

Технический долг создаётся осознанно с целью достижения определённых бизнес-целей. Это может быть компромисс между скоростью разработки и качеством кода, когда команда понимает, что в будущем потребуется рефакторинг или улучшение.

Примеры:
- Временное упрощение архитектуры для быстрого вывода продукта на рынок.
- Использование устаревших технологий с целью сокращения времени разработки, зная, что позже будет проведено обновление.

### 2. **Намеренный и Безрассудный (Reckless & Intentional Debt)**
Описание:

Технический долг создаётся осознанно, но без достаточного обоснования или без учёта будущих последствий. Это может привести к проблемам с поддержкой и масштабированием приложения.

Примеры:
- Игнорирование стандартов кодирования ради ускорения разработки без планирования рефакторинга.
- Внедрение временных решений без понимания их влияния на долгосрочную устойчивость системы.

### 3. **Ненамеренный и Осторожный (Prudent & Unintentional Debt)**
Описание:

Технический долг возникает непреднамеренно, несмотря на лучшие усилия команды поддерживать высокое качество кода. Это может быть результатом недостатка знаний, изменения требований или других внешних факторов.

Примеры:
- Возникновение технического долга из-за отсутствия документации или понимания требований.
- Наследование устаревшего кода, который сложно поддерживать, без возможности сразу его улучшить.

### 4. **Ненамеренный и Безрассудный (Reckless & Unintentional Debt)**
Описание:

Технический долг появляется из-за небрежности или отсутствия дисциплины в процессе разработки. Это часто связано с плохими практиками, недостаточным тестированием или игнорированием стандартов.

Примеры:
- Частое внесение изменений в код без соответствующих тестов.
- Отсутствие кода рецензирования, что приводит к накоплению ошибок и ухудшению качества кода.

### **Важность Понимания Квадранта Технического Долга**

Понимание того, в каком квадранте находится текущий технический долг, помогает команде:

- **Приоритизировать** задачи по устранению долга.
- **Разрабатывать стратегии** для предотвращения накопления новых долгов.
- **Балансировать** между скоростью разработки и качеством продукта.
- **Обосновывать** решения перед стейкхолдерами, показывая, какие долги являются осознанными и необходимыми для достижения бизнес-целей.

Заключение:

Квадрант технического долга Мартином Фаулером предоставляет полезную рамку для анализа и управления техническим долгом в проектах разработки ПО. Осознанное управление техническим долгом способствует улучшению качества кода, повышению эффективности команды и устойчивости продукта в долгосрочной перспективе.

---

</details>


# CAP-теорема

### **CAP-теорема** (Consistency, Availability, Partition Tolerance)  
Это фундаментальная теорема в распределённых системах, сформулированная **Эриком Брюером** в 2000 году. Она утверждает, что в **распределённой базе данных** невозможно одновременно обеспечить три свойства:  

1. **Consistency (Согласованность)** — все узлы системы видят одни и те же данные в одно и то же время.  
2. **Availability (Доступность)** — каждый запрос получает ответ (может быть не самым свежим).  
3. **Partition Tolerance (Устойчивость к разделению)** — система продолжает работать, даже если соединение между узлами разорвано.  

### **Вывод CAP-теоремы**  
В реальной системе можно добиться **только двух из трёх** свойств:  
- **CP (Consistency + Partition Tolerance)** — данные всегда согласованы, но могут быть недоступны в случае разделения сети. (Пример: **MongoDB в режиме строгой консистентности**)  
- **AP (Availability + Partition Tolerance)** — система всегда доступна, но данные могут быть несогласованными. (Пример: **Cassandra, DynamoDB**)  
- **CA (Consistency + Availability)** — невозможен в распределённых системах, так как при сетевых сбоях одно из свойств обязательно будет нарушено.  



## Sources
 
[Rust Design Patterns rust-unofficial](https://rust-unofficial.github.io/patterns/)

[Rust Design Patterns github.com/rust-unofficial](https://github.com/rust-unofficial/patterns)

[Rust Design Patterns refactoring.guru](https://refactoring.guru/ru/design-patterns/rust)




 
