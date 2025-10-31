/*
 `Pattern Specification` предлагает решение, позволяющее создавать многократно используемые бизнес-правила, которые можно комбинировать с использованием логики.

*/

pub trait EmailValidator {
    fn is_valid(&self, email: &EmailAddress) -> bool;
}

use client::EmailAddress;
mod client {
    pub struct EmailAddress {
        user_name: String,
        domain: String,
    }

    impl EmailAddress {
        pub fn new(user_name: String, domain: String) -> Self {
            Self { user_name, domain }
        }
        pub fn user_name(&self) -> &str {
            self.user_name.as_str()
        }
        pub fn domain(&self) -> &str {
            self.domain.as_str()
        }
    }
}

use validators::{DomainValidator, UsernameValidator};
mod validators {
    use super::client::EmailAddress;
    use super::EmailValidator;

    pub struct UsernameValidator {
        pub min_length: usize,
    }

    impl EmailValidator for UsernameValidator {
        fn is_valid(&self, email: &EmailAddress) -> bool {
            email.user_name().len() >= self.min_length
        }
    }

    pub struct DomainValidator {
        pub min_length: usize,
        pub allowed_domains: Vec<String>,
    }

    impl EmailValidator for DomainValidator {
        fn is_valid(&self, email: &EmailAddress) -> bool {
            email.domain().len() >= self.min_length
                && self.allowed_domains.contains(&email.domain().to_owned())
        }
    }
}

use logic::EmailValidatorImpl;
mod logic {
    use super::client::EmailAddress;
    use super::EmailValidator;

    pub struct EmailValidatorImpl {
        validators: Vec<Box<dyn EmailValidator>>,
    }

    impl EmailValidatorImpl {
        pub fn new(validators: Vec<Box<dyn EmailValidator>>) -> Self {
            Self { validators }
        }

        pub fn is_valid(&self, email: &EmailAddress) -> bool {
            self.validators
                .iter()
                .all(|validator| validator.is_valid(email))
        }
    }
}

fn main() {
    let my_email = EmailAddress::new("test".to_string(), "gmail.com".to_string());
    let local_part_validator = UsernameValidator { min_length: 3 };
    let domain_validator = DomainValidator {
        min_length: 3,
        allowed_domains: vec!["gmail.com".to_string()],
    };
    let email_validator = EmailValidatorImpl::new(vec![
        Box::new(local_part_validator),
        Box::new(domain_validator),
    ]);
    println!("{}", email_validator.is_valid(&my_email));
}
