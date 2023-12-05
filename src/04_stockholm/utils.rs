use std::array::TryFromSliceError;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{Engine as _, engine::general_purpose};
use sha2::{Sha256, Digest};
use crate::errors::StockholmError;

pub fn read_key(key_file: &PathBuf) -> Result<String, StockholmError> {
	let key = fs::read(key_file).map_err(|err| StockholmError::ReadFile(err.to_string()))?;
	String::from_utf8(key).map_err(|err| StockholmError::Utf8Decode(err.to_string()))
}

pub fn decode_key(key_encoded: String) -> Result<[u8;32], StockholmError> {
	let key = general_purpose::STANDARD.decode(key_encoded).map_err(|err| StockholmError::Base64Decode(err.to_string()))?;
	key[0..32].try_into().map_err(|err: TryFromSliceError| StockholmError::ExtractKey(err.to_string()))
}

pub fn generate_key() -> Result<[u8;32], StockholmError> {
	let mut hasher256 = Sha256::new();
	let key = whoami::hostname() +
		whoami::username().as_str() +
		SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos().to_string().as_str();
	hasher256.update(key.as_bytes());
	let key = format!("{:x}", hasher256.clone().finalize()).into_bytes();
	key[0..32].try_into().map_err(|err: TryFromSliceError| StockholmError::ExtractKey(err.to_string()))
}

pub fn generate_file_iv(key_encoded: &str, file_path: &str) -> Result<[u8;16], StockholmError> {
	let file_path = file_path.trim_end_matches(".ft");
	let mut hasher256 = Sha256::new();
	hasher256.update(format!("{key_encoded}{file_path}"));
	let iv = format!("{:x}", hasher256.clone().finalize()).into_bytes();
	iv[0..16].try_into().map_err(|err: TryFromSliceError| StockholmError::ExtractIv(err.to_string()))
}