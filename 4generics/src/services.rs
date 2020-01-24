use crate::{AssocService, Error, StartableService};

/// This service will always start
pub struct ServiceOne;
impl StartableService for ServiceOne {}
impl AssocService for ServiceOne {
    type AssocError = String;
}

/// This service will always fail to start
pub struct ServiceTwo;
impl StartableService for ServiceTwo {
    fn start(&mut self) -> Result<(), String> {
        Err("Service two failed!".to_owned())
    }
}
impl AssocService for ServiceTwo {
    type AssocError = Error;
    fn start(&mut self) -> Result<(), Self::AssocError> {
        Err(Error::Custom("Service two failed!".to_owned()))
    }
}

/// This service may or may not fail
pub struct ServiceThree {
    pub fails: bool,
}
impl StartableService for ServiceThree {
    fn start(&mut self) -> Result<(), String> {
        if self.fails {
            Err(Error::FailedToStart.to_string())
        } else {
            Ok(())
        }
    }
}
impl AssocService for ServiceThree {
    type AssocError = Error;
    fn start(&mut self) -> Result<(), Self::AssocError> {
        if self.fails {
            Err(Error::FailedToStart)
        } else {
            Ok(())
        }
    }
}

impl ServiceThree {
    /// Additional implementation of same function signature
    pub fn start(&mut self) -> Result<(), ()> {
        if self.fails {
            Err(())
        } else {
            Ok(())
        }
    }
}
