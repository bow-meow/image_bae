# Image Generation Tool

A tool to generate images in sequence with customizable parameters such as fonts, dimensions, file size, colors, text colors, and formats.

---

## Sequencing

You can sequence any command with a comma. Here are some examples:

- `--color red,blue,green`
- `--filesize 1000,100,500`
- `--format rand,tif,rand`

In these examples, every 3 images will follow the specified sequence.

---

## Parameters

### Text
The text displayed on the image.

**Examples:**
- `--text [1..100]`
- `--text [200..1]`
- `--text [a..z]`
- `--text [z..a]`
- `--text "foo"`
- `--text "foo [1..3] [a..c]"`

---

### Size
The dimensions of the image.

**Examples:**
- `--size 500`
- `--size 500x500`
- `--size rand(100..500)`
- `--size 500xrand(100..500)`
- `--size rand(100..500)x500`
- `--size rand(100..500)xrand(100..500)`

---

### Font
The font style used for the text.

**Examples:**
- `--font comicsans`
- `--font "comicsans,arial,cour"`
- `--font rand`

---

### Color
The background color of the image.

**Examples:**
- `--color red`
- `--color "red,blue,green"`
- `--color #adhas2`
- `--color (255,21,200)`
- `--color rand(red,#adhas2,(255,21,200))`
- `--color rand`

---

### Text Color
The color of the text on the image.

**Examples:**
- `--textc red`
- `--textc "red,blue,green"`
- `--textc #adhas2`
- `--textc (255,21,200)`
- `--textc rand(red,#adhas2,(255,21,200))`
- `--textc rand`

---

### Format
The format of the image. Currently limited to `jpg`, `png`, and `tif`.

**Examples:**
- `--format jpg`
- `--format jpg,png,tif`
- `--format rand(jpg,png,tif)`
- `--format rand`

---

### Filesize
The file size or weight of the file. `1000` is equal to `1MB`.

**Examples:**
- `--filesize 1000`
- `--filesize rand`
- `--filesize rand(100..1000)`

---

### Repeater
The number of times the operation repeats.

**Examples:**
- `-r 10`
- `-r rand`
- `-r rand(1..10)`

---

## Example Command

```bash
image_bae.exe --text "[1..100]" --color rand --font rand --textc rand --format rand

## Roadmap

- Allow size parameters like `500xrand`, `randxrand`, or `randx500`.
- Add the ability to change the font size.
- Add support for generating layered TIFFs.
- Improve error messages (e.g., using `thiserror` and addressing `ParseIntError`).
- Add ability to set color profiles.
- Create GUI using `egui`.