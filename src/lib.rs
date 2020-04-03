#![feature(or_patterns)]

pub mod expr;
pub mod cont_expr;
pub mod flat_expr;
pub mod literals;
mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
