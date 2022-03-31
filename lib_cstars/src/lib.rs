pub mod commands;
pub mod configuration;
pub mod errors;
pub mod http;
pub mod shared;

use log;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
