# Arachnida

## Spider
Crawls a website and download its images.

```
Usage: spider [OPTIONS] <URL>

Arguments:
  <URL>  The URL to crawl images from

Options:
  -r              Recursively download images
  -l <LEVEL>      Maximum depth level of recursion [default: 5]
  -p <PATH>       Path where downloaded files will be saved [default: ./data]
  -h, --help      Print help
```

## Scorpion
Prints EXIF and other metadata from image files.

```
Usage: spider <FILES>

Arguments:
  <FILE>  The files to read metadata from
```