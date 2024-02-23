mod tests {
    use validator::Validate;
    use validator_struct::{ValidatorMessageStruct, ValidatorStruct};

    #[derive(Validate, ValidatorMessageStruct)]
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

        let err = bad_foo.validate_struct().unwrap_err().foo;
        assert_eq!(err, Some(vec!["Please provide a valid foo!".to_string()]));
        let err = bad_foo.validate().unwrap_err().to_string();
        assert_eq!(err, "foo: Please provide a valid foo!");
    }
}
