pub mod symbol;
pub mod errors;

#[allow(dead_code)]
pub mod mark;

#[allow(dead_code)]
pub mod temp;

/// Takes a Result type and returns the value if Ok and panics if Err
#[macro_export]
macro_rules! trypanic {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => panic!(err)
    })
}
