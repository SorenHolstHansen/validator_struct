# validator_struct

A simple ergonomic addition to the validator crate.

# Usage

We provide two derive macros to make working with the validator crate easier: `ValidatorStruct` and `ValidatorMessageStruct`

They can be used alongside the `Validate` derive macro like this
```rust
#[derive(Validate, ValidatorStruct, ValidatorMessageStruct)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(phone)]
    phone: String,
    #[validate(url)]
    site: String,
    #[validate(length(min = 1), custom = "validate_unique_username")]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
}

fn validate_signup_data(data: SignupData) {
  // validate_struct() returns a SignupDataError struct
  // Where each field is replaced by a `Vec<ValidationError>`
  data.validate_struct();
  // validate_struct() returns a SignupDataErrorMessage struct
  // Where each field is replaced by a `Vec<String>`
  data.validate_message_struct(); 
}
```
