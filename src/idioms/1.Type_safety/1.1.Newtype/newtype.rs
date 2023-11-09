/// 1.1 Newtype

mod no_correct{
    mod user{
        struct User{
            id:i32,
            age:i32,
            name:String
        }
    }

    fn get_user(id:i32) -> User{
        // search db ...
        User{id:id,age:0,name:"".into()}
    }

    fn main(){
        let age:i32 = 68;
        let user:User = get_user(age);// Некорректное использование ф-ции `get_user`
        // Вместо `id` передали `age`
    }
}

mod correct{
    mod user{
        struct UserId(i32);
        impl UserId{
            fn new(id:i32) -> Option<Self>{
                // TODO:валидация ...
                Some(Self(id))
            }
        }
        struct UserAge(i32);
        struct UserName(String);
        struct User{
            id:UserId,
            age:UserAge,
            name:UserName
        }
    }
    fn get_user(id:user::UserId) -> user::User{
        // search db ...
        user::User{id:id,age:user::UserAge(0),name:user::UserName("".into())}
    }
    fn main(){
        let age:user::UserId = user::UserId::new(68).expect("Invalid UserId");
        let user:user::User = get_user(age);// теперь ф-цию `get_user` мы не сможем использовать некорректно
    }
}
