mod error;
mod services;

pub use error::Error;
pub use services::*;

/// Basic trait which uses mutable reference to start it
pub trait StartableService {
    fn start(&mut self) -> Result<(), String> {
        Ok(())
    }
}
/// Trait with associated type which is used here as the Error type
pub trait AssocService {
    type AssocError;

    fn start(&mut self) -> Result<(), Self::AssocError> {
        Ok(())
    }
}

/// Generics can be used inline or in where, but where is more extensible and usable for
/// associated types or when the trait itself does not implement the trait
/// ex: (int, T): MyTrait
pub fn start_service<S: AssocService>(s: &mut S) -> Result<(), String>
where
    S::AssocError: ToString + Clone,
{
    Ok(s.start().map_err(|e| {
        // Clone is not actually needed, just to show usage of requiring multiple traits
        let _ = e.clone();
        e.to_string()
    })?)
}

/// To be able to have structures used in a type or function where the type must be defined over
/// ALL elements, you must use existentially quantified types in the form of dynamic dispatch
/// pointers which come at a runtime cost instead of compiling into multiple implementations of
/// functions for each type the generic is used for
pub fn start_all(services: &mut [&mut dyn StartableService]) -> Vec<Result<(), String>> {
    services.iter_mut().map(|s| s.start()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_assoc_services() {
        let mut s1 = ServiceOne;
        let mut s2 = ServiceTwo;
        let mut s3_f = ServiceThree { fails: true };
        let mut s3_s = ServiceThree { fails: false };
        assert_eq!(start_service(&mut s1), Ok(()));
        assert_eq!(
            start_service(&mut s2),
            Err("Custom Service error: Service two failed!".to_owned())
        );
        assert_eq!(
            start_service(&mut s3_f),
            Err("Failed to start service".to_owned())
        );
        assert_eq!(start_service(&mut s3_s), Ok(()));
    }

    #[test]
    fn start_all_services() {
        let mut s1 = ServiceOne;
        let mut s2 = ServiceTwo;
        let mut s3_f = ServiceThree { fails: true };
        let mut s3_s = ServiceThree { fails: false };

        // Type does not need to be vector, just simulating a runtime use case
        let mut services: Vec<&mut dyn StartableService> = vec![];
        services.push(&mut s1);
        services.push(&mut s2);
        services.push(&mut s3_f);
        services.push(&mut s3_s);

        let results = start_all(&mut services);
        let (succeeded, errors): (Vec<_>, Vec<_>) = results.iter().partition(|r| r.is_ok());
        assert_eq!(succeeded.len(), 2);
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn call_all_variants() {
        let mut s = ServiceThree { fails: true };
        // If a base implementation exists, that will be used as default
        assert_eq!(s.start(), Err(()));

        // Any specific implementation can be called by specifying trait implementation
        // or using in a function
        assert_eq!(
            StartableService::start(&mut s),
            Err("Failed to start service".to_owned())
        );
        assert_eq!(AssocService::start(&mut s), Err(Error::FailedToStart));
        assert_eq!(ServiceThree::start(&mut s), Err(()));
    }
}
