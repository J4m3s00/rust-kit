use std::error;

mod intern {
    pub mod bindings;
}

pub fn start_application() -> Result<(), Box<dyn error::Error>> {
    unsafe {
        let result = intern::bindings::start_application();
        if result == 0 {
            Ok(())
        } else {
            Err("Error starting application".into())
        }
    }
}
