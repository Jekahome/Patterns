## Exhaustivity

Проверка на полноту вариантов.
Расширяя кодовую базу уже имеющихся перечислений и структур, есть шанс не учесть новые поля в коде, что может привести к ошибке во время выполнения. Для того чтобы отловить эту ошибку на этапе компиляции следует придерживаться следующих рекомендаций. Использовать полный список вариантов перечислений и пользоваться полями созданной струтурой,а не отдельными ее полями.

Используйте полный список вариантов перечисления.
Не используйте подстановочный знак `_` в match выражениях. Так как при добавлении нового вариант в перечисление оно не будет корректно учтено в логиге кода.

```
fn grant_permissions(role: &Role) -> Permissions {
    match role {
        Role::Reporter => Permissions::Read,
        Role::Developer => Permissions::Read & Permissions::Edit,
        _ => Permissions::All, // anybody else is administrator 
    }
}

Следует заменить на полный список вариантов перечисления и тогда при добавлении нового варианта код не скомпилируется, что даст вам повод исправить код.

fn grant_permissions(role: &Role) -> Permissions {
    match role {
        Role::Reporter => Permissions::Read,
        Role::Developer => Permissions::Read & Permissions::Edit,
        Role::Admin => Permissions::All, 
    }
}
```

Для структур так же следует использовать полный список полей с помощью деструктуризации.
В случае добавления нового поля, код не скомпилируется, что даст вам повод для его исправить.

```
struct Address {
    country: Country,
    city: City,
    street: Street,
    zip: Zip,
}

// Плохо
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.country)?;
        writeln!(f, "{}", self.city)?;
        writeln!(f, "{}", self.street)?;
        write!(f, "{}", self.zip)
    }
}

// Хорошо
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            country,
            city,
            street,
            zip,
        } = self;
        writeln!(f, "{country}")?;
        writeln!(f, "{city}")?;
        writeln!(f, "{street}")?;
        write!(f, "{zip}")
    }
}
```

[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_5_exhaustivity)