/*
С помощью классов создать свой язык для гибкости и простоты использования пользователем

Interpreter - поведенческий шаблон проектирования, решающий часто встречающуюся, но подверженную изменениям, задачу.
Также известен как Little (Small) Language
Сравнить данный паттерн можно с тем, как вы закладываете часто используемые действия в сокращенный набор слов, чтобы сам «интерпретатор» потом превратил этот набор в более комплексные осмысленные действия.

Шаблон интерпретатора используется для определения грамматического представления языка и предоставляет интерпретатор для работы с этой грамматикой.
Он предоставляет механизм интерпретации и оценки языковых выражений путем определения грамматики и интерпретатора.
Эффективно используя шаблон «Интерпретатор», вы можете добиться расширяемости, модульности кода и гибкости языковой интерпретации,
что в конечном итоге повышает функциональность и удобство использования ваших приложений языковой обработки.

Интерпретатор полагается на рекурсивный обход шаблона Composite для интерпретации «предложений», которые его просят обработать.

Эмпирические правила
Если рассматривать его в наиболее общей форме (т.е. операцию, распределенную по иерархии классов на основе шаблона Composite),
почти каждое использование шаблона Composite будет также содержать шаблон Interpreter.
Но шаблон Интерпретатора следует зарезервировать для тех случаев, когда вы хотите рассматривать эту иерархию классов как определение языка.
Interpreter может использовать State для определения контекстов синтаксического анализа.
Абстрактное синтаксическое дерево Интерпретатора является составным (поэтому также применимы Iterator и Visitor).
Terminal Expression в абстрактном синтаксическом дереве Interpreter могут использоваться совместно с Flyweight.
Шаблон не касается синтаксического анализа. Когда грамматика очень сложна, более подходящими являются другие методы (например, синтаксический анализатор).


Interpreter предоставляет механизм интерпретации и оценки языковых выражений путем определения грамматики и интерпретатора.
Шаблон представляет элементы языка как классы и использует рекурсивные алгоритмы для вычисления выражений.
Это способствует расширяемости и позволяет добавлять новые грамматические правила или языковые конструкции.

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


[Abstract Syntax Tree, AST](https://ru.wikipedia.org/wiki/%D0%90%D0%B1%D1%81%D1%82%D1%80%D0%B0%D0%BA%D1%82%D0%BD%D0%BE%D0%B5_%D1%81%D0%B8%D0%BD%D1%82%D0%B0%D0%BA%D1%81%D0%B8%D1%87%D0%B5%D1%81%D0%BA%D0%BE%D0%B5_%D0%B4%D0%B5%D1%80%D0%B5%D0%B2%D0%BE)
Абстрактное синтаксическое дерево (АСД, англ. abstract syntax tree, AST) — конечное помеченное ориентированное дерево,
в котором внутренние вершины сопоставлены (помечены) с операторами языка программирования, а листья — с соответствующими
операндами. Таким образом, листья являются пустыми операторами и представляют только переменные и константы.

[Обработка древовидных структур и унифицированное AST](https://habr.com/ru/companies/pt/articles/210060/)
*/

/// Интерпретатор asm синтаксиса
use dummy_interpreter_asm::*;
mod dummy_interpreter_asm {
    use std::collections::HashMap;

    // Terminal Expressions
    // обеспечивает логику интерпретации числового значения
    #[derive(Clone, Debug)]
    pub enum TerminalTypeExpressions {
        Number(usize),
        Str(String),
    }

