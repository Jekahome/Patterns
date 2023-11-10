## Newtype

Предотвращает недопустимое использование данных.
Завернув обший тип в свою оболочку мы получаем свой собственный тип данных,
который неполучится спутать с его внутренным типом и подставить в неверное место,
так же вы можете обеспечить требуемые инварианты для значений типа, ограничив или расширив их.

Также newtype ближе к предметной области и следовательно лугче документировать и понять его применение.

Newtype не несет дополнительных накладных расходов во время выполнения (абстракция с нулевой стоимостью),
но для полноценного использования newtype требуется реализовать трейты (Clone/Copy,From/Into,AsRef​​/AsMut...)

Также [Newtype](https://rust-unofficial.github.io/patterns/patterns/structural/compose-structs.html) применим для решения пробоем с проверкой заимствований полей структуры.
Хотя поля могут быть заимствованы независимо, иногда вся структура используется сразу, что предотвращает другое использование. Решением может быть разложение структуры на несколько более мелких структур. 
Декомпозиция структур позволяет обойти ограничения средства проверки заимствований. И это часто приводит к лучшему дизайну.

[Rust Design Patterns: Newtype](https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html)

[Rust By Example: 14.7. New Type Idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)

[Alexis King: Parse, don’t validate (ru)](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate)

[Stefan Baumgartner: Refactoring in Rust: Abstraction with the Newtype Pattern](https://fettblog.eu/refactoring-rust-abstraction-newtype)

[Official nutype crate docs](https://docs.rs/nutype)

## Type states

Rust использует traits для выражения поведения понятий и дженериков, которые позволяют параметризовать тип. 
Их комбинация позволяет нам выражать поведение типов, которое может быть проверено во время компиляции. 

### В идеале мы хотели бы видеть следующие характеристики:

- Может быть только в одном состоянии за раз.

- Каждое State shared должно иметь свои собственные связанные значения, если это необходимо.

- Переход между состояниями должен иметь четко определенную семантику.

- Должно быть возможно иметь некоторый уровень общего состояния.

- Допускаются только явно определенные переходы.

- Переход из одного состояния в другое должен потреблять состояние, чтобы его больше нельзя было использовать.

- Нам не нужно выделять память для всех состояний. Не более, чем размер наибольшего размера.

- Любые сообщения об ошибках должны быть легко понятны.

- Для этого нам не нужно прибегать к распределению кучи. Все должно быть возможно в стеке.

- Система типов должна быть использована для наших самых больших способностей.

- Как можно больше ошибок должно быть во время компиляции 

- Проверка смена состояний проверяется на момент компиляции обеспечивается фантомным типом <T>

[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_1_type_safety)

[David Teller: Typestates in Rust](https://yoric.github.io/post/rust-typestate)

[Cliff L. Biffle: The Typestate Pattern in Rust](https://cliffle.com/blog/rust-typestate)

[Ana Hobden: Pretty State Machine Patterns in Rust](https://hoverbear.org/2016/10/12/rust-state-machine-pattern)

[Will Crichton: Type-level Programming in Rust](https://willcrichton.net/notes/type-level-programming)

[Sergey Potapov: Builder with typestate in Rust](https://www.greyblake.com/blog/builder-with-typestate-in-rust)

[Azriel Hoh: Compile Time Correctness: Type State](https://peace.mk/blog/compile-time-correctness-type-state)

[Phantom type parameters](https://doc.rust-lang.org/stable/rust-by-example/generics/phantom.html)