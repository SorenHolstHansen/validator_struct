macro_rules! assert_is_type {
    ($t:ty, $i:ident: $ti:ty) => {
        const _: () = {
            fn dummy(v: $t) {
                let _: $ti = v.$i;
            }
        };
    };
}

mod tests {
    use validator::Validate;
    use validator_struct::{ValidatorMessageStruct, ValidatorStruct};

    #[derive(Validate, ValidatorStruct)]
    struct Foo {
        #[validate(length(equal = 5, message = "Please provide a valid foo!"))]
        foo: String,
        bar: String,
    }

    #[test]
    fn test_message() {
        let bad_foo = Foo {
            foo: "hi!".into(),
            bar: "there".into(),
        };
        // assert_is_type!(FooError, bar: i32);
        let err = bad_foo.validate_struct();
        println!("KJHDFKJ {:#?}", err.unwrap_err());
        let err = bad_foo.validate().unwrap_err().to_string();
        assert_eq!(err, "foo: Please provide a valid foo!");
    }
}
