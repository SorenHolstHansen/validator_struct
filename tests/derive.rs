mod tests {
    use serde::{Deserialize, Serialize};
    use validator::{Validate, ValidationError};
    use validator_struct::ValidatorStruct;

    #[derive(Validate, ValidatorStruct)]
    #[validator_struct(derive(Serialize))]
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

    fn validate_username(username: &str) -> Result<(), ValidationError> {
        if username.len() < 2 {
            return Err(ValidationError::new("Username is too short"));
        };

        Ok(())
    }

    #[derive(Deserialize, Validate, ValidatorStruct)]
    #[validator_struct(derive(Serialize))]
    struct BasicSignupForm {
        #[validate(custom = "validate_username")]
        username: String,
    }

    #[test]
    fn test_custom_validator() {
        let bad_form = BasicSignupForm {
            username: "a".to_string(),
        };

        let err = bad_form.validate_struct().unwrap_err().username;
        assert_eq!(err, Some(vec!["Username is too short".to_string()]));
    }
}
