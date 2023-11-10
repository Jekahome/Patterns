#![allow(dead_code)]

/*
`Template Visitor` позволяет добавлять в программу новые операции, не изменяя разнородные классы объектов,
над которыми эти операции могут выполняться.
(т.е. все струтуры или перечисления остаюся нетронутыми `Industrial,Residential,Commercial,Build,LevelBuild`)

`Template Visitor` полезен везде, где вы хотите применить алгоритм к разнородным данным.
Если данные однородны, просто применяем один метод.
Использование объекта посетителя (а не функционального подхода) позволяет посетителю сохранять состояние и,
таким образом, передавать информацию между узлами.

Т.е. в идеале мы бы могли имплементироваться от трейта и просто вызывая обший для всех метод решить свою задачу, но мы в силу каких-то
причин так не можем делать.
Так-же, возможная причина, это неуместная логика в рамках этих структур или потенциальные изменения в новой требуемой логике.

`Template Fold` аналогичен `Template Visitor`, но создает новую версию посещенной структуры данных.
*/

/*
Проблема
Ваша команда разрабатывает приложение, работающее с геоданными в виде графа.
Узлами графа являются городские локации: памятники, театры, рестораны, важные предприятия и прочее.
Каждый узел имеет ссылки на другие, ближайшие к нему узлы.
Каждому типу узлов соответствует свой класс, а каждый узел представлен отдельным объектом.

Ваша задача — сделать экспорт этого графа в XML. Дело было бы плёвым, если бы вы могли редактировать классы узлов.
Достаточно было бы добавить метод экспорта в каждый тип узла, а затем, перебирая узлы графа, вызывать этот метод для каждого узла.
Благодаря полиморфизму, решение получилось бы изящным, так как вам не пришлось бы привязываться к конкретным классам узлов.
Но, к сожалению, классы узлов менять нельзя!
*/
use std::any::Any;

// Узлы
// for trait-object variants
struct Industrial(String);
struct Residential(Vec<u8>);
struct Commercial([i32; 5]);

// Узлы
// for enum variants
use build::*;
mod build {
    pub enum Build {
        EIndustrial(Box<LevelBuild>),
        EResidential(Box<LevelBuild>),
        ECommercial(Box<LevelBuild>),
    }
    pub enum LevelBuild {
        EFrameLevel(u8),
        ENone,
    }
}

// Aggregator
struct CLientCity {
    fields_1: Vec<Box<dyn Any>>,
    fields_2: Vec<Build>,
}

// Необходимо имея неоднородную структуру типов, использовать конкретный метод для этого типа
// В случае с коллекцией можем использовать только trait objects и как следствие downcast
// Но в случае с enum просто matching
impl CLientCity {
    fn export_trait_objects(&self, export_visitor: impl VisitorForTraitObject) {
        for item in self.fields_1.iter() {
            if let Some(obj) = item.downcast_ref::<Industrial>() {
                export_visitor.for_industrial(obj);
            } else if let Some(obj) = item.downcast_ref::<Residential>() {
                export_visitor.for_residential(obj);
            } else if let Some(obj) = item.downcast_ref::<Commercial>() {
                export_visitor.for_commercial(obj);
            }
        }
    }
    fn export_enum(&self, export_visitor: impl VisitorForEnum) {
        for item in self.fields_2.iter() {
            match item {
                Build::EIndustrial(level) => {
                    export_visitor.for_industrial(item);
                    export_visitor.for_level(level);
                }
                Build::EResidential(level) => {
                    export_visitor.for_residential(item);
                    export_visitor.for_level(level);
                }
                Build::ECommercial(level) => {
                    export_visitor.for_commercial(item);
                    export_visitor.for_level(level);
                }
            }
        }
    }
}

//------------------------------------------------------------------------------------------------------
trait VisitorForTraitObject {
    fn for_industrial(&self, obj: &Industrial);
    fn for_residential(&self, obj: &Residential);
    fn for_commercial(&self, obj: &Commercial);
}
struct ExportConcreteVisitor;
impl VisitorForTraitObject for ExportConcreteVisitor {
    fn for_industrial(&self, obj: &Industrial) {
        println!("trait-obj export for_industrial {}", obj.0);
    }
    fn for_residential(&self, obj: &Residential) {
        println!("trait-obj export for_residential {}", obj.0.len());
    }
    fn for_commercial(&self, obj: &Commercial) {
        println!("trait-obj export for_commercial {:?}", obj.0);
    }
}

//------------------------------------------------------------------------------------------------------
trait VisitorForEnum {
    fn for_industrial(&self, _e: &Build);
    fn for_residential(&self, _e: &Build);
    fn for_commercial(&self, _e: &Build);
    fn for_level(&self, _e: &LevelBuild);
}
struct ExportConcreteVisitor2;
impl VisitorForEnum for ExportConcreteVisitor2 {
    fn for_industrial(&self, _e: &Build) {
        println!("enum export for_industrial");
    }
    fn for_residential(&self, _e: &Build) {
        println!("enum export for_residential");
    }
    fn for_commercial(&self, _e: &Build) {
        println!("enum export for_commercial");
    }
    fn for_level(&self, _e: &LevelBuild) {
        println!("enum export for_level");
    }
}

// cargo run --example p_behavior_visitor
fn main() {
    let export_visitor = ExportConcreteVisitor;
    let export_visitor2 = ExportConcreteVisitor2;

    let fields_1: Vec<Box<dyn Any>> = {
        let some_city_object1 = Box::new(Industrial("Industrial".into()));
        let some_city_object2 = Box::new(Residential(vec![1]));
        let some_city_object3 = Box::new(Commercial([1, 2, 3, 4, 5]));
        vec![some_city_object1, some_city_object2, some_city_object3]
    };
    let fields_2: Vec<Build> = {
        vec![
            Build::EIndustrial(Box::new(LevelBuild::EFrameLevel(1))),
            Build::ECommercial(Box::new(LevelBuild::EFrameLevel(3))),
        ]
    };

    let nodes_city_object: CLientCity = CLientCity { fields_1, fields_2 };
    nodes_city_object.export_trait_objects(export_visitor);
    nodes_city_object.export_enum(export_visitor2);
}
