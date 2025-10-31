/*
Пример шаблона проектирования декоратора
Это все равно, что добавить сырную начинку в пиццу или добавить начинку из грецких орехов и шоколада в мороженое. 
Вы можете добавить в пиццу любой сыр или соус непосредственно перед подачей клиенту, но основа пиццы останется неизменной.

Точно так же вы можете заказать грецкие орехи или вишню в ванильном мороженом, 
а можете передумать и попросить добавить только шоколадную начинку и вишневый соус. 
База ванильного мороженого не изменится, и вы можете добавлять к мороженому любую начинку, какую захотите.

Если в группе 20 человек, они могут заказать 20 ванильных мороженых с разными начинками, орехами и фруктами во время подачи.
*/

// Decorator pattern

pub fn decorator() {
    let obj = BaseObject { value: 100 };
    process(&obj);

    let decorator_obj = DecoratorObject { base: obj, more_value: 999 };
    process(&decorator_obj);

    decorator_obj.do_something_more();
}

trait Component {
    fn do_something(&self);
}

trait Decorator: Component {
    fn do_something_more(&self);
}

struct BaseObject {
    value: usize,
}

impl Component for BaseObject {
    fn do_something(&self) {
        println!("calculate value: {}", self.value);
    }
}

struct DecoratorObject {
    base: BaseObject,
    more_value: usize,
}

impl Component for DecoratorObject {
    fn do_something(&self) {
        self.base.do_something()
    }
}

impl Decorator for DecoratorObject {
    fn do_something_more(&self) {
        println!("calculate more: {}", self.more_value);
    }
}

fn process(c: &Component) {
    c.do_something();
}