#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum StringifyError {
	#[error("Some operation overflowed or some number conversion failed")]
	NumberOutOfRange,
	#[error("No units were enabled")]
	NoUnitsEnabled,
}
