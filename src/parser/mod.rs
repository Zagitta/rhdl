//pub mod grammar;
pub mod utils;

include!("grammar.rs");


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_parses() {
        assert_eq!(parse_identifier("abc"), Ok("abc"));
        assert_eq!(parse_identifier("\\abc\\"), Ok("abc"));
        assert!(parse_identifier("1abc").is_err());
    }
}