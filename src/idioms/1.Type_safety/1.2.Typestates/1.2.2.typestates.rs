#![allow(dead_code)]
///Вариант основан на преобразованим From
use std::string::String;

mod post {
    #[derive(Clone)]
    pub struct Id(pub u64);

    #[derive(Clone)]
    pub struct Title(pub String);

    #[derive(Clone)]
    pub struct Body(pub String);
}

mod user {
    #[derive(Clone)]
    pub struct Id(pub u64);
}

/// Состояния
struct New {
    post_id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}
struct Unmoderated {
    post_id: post::Id,
}
struct Published {
    post_id: post::Id,
}
struct Deleted {
    post_id: post::Id,
}

/// New --  Unmoderated
impl From<New> for Unmoderated {
    fn from(_val: New) -> Unmoderated {
        Unmoderated {
            post_id: _val.post_id,
        }
    }
}

/// Unmoderated --  Published
impl From<Unmoderated> for Published {
    fn from(_val: Unmoderated) -> Published {
        Published {
            post_id: _val.post_id,
        }
    }
}

/// Unmoderated --  Deleted
impl From<Unmoderated> for Deleted {
    fn from(_val: Unmoderated) -> Deleted {
        Deleted {
            post_id: _val.post_id,
        }
    }
}

/// Published --  Deleted
impl From<Published> for Deleted {
    fn from(_val: Published) -> Deleted {
        Deleted {
            post_id: _val.post_id,
        }
    }
}

fn new(post_id: post::Id, user_id: user::Id, title: post::Title, body: post::Body) -> New {
    New {
        post_id: post_id,
        user_id: user_id,
        title: title,
        body: body,
    }
}

fn publish(post: New) -> Unmoderated {
    println!("New -- \"publish()\" --> Unmoderated");
    //Unmoderated::from(post)
    post.into()
}
fn allow(post: Unmoderated) -> Published {
    println!("Unmoderated -- \"allow()\" --> Published");
    //Published::from(post)
    post.into()
}

fn deny(post: Unmoderated) -> Deleted {
    println!("Unmoderated -- \"deny()\" --> Deleted");
    //Deleted::from(post)
    post.into()
}

fn delete(post: Published) -> Deleted {
    println!("Published -- \"delete()\" --> Deleted");
    // Deleted::from(post)
    post.into()
}

fn main() {
    let in_new: New = new(
        post::Id(1),
        user::Id(1),
        post::Title(String::from("Title")),
        post::Body(String::from("Body")),
    );
    let in_unmoderated: Unmoderated = publish(in_new);
    let in_published: Published = allow(in_unmoderated);
    let _in_delete: Deleted = delete(in_published);

    println!("\n");

    let in_new: New = new(
        post::Id(1),
        user::Id(2),
        post::Title(String::from("Title")),
        post::Body(String::from("Body")),
    );
    let in_unmoderated: Unmoderated = publish(in_new);
    let _in_delete: Deleted = deny(in_unmoderated);
}
