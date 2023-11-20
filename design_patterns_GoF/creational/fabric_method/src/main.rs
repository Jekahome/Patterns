/*
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

P.S. `Factory Method` - это идиома, а `Factory` - это паттерн? (book: Head First)
*/

use gui::{Button, DialogFactory};
pub mod gui {
    pub trait Button {
        fn render(&self);
        fn on_click(&self);
    }

    pub trait DialogFactory {
        type B: Button;
        /// The factory method. It must be overridden with a concrete implementation.
        fn create_button_factory_method(&self) -> Self::B;

        fn render(&self) {
            let button = self.create_button_factory_method();
            button.render();
            self.refresh();
        }
        fn refresh(&self) {
            println!("Dialog - Refresh");
        }
    }
}

use html_gui::*;
pub mod html_gui {
    use super::*;
    pub struct HtmlButton;

    impl Button for HtmlButton {
        fn render(&self) {
            println!("<button>Test Button</button>");
            self.on_click();
        }
        fn on_click(&self) {
            println!("Click! Button says - 'Hello World!'");
        }
    }

    pub struct HtmlConcreteFactory;

    impl DialogFactory for HtmlConcreteFactory {
        type B = HtmlButton;
        fn create_button_factory_method(&self) -> Self::B {
            HtmlButton
        }
    }
}

use windows_gui::*;
pub mod windows_gui {
    use super::*;

    pub struct WindowsButton;

    impl Button for WindowsButton {
        fn render(&self) {
            println!("Drawing a Windows button");
            self.on_click();
        }
        fn on_click(&self) {
            println!("Click! Hello, Windows!");
        }
    }

    pub struct WindowsConcreteFactory;

    impl DialogFactory for WindowsConcreteFactory {
        type B = WindowsButton;
        fn create_button_factory_method(&self) -> Self::B {
            WindowsButton
        }
    }
}

use linux_gui::*;
pub mod linux_gui {
    use super::*;

    pub struct LinuxButton;

    impl Button for LinuxButton {
        fn render(&self) {
            println!("Drawing a Linux button");
            self.on_click();
        }
        fn on_click(&self) {
            println!("Click! Hello, Linux!");
        }
    }

    pub struct LinuxConcreteFactory;

    impl DialogFactory for LinuxConcreteFactory {
        type B = LinuxButton;
        fn create_button_factory_method(&self) -> Self::B {
            LinuxButton
        }
    }
}

struct Client<B> {
    btn: B,
}
impl<B: Button> Client<B> {
    fn new(btn: B) -> Self {
        Self { btn }
    }
    fn use_btn(&self) {
        self.btn.render();
    }
}

enum Variants {
    Windows(Box<dyn DialogFactory<B = WindowsButton>>),
    Linux(Box<dyn DialogFactory<B = LinuxButton>>),
    Html(Box<dyn DialogFactory<B = HtmlButton>>),
}

fn use_client(dialog: Variants) {
    match dialog {
        Variants::Windows(dialog) => {
            let client = Client::new(dialog.create_button_factory_method());
            client.use_btn();
        }
        Variants::Linux(dialog) => {
            let client = Client::new(dialog.create_button_factory_method());
            client.use_btn();
        }
        Variants::Html(dialog) => {
            let client = Client::new(dialog.create_button_factory_method());
            client.use_btn();
        }
    }
}
// cargo run --bin fabric_method
fn main() {
    let mut line = String::new();
    println!("Enter type OS: (Windows, Linux, HTML (default: HTML): ");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    let dialog: Variants = match line.trim().to_lowercase().as_str() {
        "windows" => Variants::Windows(Box::new(WindowsConcreteFactory)),
        "linux" => Variants::Linux(Box::new(LinuxConcreteFactory)),
        "html" => Variants::Html(Box::new(HtmlConcreteFactory)),
        _ => Variants::Html(Box::new(HtmlConcreteFactory)),
    };
    use_client(dialog);

    /*
        if cfg!(windows) {
             ...
        }else if cfg!(target_os = "linux") {
             ...
        } else {
             ...
        }
    */
}
