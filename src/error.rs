#[derive(Debug)]
pub enum Error<'a> {
    ParseError(&'a str),
}
