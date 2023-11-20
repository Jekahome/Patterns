#![allow(dead_code)]
///Вариант основан на преобразованим From

/// Состояния
struct New;
struct Unmoderated;
struct Published;
struct Deleted;

/// New --  Unmoderated
impl From<New> for Unmoderated {
    fn from(_val: New) -> Unmoderated {
        Unmoderated
    }
}

/// Unmoderated --  Published
impl From<Unmoderated> for Published {
    fn from(_val: Unmoderated) -> Published {
        Published
    }
}

/// Unmoderated --  Deleted
impl From<Unmoderated> for Deleted {
    fn from(_val: Unmoderated) -> Deleted {
        Deleted
    }
}

/// Published --  Deleted
impl From<Published> for Deleted {
    fn from(_val: Published) -> Deleted {
        Deleted
    }
}

fn new() -> New {
    New
}

fn publish(a: New) -> Unmoderated {
    println!("New -- \"publish()\" --> Unmoderated");
    Unmoderated::from(a)
}
fn allow(a: Unmoderated) -> Published {
    println!("Unmoderated -- \"allow()\" --> Published");
    Published::from(a)
}

fn deny(a: Unmoderated) -> Deleted {
    println!("Unmoderated -- \"deny()\" --> Deleted");
    Deleted::from(a)
}

fn delete(a: Published) -> Deleted {
    println!("Published -- \"delete()\" --> Deleted");
    Deleted::from(a)
}

fn main() {
    let in_new: New = new();
    let in_unmoderated: Unmoderated = publish(in_new);
    let in_published: Published = allow(in_unmoderated);
    let _in_delete: Deleted = delete(in_published);

    println!("\n");

    let in_new: New = new();
    let in_unmoderated: Unmoderated = publish(in_new);
    let _in_delete: Deleted = deny(in_unmoderated);
}
