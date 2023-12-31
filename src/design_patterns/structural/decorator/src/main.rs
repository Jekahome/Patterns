#![allow(dead_code)]
#![allow(unreachable_patterns)]

/*
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

*/

// Конструктор коктейлей
#[derive(Clone, Copy)]
enum Size {
    TALL,
    GRANDE,
    VENTI,
}
impl Default for Size {
    fn default() -> Size {
        Size::TALL
    }
}

// Напиток
trait Beverage {
    fn get_description(&self) -> String;
    fn set_size(&mut self, size: Size);
    fn get_size(&self) -> Size;
    fn cost(&self) -> f32;
}

// Виды напитка ------------------------------------------------------------------------------------
struct HouseBlend {
    size: Size,
    description: String,
    default_cost: f32,
}
impl Default for HouseBlend {
    fn default() -> Self {
        Self {
            size: Size::default(),
            description: String::from("House Blend Coffee"),
            default_cost: 0.89f32,
        }
    }
}
impl HouseBlend {
    fn new(size: Size, default_cost: f32, description: String) -> Self {
        Self {
            size,
            description,
            default_cost,
        }
    }
    fn new_with_size_default(size: Size) -> Self {
        let def = Self::default();
        Self {
            size,
            description: def.description,
            default_cost: def.default_cost,
        }
    }
}
impl Beverage for HouseBlend {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn get_size(&self) -> Size {
        self.size
    }
    fn cost(&self) -> f32 {
        match self.get_size() {
            Size::TALL => self.default_cost,
            Size::GRANDE => self.default_cost + 0.35,
            Size::VENTI => self.default_cost + 0.40,
            _ => self.default_cost,
        }
    }
}

// Виды напитка
struct DarkRoast {
    size: Size,
    description: String,
    default_cost: f32,
}
impl Default for DarkRoast {
    fn default() -> Self {
        Self {
            size: Size::default(),
            description: String::from("DarkRoast"),
            default_cost: 1.0f32,
        }
    }
}
impl DarkRoast {
    fn new(size: Size, default_cost: f32, description: String) -> Self {
        Self {
            size,
            description,
            default_cost,
        }
    }
    fn new_with_size_default(size: Size) -> Self {
        let def = Self::default();
        Self {
            size,
            description: def.description,
            default_cost: def.default_cost,
        }
    }
}
impl Beverage for DarkRoast {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn get_size(&self) -> Size {
        self.size
    }
    fn cost(&self) -> f32 {
        match self.get_size() {
            Size::TALL => self.default_cost,
            Size::GRANDE => 0.35,
            Size::VENTI => 0.40,
            _ => self.default_cost,
        }
    }
}

// Виды напитка
struct Espresso {
    size: Size,
    description: String,
    default_cost: f32,
}
impl Default for Espresso {
    fn default() -> Self {
        Self {
            size: Size::default(),
            description: String::from("Espresso"),
            default_cost: 1.99f32,
        }
    }
}
impl Espresso {
    fn new(size: Size, default_cost: f32, description: String) -> Self {
        Self {
            size,
            description,
            default_cost,
        }
    }
    fn new_with_size_default(size: Size) -> Self {
        let def = Self::default();
        Self {
            size,
            description: def.description,
            default_cost: def.default_cost,
        }
    }
}
impl Beverage for Espresso {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn get_size(&self) -> Size {
        self.size
    }
    fn cost(&self) -> f32 {
        match self.get_size() {
            Size::TALL => self.default_cost,
            Size::GRANDE => self.default_cost + 0.40,
            Size::VENTI => self.default_cost + 0.45,
            _ => self.default_cost,
        }
    }
}

// Виды напитка
struct Decaf {
    size: Size,
    description: String,
    default_cost: f32,
}
impl Default for Decaf {
    fn default() -> Self {
        Self {
            size: Size::default(),
            description: String::from("Decaf"),
            default_cost: 1.99f32,
        }
    }
}
impl Decaf {
    fn new(size: Size, default_cost: f32, description: String) -> Self {
        Self {
            size,
            description,
            default_cost,
        }
    }
}
impl Beverage for Decaf {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn get_size(&self) -> Size {
        self.size
    }
    fn cost(&self) -> f32 {
        match self.get_size() {
            Size::TALL => self.default_cost,
            Size::GRANDE => self.default_cost + 0.35,
            Size::VENTI => self.default_cost + 0.40,
            _ => self.default_cost,
        }
    }
}

