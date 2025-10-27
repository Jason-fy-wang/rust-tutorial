// import utils
pub mod utils;

// import service
pub mod service;


// doc test 
/// # Example:
/// ```
/// assert_eq!(common::add(1,3),4);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
}