    #[derive(Debug)]
    pub struct Context {
        stack: Vec<TerminalTypeExpressions>,
        map: HashMap<String, TerminalTypeExpressions>,
    }
    impl Context {
        pub fn new() -> Self {
            Self {
                stack: vec![],
                map: HashMap::new(),
            }
        }
        fn head(&mut self) -> Option<TerminalTypeExpressions> {
            if self.stack.len() > 0 {
                return self.stack.pop();
            }
            None
        }
        fn add(&mut self, t: TerminalTypeExpressions) {
            self.stack.push(t);
        }
        fn size(&self) -> usize {
            self.stack.len()
        }
        fn get_variable(&self, var_name: &str) -> Option<TerminalTypeExpressions> {
            if self.map.contains_key(var_name) {
                return Some(self.map.get(var_name).unwrap().clone());
            }
            None
        }
        fn create_variable(&mut self, var_name: &str, val: TerminalTypeExpressions) {
            self.map.insert(var_name.to_string(), val);
        }
    }

    // Abstract Expression
    pub trait AbstractExpression {
        fn interpret(&self, ctx: &mut Context);
    }

    // Non-Terminal Expression
    // предоставляет логику интерпретации для инструкции внесения в стек данных
    pub struct LoadDataExpression {
        data: TerminalTypeExpressions,
    }
    impl LoadDataExpression {
        pub fn new(data: TerminalTypeExpressions) -> Self {
            Self { data }
        }
    }
    impl AbstractExpression for LoadDataExpression {
        fn interpret(&self, ctx: &mut Context) {
            ctx.add(self.data.clone());
        }
    }

    // Non-Terminal Expression
    // предоставляет логику интерпретации для выражения присвоения переменной
    pub struct WriteVarExpression {
        data: TerminalTypeExpressions,
    }
    impl WriteVarExpression {
        pub fn new(data: TerminalTypeExpressions) -> Self {
            Self { data }
        }
    }
    impl AbstractExpression for WriteVarExpression {
        fn interpret(&self, ctx: &mut Context) {
            match self.data {
                TerminalTypeExpressions::Str(ref key) => {
                    if let Some(b) = ctx.head() {
                        match b {
                            TerminalTypeExpressions::Number(_) => {
                                ctx.create_variable(key, b);
                            }
                            TerminalTypeExpressions::Str(_) => {
                                ctx.create_variable(key, b);
                            }
                        }
                    } else {
                        panic!("WriteVarExpression:stack empty")
                    }
                }
                _ => {
                    panic!("WriteVarExpression:key must be a string")
                }
            }
        }
    }

    // Non-Terminal Expression
    // предоставляет логику интерпретации для выгрузки переменных в стек
    pub struct ReadVarExpression {
        data: TerminalTypeExpressions,
    }
    impl ReadVarExpression {
        pub fn new(data: TerminalTypeExpressions) -> Self {
            Self { data }
        }
    }
    impl AbstractExpression for ReadVarExpression {
        fn interpret(&self, ctx: &mut Context) {
            match self.data {
                TerminalTypeExpressions::Str(ref key) => {
                    if let Some(val) = ctx.get_variable(key) {
                        ctx.add(val);
                    } else {
                        panic!("ReadVarExpression:stack empty");
                    }
                }
                _ => {
                    panic!("ReadVarExpression:key must be a string")
                }
            }
        }
    }

    // Non-Terminal Expression
    // предоставляет логику интерпретации для выражения сложения
    pub struct AddExpression;
    impl AbstractExpression for AddExpression {
        fn interpret(&self, ctx: &mut Context) {
            if ctx.size() > 1 {
                if let Some(left) = ctx.head() {
                    if let Some(right) = ctx.head() {
                        match (left, right) {
                            (
                                TerminalTypeExpressions::Str(s1),
                                TerminalTypeExpressions::Str(s2),
                            ) => {
                                ctx.add(TerminalTypeExpressions::Str(format!("{s1}{s2}")));
                            }
                            (
                                TerminalTypeExpressions::Number(n1),
                                TerminalTypeExpressions::Number(n2),
                            ) => {
                                let (res, is_err) = n1.overflowing_add(n2);
                                if is_err {
                                    panic!("AddExpression:overflowing add");
                                }
                                ctx.add(TerminalTypeExpressions::Number(res));
                            }
                            _ => {
                                panic!("AddExpression:data type mismatch");
                            }
                        }
                    }
                }
            } else {
                panic!("AddExpression:stack empty");
            }
        }
    }

