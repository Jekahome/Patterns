## Bound impl

### Конкретизиируйте/опысывайте поведение, а не данные

Описывайте bond сигнатурой не данные, а поведение т.е. методы

Размещение границ признаков на impl блоках, методах и функциях, а не на типах, уменьшает загрязнение границ признаков, уменьшает связанность частей кода и делает общий код более чистым, простым и эргономичным.

```
trait UserRepo{}

// Плохо -------------------------------------------------------------------------------------
struct UserService<R: UserRepo> {
    repo: R,
}

Мы указываем R: UserRepo здесь bound, поскольку мы хотим ограничить типы в repo поле для реализации UserRepo поведения.
Однако такое ограничение непосредственно на тип приводит к так называемому «загрязнению границ признаков»: мы должны повторять эту границу в каждом отдельном случае impl, даже в тех, которые вообще не имеют отношения к UserRepo поведению.

impl<R> Display for UserService<R>
where
    R: Display + UserRepo, // Здесь нас не интересует UserRepo,  все, что нам нужно, это просто Display.
{                          
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserService with repo {}", self.repo)
    }
}
В сложной кодовой базе такое загрязнение умножается из-за разных типов и в какой-то момент может стать кошмаром.

Решением этой проблемы было бы понимание того, что черта представляет собой определенное поведение , и на самом деле нам нужно это поведение только тогда, когда мы его декларируем . 
В объявлении типа ничего не говорится о поведении, все дело в данных . 
Именно в функциях и методах происходит поведение. 
Итак, давайте просто ожидаем определенного поведения, когда нам это действительно нужно:
 

// Хорошо -------------------------------------------------------------------------------------
 
struct UserService<R> {
    repo: R,
}

// Ожидайте отображения, когда мы выражаем поведение отображения.
impl<R: Display> Display for UserService<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserService with repo {}", self.repo)
    }
}

// Ожидайте UserRepo, когда мы выражаем фактическое поведение UserService, который имеет дело с пользователями.
impl<R: UserRepo> UserService<R> {
    fn activate(&self, user: User) {
        // Изменение состояния пользователя в UserRepo ...
    }
}
```

### Структуры данных не дублируют производные границы признаков

```
// Хорошо -------------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
struct Good<T> { /* ... */ }

// Плохо ---------------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
struct Bad<T: Clone + Debug + PartialEq> { /* ... */ }

Исключения
Есть три исключения, когда требуются границы признаков для структур:

- Структура данных относится к ассоциированному типу признака.
- Граница есть ?Sized.(потому что атоматически компилятор ставит Sized)
- В структуре данных есть Drop имплант, требующий границ признака. 
 В настоящее время Rust требует, чтобы Drop в структуре данных присутствовали все границы признаков в impl.
```

### Уберите не нужные границы

Поднять границы признаков для расширения возможностей использования типа

```
Вы должны попытаться максимально поднять границы признаков (особенно в коде библиотеки), поскольку это расширяет возможности использования типа.
// Плохо -------------------------------------------------------------------------------------

#[derive(Clone)]
struct Loader<K, V> {
    state: Arc<Mutex<State<K, V>>>,
}
struct My;

let loader: Loader<My, My> = ..;
let copy = loader.clone(); // compile error as `My` doesn't impl `Clone`

Это происходит из-за того, что #[derive(Clone)] применяются к `K: Clone` и `V: Clone` в производном коде, несмотря на то, что они вообще не нужны

// Хорошо -------------------------------------------------------------------------------------
Предоставляя ручную реализацию, мы можем `Loader<My, My>` без проблем клонировать значения типа:

struct Loader<K, V> {
    state: Arc<Mutex<State<K, V>>>,
}
// Ручная реализация используется для исключения применения ненужных границ клонирования.
impl<K, V> Clone for Loader<K, V> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

let loader: Loader<My, My> = ..;
let copy = loader.clone(); // it compiles now!
```
 
[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_3_bound_impl)

[Зацепление связанность](https://en.wikipedia.org/wiki/Coupling_(computer_programming))