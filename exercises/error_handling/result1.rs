// result1.rs
// Make this test pass! Execute `rustlings hint result1` for hints :)

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // 1
        // match value {
        //     _ if value < 0 => return Err(CreationError::Negative),
        //     _ if value == 0 => return Err(CreationError::Zero),
        //     _ => return Ok(PositiveNonzeroInteger(value as u64)),
        // };

        // 2
        let v;
        match value {
            _ if value < 0 => v = Err(CreationError::Negative),
            _ if value == 0 => v = Err(CreationError::Zero),
            _ => v = Ok(PositiveNonzeroInteger(value as u64)),
        };
        v
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
