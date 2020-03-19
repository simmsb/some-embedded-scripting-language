#![feature(or_patterns)]

pub mod expr;
pub mod cont_expr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
