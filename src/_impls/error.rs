use std::error::Error;


impl Error for crate::Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|s| { s as &(dyn Error + 'static) })
    }
    
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
    
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
