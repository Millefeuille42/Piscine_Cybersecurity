use std::fmt;

#[derive(Debug)]
pub enum StockholmError {
	ExtractIv(String),
	ExtractKey(String),
	ExtractPath(String),
	WriteFile(String),
	RenameFile(String),
	ReadFile(String),
	ReadDir(String),
	Utf8Decode(String),
	Base64Decode(String),
}

impl fmt::Display for StockholmError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			StockholmError::ExtractIv(details) => write!(f, "unable to extract iv from hash: {}", details),
			StockholmError::ExtractKey(details) => write!(f, "unable to extract key from hash: {}", details),
			StockholmError::ExtractPath(details) => write!(f, "unable to extract path: {}", details),
			StockholmError::WriteFile(details) => write!(f, "unable to write file: {}", details),
			StockholmError::RenameFile(details) => write!(f, "unable to rename file: {}", details),
			StockholmError::ReadFile(details) => write!(f, "unable to read file: {}", details),
			StockholmError::ReadDir(details) => write!(f, "unable to read directory: {}", details),
			StockholmError::Utf8Decode(details) => write!(f, "unable to decode (utf8), this is often due to a wrong key: {}", details),
			StockholmError::Base64Decode(details) => write!(f, "unable to decode (base64), file or key might be corrupted or invalid: {}", details),
		}
	}
}
