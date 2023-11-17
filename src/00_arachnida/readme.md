# 00 - Arachnida

## 00 - Spider

### Goal

The spider program allow you to extract all the images from a website, recursively, by
providing an url as a parameter.
You have to manage the following program options:

`./spider [-rlp] URL`

- Option -r : recursively downloads the images in a URL received as a parameter.
- Option -r -l [N] : indicates the maximum depth level of the recursive download.
If not indicated, it will be 5.
- Option -p [PATH] : indicates the path where the downloaded files will be saved.
If not specified, ./data/ will be used.

The program will download the following extensions by default:
- .jpg/jpeg
- .png
- .gif
- .bmp

## 01 - Scorpion

The second scorpion program receive image files as parameters and must be able to
parse them for EXIF and other metadata, displaying them on the screen.

The program must at least be compatible with the same extensions that spider handles.

It displays basic attributes such as the creation date, as well as EXIF data. 
The output format is up to you.

`./scorpion FILE1 [FILE2 ...]`