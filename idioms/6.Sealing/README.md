
## Sealing

Запечатывание в программировании обычно означает, что некоторые API (в основном общедоступные) не могут быть унаследованы, расширены или реализованы за пределами места их определения.
Основная цель запечатывания признака — это перспективность API. Так как мы уверены в том что только мы используем все имплементации запечатанного трейта то мы можем его менять.

Sealed трейт — общедоступный трейт, который не может быть реализован вне места его определения ( модуля или крейта , в зависимости от видимости этого трейта).

```
/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait TheTrait: private::Sealed {
    // Zero or more methods that the user is allowed to call.
    fn ...();

    // Zero or more private methods, not allowed for user to call.
    #[doc(hidden)]
    fn ...();
}

// Implement for some types.
impl TheTrait for usize {
    /* ... */
}

mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl Sealed for usize {}
}
```
Пустой частный Sealed супертрейт не может быть унаследован последующими крейтами, поэтому мы гарантируем, что реализации Sealed (и, следовательно TheTrait ) существуют только в текущем крейте.


Крейт [sealed](https://docs.rs/sealed) предоставляет удобный макрос для запечатывания.

```
use sealed::sealed;

#[sealed]
pub trait TheTrait {}

#[sealed]
impl TheTrait for usize {}
```

### Однако существуют альтернативные способы запечатать признак через [сигнатуру его метода](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/#sealing-traits-via-method-signatures) или даже частично запечатать его

```
mod private {
    pub struct Token;
}

pub trait SealedTrait {
    fn method(&self, _: private::Token);
}
pub struct TypeThatImplsSealed;

impl SealedTrait for TypeThatImplsSealed {
    fn method(&self, _: private::Token) {
        // impl here
    }
}
```
И при этом код вне модуля не сможет, ни реализовать признак, ни вызвать метод
```
struct DownstreamType {}

impl upstream::SealedTrait for DownstreamType {
    // ERROR: module `private` is private
    fn method(&self, token: upstream::private::Token) {}
}


fn call_method(value: impl upstream::SealedTrait) {
    // ERROR: module `private` is private
    let token = upstream::private::Token;
    value.method(token);
}
```

### Разрешить вызов только некоторых методов

Это позволяет нам явно решить, какие методы должны быть невызываемыми из последующих крейтов. 
```
mod upstream{
    mod private {
        pub struct Token;
    }

    pub trait SealedTrait {
        fn callable_method(&self);

        fn non_callable_method(&self, _: private::Token);
    }
}

Ничто не мешает вызывать эти методы из последующих крейтов.

fn call_method(value: impl upstream::SealedTrait) {
    value.callable_method();
}
```

### Частично запечатанные черты

Чтобы сделать SealedTrait частично изолированной — нам просто нужно предоставить реализацию по умолчанию для всех методов, которые принимают аргумент, который последующие ящики не могут реализовать.

```
mod private {
    pub struct Token;
}

pub trait PartiallySealedTrait {
    fn callable_method(&self);

    fn non_callable_method(&self, _: private::Token) {
        println!("you can't change this");
    }
}

Здесь вы, возможно, заметили, что непереопределяемый метод нашей особенности также не может быть вызван последующими крейтами. Однако мы можем разрешить нижестоящим ящикам вызывать его косвенно , если наш ящик предоставит такую ​​функцию

pub fn call_method_indirectly(value: &PartiallySealedTrait) {
    value.non_callable_method(private::Token)
}
```

[rust-incubator](https://github.com/instrumentisto/rust-incubator/tree/main/2_idioms/2_6_sealing)

[Rust API Guidelines: 10. Future proofing: Sealed traits protect against downstream implementations (C-SEALED)](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)

[Predrag Gruevski: A definitive guide to sealed traits in Rust](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust)

[Jack Wrenn: Private Methods on a Public Trait](https://jack.wrenn.fyi/blog/private-trait-methods)