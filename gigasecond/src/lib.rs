use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let end = start.checked_add(Duration::new(1_000_000_000,0));
    end.unwrap()
}