trait New {
    fn new(beverage: Box<dyn Beverage>) -> Self;
}

// Дополнения к напитку ----------------------------------------------------------------------------
struct Mocha {
    beverage: Box<dyn Beverage>,
}
impl New for Mocha {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}
impl Beverage for Mocha {
    fn get_description(&self) -> String {
        format!("{}, Mocha", self.beverage.get_description())
    }
    fn set_size(&mut self, size: Size) {
        self.beverage.set_size(size);
    }
    fn get_size(&self) -> Size {
        self.beverage.get_size()
    }
    fn cost(&self) -> f32 {
        let mut cost: f32 = self.beverage.cost();
        cost += match self.beverage.get_size() {
            Size::TALL => 0.20,
            Size::GRANDE => 0.35,
            Size::VENTI => 0.40,
            _ => 0.20,
        };
        cost
    }
}

// Дополнения к напитку
struct Milk {
    beverage: Box<dyn Beverage>,
}
impl New for Milk {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}
impl Beverage for Milk {
    fn get_description(&self) -> String {
        format!("{}, Milk", self.beverage.get_description())
    }
    fn set_size(&mut self, size: Size) {
        self.beverage.set_size(size);
    }
    fn get_size(&self) -> Size {
        self.beverage.get_size()
    }
    fn cost(&self) -> f32 {
        let mut cost: f32 = self.beverage.cost();
        cost += match self.beverage.get_size() {
            Size::TALL => 0.15,
            Size::GRANDE => 0.25,
            Size::VENTI => 0.30,
            _ => 0.15,
        };
        cost
    }
}

// Дополнения к напитку
struct Soy {
    beverage: Box<dyn Beverage>,
}
impl New for Soy {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}
impl Beverage for Soy {
    fn get_description(&self) -> String {
        format!("{}, Soy", self.beverage.get_description())
    }
    fn set_size(&mut self, size: Size) {
        self.beverage.set_size(size);
    }
    fn get_size(&self) -> Size {
        self.beverage.get_size()
    }
    fn cost(&self) -> f32 {
        let mut cost: f32 = self.beverage.cost();
        cost += match self.beverage.get_size() {
            Size::TALL => 0.10,
            Size::GRANDE => 0.15,
            Size::VENTI => 0.20,
            _ => 0.10,
        };
        cost
    }
}

// Дополнения к напитку
struct Whip {
    beverage: Box<dyn Beverage>,
}
impl New for Whip {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}
impl Beverage for Whip {
    fn get_description(&self) -> String {
        format!("{}, Whip", self.beverage.get_description())
    }
    fn set_size(&mut self, size: Size) {
        self.beverage.set_size(size);
    }
    fn get_size(&self) -> Size {
        self.beverage.get_size()
    }
    fn cost(&self) -> f32 {
        let mut cost: f32 = self.beverage.cost();
        cost += match self.beverage.get_size() {
            Size::TALL => 0.10,
            Size::GRANDE => 0.15,
            Size::VENTI => 0.20,
            _ => 0.10,
        };
        cost
    }
}

fn add<T: Beverage + New>(beverage: Box<dyn Beverage>) -> T {
    T::new(beverage)
}

fn main() {
    // Кофе Espresso
    let espresso: Espresso = Espresso::new_with_size_default(Size::VENTI);
    println!(
        "{:?} ${:?}",
        espresso.get_description(),
        espresso.cost() as f32
    );

    // Кофе с двойным шоколадом и взбитыми сливками
    let dark_roast: DarkRoast = DarkRoast::default();
    let mocha = add::<Mocha>(Box::new(dark_roast));
    let mocha = add::<Mocha>(Box::new(mocha));
    let milk = add::<Milk>(Box::new(mocha));
    println!("{:?} ${:?}", milk.get_description(), milk.cost() as f32);

    // Кофе "Домашняя смесь" с соей,шоколадом и взбитыми сливками
    let house_blend: HouseBlend = HouseBlend::new(Size::VENTI, 0.89, String::from("HouseBlend"));
    let soy = add::<Soy>(Box::new(house_blend));
    let mocha = add::<Mocha>(Box::new(soy));
    let whip = add::<Whip>(Box::new(mocha));
    println!("{:?} ${:?}", whip.get_description(), whip.cost() as f32);

    /*
    "Espresso" $2.44
    "DarkRoast, Mocha, Mocha, Milk" $1.55
    "HouseBlend, Soy, Mocha, Whip" $2.08
    */
}
