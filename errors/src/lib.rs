mod errors;

pub fn returns_ok() -> Result<(), errors::Error> {
    Ok(())
}

pub fn returns_error(optional_error: Option<errors::Error>) -> Result<u8, errors::Error> {
    // Upgrade optional to error, None maps to Error, Some to Ok
    let error = optional_error.ok_or(errors::Error::Other)?;

    // The logic above does the same as:
    // let error = match optional_error {
    //     Some(e) => e,
    //     None => return Err(errors::Error::Other),
    // };

    Err(error)
}

// This could be any function that returns any type for the Error as long as the error type is
// public as well
pub fn returns_io_error() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Test standard error",
    ))
}

pub fn upgrade_error() -> Result<(), errors::Error> {
    returns_io_error()?;

    // equivalent to:
    // let _val = match returns_io_error() {
    //     Ok(e) => e,
    //     Err(e) => return Err(e.into()),
    // };

    // other function always returns error, ignore next line
    Ok(())
}

// This can/ should be put into the errors.rs file, but I am keeping here for readability
impl From<std::io::Error> for errors::Error {
    fn from(_e: std::io::Error) -> errors::Error {
        errors::Error::Other
    }
}

pub fn you_can_use_anything_lol<T>(
    v: T,
) -> Result<
    (),
    (
        bool,
        T,
        &'static str,
        u8,
        impl std::error::Error,
        Box<()>,
        [u8; 10],
    ),
> {
    Err((
        true,
        v,
        "I'm a string",
        8,
        errors::Error::BaseError,
        Box::new(()),
        [8u8; 10],
    ))
}

impl std::error::Error for errors::Error {
    fn description(&self) -> &str {
        // could match to get enum types
        "description of an implementation of an Error"
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        // No need to implement cause
        None
    }
}

// ? side note, you can convert any types using the .into() function, it's pretty neat
pub fn converting_type(i: u8) -> errors::Error {
    i.into()
}

impl From<u8> for errors::Error {
    fn from(i: u8) -> errors::Error {
        match i {
            1 => errors::Error::BaseError,
            _ => errors::Error::Other,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use errors::Error;

    #[test]
    fn test_errors() {
        let res = returns_error(Some(Error::BaseError));
        let err = res.unwrap_err();
        // * Can do this because of the PartialEq derive on the error
        assert_eq!(err, Error::BaseError);
        println!("{}", err);

        let res = returns_error(Some(Error::ParameterError("parameter!".to_owned())));
        // * Can check for error using if let
        if let Err(Error::ParameterError(_)) = res {
            assert!(true)
        } else {
            assert!(false)
        }
        println!("{}", res.unwrap_err());

        let res = returns_error(Some(Error::TwoParameterError("first".to_owned(), 2)));
        // * Or can match the result for any given error
        match res {
            Err(Error::TwoParameterError(_, _)) => assert!(true),
            _ => assert!(false),
        }
        println!("{}", res.unwrap_err());

        let res = returns_error(Some(Error::StructError {
            name: "austin".to_owned(),
            number: 8,
        }));
        // * Can check equality against an equal value error
        let err = res.unwrap_err();
        assert_eq!(
            err,
            Error::StructError {
                name: "austin".to_owned(),
                number: 8,
            }
        );
        println!("{}", err);

        let res = returns_error(Some(Error::NestedError(errors::OtherError::SimpleError)));
        let err = res.unwrap_err();
        assert_eq!(err, Error::NestedError(errors::OtherError::SimpleError));
        println!("{}", err);
    }

    #[test]
    #[should_panic]
    fn unwrapping_error_panic() {
        // Unwrapping when error in return will cause a panic
        returns_error(None).unwrap();
    }

    #[test]
    fn better_handling_errors() {
        let res = returns_error(None);
        // Either match
        let _unwrapped = match res {
            Ok(v) => v,
            Err(_e) => {
                // Can do any logic in the code block
                println!("error handled here");
                // Returns 1 for u8 expected value into _unwrapped
                1
            }
        };
        // This could be simplified if the function returns an error, by doing:
        // returns_error(...)?;
        // Which will upgrade the error in returns_error to the function the call is made in
        // Can convert the error into an option to check as such
        let res = returns_error(None);
        assert_eq!(res.err(), Some(Error::Other));

        let res = returns_ok();
        assert_eq!(res.is_err(), false);
        assert_eq!(res.err(), None);

        // Can unwrap with a value if error
        let value = returns_error(None).unwrap_or(1);
        // value will take default because function always returns error
        assert_eq!(value, 1);
        // Or can leave the unwrap to default the type's initial value
        let value = returns_error(None).unwrap_or_default();
        assert_eq!(value, 0);
    }

    #[test]
    fn upgrading_error() {
        let res = upgrade_error();
        assert!(res.unwrap_err() == Error::Other);
    }
    #[test]
    fn upgrade_example() {
        let err = converting_type(1);
        assert!(err == Error::BaseError);
    }
}
