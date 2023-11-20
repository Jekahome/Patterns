// Данные, которые мы не хотим изменять и поэтому будем их посещать
mod ast {
    pub enum Stmt {
        Expr(Expr),
        Let(Name, Expr),
    }

    pub struct Name {
        value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}

// The abstract visitor
// Интерфейс для всех типов используемых данных
mod visit {
    use super::*;
    use ast::*;

    pub trait Visitor<T> {
        fn visit_name(&mut self, n: &Name) -> T;
        fn visit_stmt(&mut self, s: &Stmt) -> T;
        fn visit_expr(&mut self, e: &Expr) -> T;
    }
}

use ast::*;
use visit::*;

// Пример конкретной реализации — обход AST, интерпретирующий его как код.
pub struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 {
        panic!()
    } // нет смысла String приводить к i64
    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(), // нет смысла String приводить к i64
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}

pub fn walk_expr(visitor: &mut Interpreter, e: &Expr) {
    match *e {
        Expr::IntLit(_) => {}
        Expr::Add(ref lhs, ref rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
        Expr::Sub(ref lhs, ref rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
    }
}

// cargo run --bin visitor -- --example main.rs
fn main() {
    let exp = Expr::Add(Box::new(Expr::IntLit(8)), Box::new(Expr::IntLit(3)));
    let mut interpreter = Interpreter;
    let res = interpreter.visit_expr(&exp);
    println!("{}", res);

    let mut interpreter = Interpreter;
    let exp = Expr::Add(Box::new(Expr::IntLit(8)), Box::new(Expr::IntLit(3)));
    walk_expr(&mut interpreter, &exp);
}
