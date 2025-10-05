use time::PrimitiveDateTime as DateTime;
use time::Duration as duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(duration::seconds(1_000_000_000))
}
