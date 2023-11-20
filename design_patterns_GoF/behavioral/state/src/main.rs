/*
В зависимости от состояния изменяется поведение

`Pattern State` невозможно рассматривать в отрыве от концепции машины состояний, также известной как стейт-машина или конечный автомат
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
При использовании Strategy выбор алгоритма достаточно стабилен. При использовании State изменение состояния объекта «контекст» приводит к
выбору объектов стратегии из «палитры».

Flyweight объясняет, когда и как можно совместно использовать объекты State.
*/

// Больше примеров в idioms/Typestates
use std::marker::PhantomData;

struct SoldState; // Состояние продано
struct SoldOutState; // Состояние все продано, нет шариков
struct NoQuarterState; // Состояние монетка не внесена
struct HasQuarterState; // Состояние монетка внесена

struct GumballMashine<S> {
    count: i32,
    state: PhantomData<S>,
}

impl GumballMashine<SoldState> {
    fn realese_ball(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }
    fn get_count(&self) -> i32 {
        self.count
    }
}

impl std::fmt::Debug for GumballMashine<SoldState> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GumballMashine<SoldState> {{ count: {} }}", self.count)
    }
}
impl std::fmt::Debug for GumballMashine<SoldOutState> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "GumballMashine<SoldOutState> {{ count: {} }}",
            self.count
        )
    }
}
impl std::fmt::Debug for GumballMashine<NoQuarterState> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "GumballMashine<NoQuarterState> {{ count: {} }}",
            self.count
        )
    }
}
impl std::fmt::Debug for GumballMashine<HasQuarterState> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "GumballMashine<HasQuarterState> {{ count: {} }}",
            self.count
        )
    }
}

//Переход из состояния монетка не внесена в состояние монетка внесена
/// NoQuarterState -- HasQuarterState
impl From<GumballMashine<NoQuarterState>> for GumballMashine<HasQuarterState> {
    fn from(_val: GumballMashine<NoQuarterState>) -> GumballMashine<HasQuarterState> {
        GumballMashine {
            count: _val.count,
            state: PhantomData,
        }
    }
}
//Переход из состояния монетка внесена в состояние монетка не внесена
/// NoQuarterState -- HasQuarterState
impl From<GumballMashine<HasQuarterState>> for GumballMashine<NoQuarterState> {
    fn from(_val: GumballMashine<HasQuarterState>) -> GumballMashine<NoQuarterState> {
        GumballMashine {
            count: _val.count,
            state: PhantomData,
        }
    }
}
//Переход из состояния монетка внесена в состояние продано
/// HasQuarterState -- SoldState
impl From<GumballMashine<HasQuarterState>> for GumballMashine<SoldState> {
    fn from(_val: GumballMashine<HasQuarterState>) -> GumballMashine<SoldState> {
        GumballMashine {
            count: _val.count,
            state: PhantomData,
        }
    }
}
//Переход из состояния продано в состояние шариков больше нет
/// SoldState -- SoldOutState
impl From<&mut GumballMashine<SoldState>> for GumballMashine<SoldOutState> {
    fn from(_val: &mut GumballMashine<SoldState>) -> GumballMashine<SoldOutState> {
        GumballMashine {
            count: _val.count,
            state: PhantomData,
        }
    }
}
//Переход из состояния продано в состояние монетка не внесена
/// SoldState -- NoQuarterState
impl From<&mut GumballMashine<SoldState>> for GumballMashine<NoQuarterState> {
    fn from(_val: &mut GumballMashine<SoldState>) -> GumballMashine<NoQuarterState> {
        GumballMashine {
            count: _val.count,
            state: PhantomData,
        }
    }
}

//--------------------------------------------------------------------------------------------------
fn new_gumball(count: i32) -> GumballMashine<NoQuarterState> {
    let gumball: GumballMashine<NoQuarterState> = GumballMashine {
        count,
        state: PhantomData,
    };
    gumball
}

fn insert_quarter(gumball: GumballMashine<NoQuarterState>) -> GumballMashine<HasQuarterState> {
    println!("Монетка внесена");
    gumball.into()
}
fn eject_quarter(gumball: GumballMashine<HasQuarterState>) -> GumballMashine<NoQuarterState> {
    println!("Возврат монетки");
    gumball.into()
}
fn turn_crank(gumball: GumballMashine<HasQuarterState>) -> GumballMashine<SoldState> {
    println!("Получение шарика");
    gumball.into()
}
fn dispense(
    gunball: &mut GumballMashine<SoldState>,
) -> Result<GumballMashine<NoQuarterState>, GumballMashine<SoldOutState>> {
    gunball.realese_ball();
    if gunball.get_count() > 0 {
        return Ok(gunball.into());
    } else {
        return Err(gunball.into());
    }
}

// cargo run --bin state
fn main() {
    /*
     let gumball_no_quarter_state = new_gumball(5);
     let gumball_has_quarter_state = insert_quarter(gumball_no_quarter_state);
     //let gumball_no_quarter_state = eject_quarter(gumball_has_quarter_state);
     let mut gumball_sold_state = turn_crank(gumball_has_quarter_state);
     let gumball  = dispense(&mut gumball_sold_state);
    */

    let gumball_no_quarter_state = new_gumball(3);
    let gumball_has_quarter_state = insert_quarter(gumball_no_quarter_state); //бросить монетку
    let mut gumball_sold_state = turn_crank(gumball_has_quarter_state); //опустить рычаг
    let _gumball = dispense(&mut gumball_sold_state);
    println!("{:?}\n", gumball_sold_state);

    let gumball_no_quarter_state = new_gumball(3);
    let gumball_has_quarter_state = insert_quarter(gumball_no_quarter_state); //бросить монетку
    let gumball_no_quarter_state = eject_quarter(gumball_has_quarter_state); //вернуть монетку
    println!("Опустить рычаг не получиться, нет реализованной функции!");
    println!("{:?}\n", gumball_no_quarter_state);

    println!("Бросить монетку два раза не получиться, первый вызов берет владение переменной!\n");

    let gumball_no_quarter_state = new_gumball(3);
    let gumball_has_quarter_state = insert_quarter(gumball_no_quarter_state); //бросить монетку
    println!("{:?}", gumball_has_quarter_state);
}
/*
Монетка внесена
Получение шарика
GumballMashine<SoldState> { count: 2 }

Монетка внесена
Возврат монетки
Опустить рычаг не получиться, нет реализованной функции!
GumballMashine<NoQuarterState> { count: 3 }

Бросить монетку два раза не получиться, первый вызов берет владение переменной!

Монетка внесена
GumballMashine<HasQuarterState> { count: 3 }
*/
