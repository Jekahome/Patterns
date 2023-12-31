/*

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
*/

// Данные AST.
pub mod ast {
    #[derive(Debug)]
    pub enum Stmt {
        Expr(Box<Expr>),
        Let(Box<Name>, Box<Expr>),
    }

    #[derive(Debug)]
    pub struct Name {
        value: String,
    }
    impl Name {
        pub fn new(value: String) -> Self {
            Self { value }
        }
    }

    #[derive(Debug)]
    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}

// The abstract folder
pub mod fold {
    use crate::ast::*;
    pub trait Folder {
        // узелы просто возвращает сами узлы.
        fn fold_name(&mut self, n: Box<Name>) -> Box<Name> {
            n
        }
        // Создайте новый внутренний узел, сложив его дочерние элементы.
        fn fold_stmt(&mut self, s: Box<Stmt>) -> Box<Stmt> {
            match *s {
                Stmt::Expr(e) => Box::new(Stmt::Expr(self.fold_expr(e))),
                Stmt::Let(n, e) => Box::new(Stmt::Let(self.fold_name(n), self.fold_expr(e))),
            }
        }
        fn fold_expr(&mut self, e: Box<Expr>) -> Box<Expr> {
            e
        }
    }
}

use ast::*;
use fold::*;

// Пример конкретной реализации — переименовывает каждое имя в 'foo'.
struct Renamer;
impl Folder for Renamer {
    fn fold_name(&mut self, n: Box<Name>) -> Box<Name> {
        Box::new(Name::new("foo".to_owned()))
    }
    // Используйте методы по умолчанию для других узлов.
}

struct IntLitChange;
impl Folder for IntLitChange {
    fn fold_expr(&mut self, n: Box<Expr>) -> Box<Expr> {
        match *n {
            Expr::IntLit(v) if v > 10 => {
                let v = 0;
                Box::new(Expr::IntLit(v))
            }
            _ => n,
        }
    }
    // Используйте методы по умолчанию для других узлов.
}

// cargo run --example p_creational_fold
fn main() {
    let mut rename_logic = Renamer;
    let exp: Box<Stmt> = Box::new(Stmt::Let(
        Box::new(Name::new(String::from("ABCD"))),
        Box::new(Expr::IntLit(18)),
    ));
    let res: Box<Stmt> = rename_logic.fold_stmt(exp);
    println!("{:?}", *res); // Let(Name { value: "foo" }, IntLit(18))

    let mut intlit_logic = IntLitChange;
    let exp: Box<Stmt> = Box::new(Stmt::Let(
        Box::new(Name::new(String::from("ABCD"))),
        Box::new(Expr::IntLit(18)),
    ));
    let res: Box<Stmt> = intlit_logic.fold_stmt(exp);
    println!("{:?}", *res); // Let(Name { value: "ABCD" }, IntLit(0))
}
