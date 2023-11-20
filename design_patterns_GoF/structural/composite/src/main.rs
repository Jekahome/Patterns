/*
`Pattern Composite` позволяет объединять объекты в древовидную структуру и работать с ней, как если бы это был отдельный объект.
Замечательной особенностью `Pattern Composite` является возможность рекурсивного запуска методов по всей древовидной структуре и суммирования результатов.
Позволяя одинаково трактовать индивидуальные и составные объекты.

Использование `Pattern Composite` имеет смысл только в том случае, если базовую модель вашего приложения можно представить в виде дерева.
Решает проблему легкого доступа/обхода составных элементов

Пример:

Давайте попробуем понять шаблон Composite на примере файловой системы операционной системы.
В файловой системе существует два типа объектов: файлы и папки. Бывают случаи, когда с файлами и папками следует обращаться одинаково.
Вот тут-то и пригодится шаблон Composite.
Fileи Directoryоба trait Componentимеют один searchметод.
Для файла он просто просмотрит содержимое файла; для папки он просмотрит все файлы этой папки, чтобы найти это ключевое слово.
*/

pub trait Component {
    fn search(&self, keyword: &str);
}

pub mod dummy_file {
    use super::Component;

    pub struct File {
        name: &'static str,
    }

    impl File {
        pub fn new(name: &'static str) -> Self {
            Self { name }
        }
    }

    impl Component for File {
        fn search(&self, keyword: &str) {
            println!("Searching for keyword {} in file {}", keyword, self.name);
        }
    }
}

pub mod dummy_folder {
    use super::Component;

    pub struct Folder {
        name: &'static str,
        components: Vec<Box<dyn Component>>,
    }

    impl Folder {
        pub fn new(name: &'static str) -> Self {
            Self {
                name,
                components: vec![],
            }
        }

        pub fn add(&mut self, component: impl Component + 'static) {
            self.components.push(Box::new(component));
        }
    }

    impl Component for Folder {
        fn search(&self, keyword: &str) {
            println!(
                "Searching recursively for keyword {} in folder {}",
                keyword, self.name
            );

            for component in self.components.iter() {
                component.search(keyword);
            }
        }
    }
}

// cargo run --bin composite
fn main() {
    let file1 = dummy_file::File::new("File 1");
    let file2 = dummy_file::File::new("File 2");
    let file3 = dummy_file::File::new("File 3");

    let mut folder1 = dummy_folder::Folder::new("Folder 1");
    folder1.add(file1);

    let mut folder2 = dummy_folder::Folder::new("Folder 2");
    folder2.add(file2);
    folder2.add(file3);
    folder2.add(folder1);

    folder2.search("rose");
}