    // Non-Terminal Expression
    // предоставляет логику интерпретации для выражения произведения
    pub struct MultiplyExpression;
    impl AbstractExpression for MultiplyExpression {
        fn interpret(&self, ctx: &mut Context) {
            if ctx.size() > 1 {
                if let Some(left) = ctx.head() {
                    if let Some(right) = ctx.head() {
                        match (left, right) {
                            (
                                TerminalTypeExpressions::Str(s1),
                                TerminalTypeExpressions::Str(s2),
                            ) => {
                                ctx.add(TerminalTypeExpressions::Str(format!("{s1}{s2}")));
                            }
                            (
                                TerminalTypeExpressions::Number(n1),
                                TerminalTypeExpressions::Number(n2),
                            ) => {
                                let (res, is_err) = n1.overflowing_mul(n2);
                                if is_err {
                                    panic!("Multiply:overflowing mul");
                                }
                                ctx.add(TerminalTypeExpressions::Number(res));
                            }
                            _ => {
                                panic!("Multiply:data type mismatch");
                            }
                        }
                    }
                }
            } else {
                panic!("Multiply:stack empty");
            }
        }
    }

    pub struct Client {
        client_stack: Vec<Box<dyn AbstractExpression>>,
        ctx: Context,
    }
    impl Client {
        pub fn new(ctx: Context) -> Self {
            Self {
                client_stack: Vec::new(),
                ctx,
            }
        }
        pub fn add(&mut self, exp: Box<dyn AbstractExpression>) {
            self.client_stack.push(exp);
        }
        pub fn interpret(&mut self) {
            for exp in self.client_stack.iter() {
                exp.interpret(&mut self.ctx);
            }
        }
        pub fn ret_value_val(&mut self, var_name: &str) -> Option<TerminalTypeExpressions> {
            self.ctx.get_variable(var_name)
        }
        pub fn ret_value_stack(&mut self) -> Option<TerminalTypeExpressions> {
            self.ctx.head()
        }
    }
}

// cargo run --example p_behavior_interpreter
fn main() {
    /*
    let input ="
        LOAD DATA 2
        WRITE VARIABLE ‘a’

        LOAD VALIABLE 3
        WRITE VARIABLE ‘b’

        READ VARIABLE ‘a’
        LOAD VALIABLE 1
        ADD

        READ VARIABLE ‘b’
        MULTIPLY

        RETURN_VALUE";
    */

    // TODO: ради удобства, все инструкции упаковать в enum и реализовать FromStr в Expressions
    let ctx: Context = Context::new();
    let mut client: Client = Client::new(ctx);

    let load_exp = LoadDataExpression::new(TerminalTypeExpressions::Number(2));
    client.add(Box::new(load_exp));

    let a = TerminalTypeExpressions::Str(String::from("a"));
    let write_exp = WriteVarExpression::new(a.clone());
    client.add(Box::new(write_exp));

    let load_exp = LoadDataExpression::new(TerminalTypeExpressions::Number(3));
    client.add(Box::new(load_exp));
    let b = TerminalTypeExpressions::Str(String::from("b"));
    let write_exp = WriteVarExpression::new(b.clone());
    client.add(Box::new(write_exp));

    let read_exp = ReadVarExpression::new(a.clone());
    client.add(Box::new(read_exp));
    let load_exp = LoadDataExpression::new(TerminalTypeExpressions::Number(1usize));
    client.add(Box::new(load_exp));
    let add_exp = AddExpression;
    client.add(Box::new(add_exp));

    let read_exp = ReadVarExpression::new(b);
    client.add(Box::new(read_exp));
    let mul_exp = MultiplyExpression;
    client.add(Box::new(mul_exp));

    client.interpret();

    println!("stack:{:?}", client.ret_value_stack());
    println!("a:{:?}", client.ret_value_val("a"));
    println!("b:{:?}", client.ret_value_val("b"));
}
