/*
`Template method` - определяет скелет алгоритма, перекладывая ответственность за некоторые его шаги на подклассы.
Паттерн позволяет подклассам переопределять шаги алгоритма, не меняя его общей структуры.
Идея состоит в том, чтобы позволить наследникам абстрактного
шаблона переопределить поведение алгоритмов родителя.

Это простой способ изолировать логику в конкретные классы и уменьшить копипаст,
поэтому вы повсеместно встретите его в том или ином виде.

`Template method` предлагает разбить алгоритм на последовательность шагов, описать эти шаги в отдельных методах и
вызывать их в одном шаблонном методе друг за другом.

Это позволит подклассам переопределять некоторые шаги алгоритма, оставляя без изменений его структуру и
остальные шаги, которые для этого подкласса не так важны.
*/

trait Drink {
    // Общий алгоритм приготовления, а отличные детали переопределенны
    // Шаблонный метод определяет скелет алгоритма.
    fn template_method(&mut self) {
        self.step_cook_1();
        self.step_cook_2();
        self.step_cook_3();
        self.step_cook_4();
    }
    fn step_cook_2(&mut self);
    fn step_cook_4(&mut self);

    fn step_cook_1(&mut self) {
        println!("Drink step_cook_1");
    }
    fn step_cook_3(&mut self) {
        println!("Drink step_cook_3");
    }
}

struct Coffe;
impl Drink for Coffe {
    fn step_cook_2(&mut self) {
        println!("Coffe step_cook_2");
    }
    fn step_cook_4(&mut self) {
        println!("Coffe step_cook_4");
    }
}

struct Tea;
impl Drink for Tea {
    fn step_cook_1(&mut self) {
        println!("Tea step_cook_1");
    }
    fn step_cook_2(&mut self) {
        println!("Tea step_cook_2");
    }
    fn step_cook_4(&mut self) {
        println!("Tea step_cook_4");
    }
}

fn cook<T: Drink>(obj: Box<&mut T>) {
    obj.template_method();
    println!("");
}

// cargo run --bin template_method
fn main() {
    cook(Box::new(&mut (Tea)));
    cook(Box::new(&mut (Coffe)));
}

/*
Tea step_cook_1
Tea step_cook_2
Drink step_cook_3
Tea step_cook_4

Drink step_cook_1
Coffe step_cook_2
Drink step_cook_3
Coffe step_cook_4
*/
