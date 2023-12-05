use clap::{Arg, ArgAction, Command};
use url::Url;
use reqwest::blocking::Client;
use std::path::PathBuf;
use std::fs;


fn download_image(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = Client::new().get(url).send()?;
    let content = response.bytes()?;
    fs::write(filename, content)?;
    println!("  Saved image {filename}");
    Ok(())
}

fn crawl(url: &str, depth: u32, current_depth: u32, save_path: &PathBuf, visited: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if current_depth > depth {
        return Ok(());
    }

    println!("<{current_depth}> - Crawling {url}");

    let response = Client::new().get(url).send();
    if response.is_err() {
        eprintln!("    Invalid URL {url}");
        return Ok(());
    }
    let response = response.unwrap();
    let body = response.text()?;
    let base_url: Url = Url::parse(url)?;
    let base_domain = base_url.domain();
    if base_domain.is_none() {
        eprintln!("No domain in URL");
        return Ok(());
    }
    let base_domain = base_domain.unwrap();

    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("img").unwrap();

    for element in document.select(&selector) {
        if let Some(src) = element.value().attr("src") {
            let img_url = base_url.join(src)?;
            let mut file_path = img_url.clone();
            file_path.set_query(None);
            let file_path = file_path.path_segments().ok_or("invalid_path")?.last().ok_or("invalid_path")?;
            if !file_path.ends_with(".png")
                && !file_path.ends_with(".bmp")
                && !file_path.ends_with(".gif")
                && !file_path.ends_with(".jpeg")
                && !file_path.ends_with(".jpg") { continue }
            let file_path = save_path.join(file_path);
            if file_path.metadata().is_err() {
                download_image(
                    img_url.as_ref(),
                    file_path.to_str().unwrap()
                )?
            }
        }
    }

    if current_depth >= depth { return Ok(()); }
    let selector = scraper::Selector::parse("a[href]").unwrap();
    let mut links_to_visit: Vec<String> = Vec::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let mut new_url = "".to_string();
            if href.starts_with("http") {
                let url_href = Url::parse(href);
                if url_href.is_err() { continue; }
                if let Some(domain_href) = url_href.unwrap().domain() {
                    if base_domain == domain_href {
                        new_url = href.to_string();
                    }
                } else { continue; }
            } else {
                if href.starts_with('#') { continue; }
                let _new_url = base_url.join(href)?;
                new_url = _new_url.to_string();
            }
            if visited.contains(&new_url.to_string()) { continue; }
            links_to_visit.push(new_url.to_string());
            visited.push(new_url.to_string());
            println!("  <{current_depth}> - {url} - Found link: {}", href);
        }
    }

    for link in links_to_visit.iter() {
        if let Err(e) = crawl(link, depth, current_depth + 1, save_path, visited) {
            eprintln!("An error occurred while crawling {e}");
        }
    }

    Ok(())
}

fn main() {
    let matches = Command::new("spider")
        .arg(Arg::new("URL")
            .required(true)
            .index(1)
            .help("The URL to crawl images from"))
        .arg(Arg::new("recursive")
            .short('r')
            .action(ArgAction::SetTrue)
            .help("Recursively download images"))
        .arg(Arg::new("level")
            .short('l')
            .default_value("5")
            .value_name("LEVEL")
            .requires("recursive")
            .value_parser(clap::value_parser!(u32))
            .help("Maximum depth level of recursion"))
        .arg(Arg::new("path")
            .short('p')
            .default_value("./data")
            .value_name("PATH")
            .help("Path where downloaded files will be saved"))
        .get_matches();

    let url: &String = matches.get_one("URL").unwrap();
    let recursive: bool = matches.get_flag("recursive");
    let mut depth: u32 = *matches.get_one("level").unwrap();
    let save_path: &String = matches.get_one::<String>("path").unwrap();
    if !recursive { depth = 0; }

    let mut save_dir = PathBuf::default();
    save_dir.push(save_path);

    let mut visited: Vec<String> = Vec::new();
    visited.push(url.to_string());
    fs::create_dir_all(save_path).expect("Unable to create save dir");
    if let Err(e) = crawl(url.as_str(), depth, 0, &save_dir, &mut visited) {
        eprintln!("Error: {e}");
    }
}