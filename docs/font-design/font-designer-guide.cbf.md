# Compact Bitmap Font (CBF): A Designer's Guide

The **Compact Bitmap Font (CBF)** format is a compact, memory-efficient format for bitmap (pixel) fonts. It is designed for embedded systems, retro-style games, terminals, and anywhere performance and simplicity matter.

This guide helps designers prepare the visual and metadata assets needed to generate a `.cbf` font file.

---

## 🧹 Required Assets

To compile a CBF font, you need:

1. **A PNG image**: Your pixel font glyphs arranged in a single row.
2. **A JSON config**: Metadata and layout settings for your font.

---

## 🎨 PNG Source Image Specifications

### ✅ Format and Layout

| Requirement        | Description                                                                                                                                             |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| File format        | PNG, **without transparency**                                                                                                                           |
| Color mode         | RGB                                                                                                                                                     |
| Background color   | Solid white (`#FFFFFF` / `RGB(255, 255, 255)`)                                                                                                          |
| Glyph layout       | All glyphs must be arranged **horizontally in a single row**, in the order defined in the JSON's `char_order`                                           |
| Glyph color        | Solid black (`#000000` / `RGB(0, 0, 0)`)                                                                                                                |
| Top padding        | Exactly **1px padding** above the [ascenders](https://en.wikipedia.org/wiki/Ascender_%28typography%29) (this vertical space houses glyph separators)    |
| Bottom padding     | None — [descenders](https://en.wikipedia.org/wiki/Descender) of glyphs must touch the bottom edge of the image                                          |
| Left/Right padding | Exactly **1px margin** on each side (background color)                                                                                                  |
| Separation markers | **1-pixel colored dots** on the line above [ascenders](https://en.wikipedia.org/wiki/Ascender_%28typography%29), horizontally -- between the glyphs     |
| Start/end  markers | The pixel in the top-left corner and the pixel in top-right corner of the image must contain the glyph separator.                                       |
| Horizontal spacing | **1px wide white space** (background color) to separate glyphs from each other                                                                          |
| The 'space' glyph  | The charset **must** include space character, so there should be a corresponding glyph in the PNG.                                                      |




#### ⚠️ Important Notes

* Characters are separated by a **colored pixel** (separator) **above the glyph**, not by vertical lines.
* 1px horizontal white space (background color) exists between glyphs.
* The font must be monochrome: black-on-white only.
* The image should contain **no extra padding or alignment guides** outside of what’s described.

---









## Overview

In order to compile a font you need 2 assets: 
1. A PNG source image with your font design.
2. A JSON font configuration file.

The generator for compiling a font is available as:
- An online tool: https://coduments.com/to-be-announced
- A CLI tool:
    - Download a binary: https://coduments.com/to-be-announced
    - Build from source code: https://github.com/dipdowel/compact-bitmap-font/tree/develop/rust/cbf_wiz


## Assets preparation
 
### Source font image (PNG)

1. The source image of a pixel font should contain glyphs design of your font.
2. The image should be a PNG file, without transparency.
3. There should be 1 pixel spacing between the glyphs horizontally, i.e. there should be a vertical column `1px` wide, separating characters from each other.
4. On the top and bottom there should be no vertical spacing between the most protruding extremities of the characters and the edge of the image
5. On the left and right, there must be 1 column left blank (i.e. 1px margin on left and right) with the char separator marker. See `Glyph separation markers (separators).` below.



![sample source PNG](./media/source-image-sample.png)
NB: this image was scaled up for illustration purposes! 




Please see [cbf_wiz](./media/source-image-sample.png)

### Background color
The image background must be a 100% white color in RGB model, for example:
- Hex value `#ffffff`
- RGB value: `255, 255, 255`

### Font glyph color
The color used for depicting the font glyphs must be a 100% black in RGB model, for example:
- Hex value `#000000`
- RGB value: `0, 0, 0`

### Glyph separation markers (separators).
- The image should contain separation markers of 1 pixel. These markers signal the start and end of each glyph/character. 
- Color of the separators must be any RGB color other than 100% black and 100% white. For example, cyan could be a good option.










### Font configuration and metadata (JSON)

The main asset for compiling 



- [cbf_wiz](../rust/cbf_wiz/README.md) -- a Rust-based CBF font generator, verifier and sample text renderer.
- [cbf_viewer](https://cbf.codument.com/) -- an online CBF viewer and font generator.
## Status
> ⚠️ **NB:** The specification itself is stable, but this document is still being improved. Stay tuned.



CBF files store pixel fonts using a compact header followed by metadata, character information, and 1-bit image data.

While the format prioritizes minimal memory overhead, UTF-8 encoding is used for its string fields (e.g., font name, author signature, character order) because:

* **Cross-language compatibility**: UTF-8 is standard across modern platforms.
* **Efficiency for ASCII**: UTF-8 is 1 byte per ASCII character, which matches plain ASCII.
* **Support for internationalization**: UTF-8 means Unicode support.

---

## File Structure

A CBF file consists of:

1. **Header** (14 `u16` values)
2. **Font Name** (UTF-8 string)
3. **Author Signature** (UTF-8 string)
4. **Character Order** (UTF-8 string)
5. **Character Widths** (byte array)
6. **Font Bitmap Data** (1-bit per pixel)

### 1. Header Layout (14 x 2 bytes = 28 bytes total)

Header consists of 14 `u16` values (little-endian):

| Index | Field Name              | Description                                            |
| ----- | ----------------------- | ------------------------------------------------------ |
| 0     | `cbf_magic_number`      | CBF Magic Number `0xCBF0` (Compact Bitmap F0nt)        |
| 1     | `cbf_version`           | Format version, currently `1`                          |
| 2     | `font_name_size`        | Number of bytes in the font name string (UTF-8)        |
| 3     | `author_signature_size` | Number of bytes in the author signature string (UTF-8) |
| 4     | `char_order_size`       | Number of bytes in the character order string (UTF-8)  |
| 5     | `char_widths_size`      | Number of entries in character widths array (`u8[]`)   |
| 6     | `font_image_width`      | Width of the bitmap in pixels                          |
| 7     | `font_image_height`     | Height of the bitmap in pixels                         |
| 8     | `spacing_props`         | Lower byte: kerning (u8), upper byte: leading (u8)     |
| 9     | `default_char_part_1`   | UTF-8 bytes 0–1 of default character as `u16`          |
| 10    | `default_char_part_2`   | UTF-8 bytes 2–3 of default character as `u16`          |
| 11    | `font_ver`              | User-defined font version                              |
| 12    | `date_year`             | Date: font creation year (4-digit)                     |
| 13    | `month_day`             | Date: lower byte: day (u8), upper byte: month (u8)     |


### 2. Font Name

UTF-8 string, length defined by header entry \[2].

### 3. Author Signature

UTF-8 string, length defined by header entry \[3].

### 4. Character Order String

A UTF-8 string that defines the order of the glyphs on the font bitmap. Size of the string is defined in the header \[4].

Consider the following character order string + the corresponding font image:
```text
 ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ 
```
 
![cc_red_alert_inet.png](cc_red_alert_inet.png)

 
> ⚠️ **NB:** The actual bitmap image in a CBF file does not have 1px margins between the glyphs to minimize the file size. The margins were added to the image above for better visibility.


#### Purpose and Benefits

* **Fast glyph lookup**: When rendering a UTF-8 encoded string, each character can be located via a simple lookup in this list.
* **Layout flexibility**: Glyphs in the image can appear in any order; this field tells the renderer how to associate image segments with characters.
* **Sparse font support**: The font can include only a specific subset of characters (e.g., digits, capital letters only, etc.) rather than needing to cover full Unicode ranges.

### 5. Character Widths

Array of `u8`, each representing the width (in pixels) of each glyph. Length is specified by `char_widths_size` in the header \[5]. This array must have **the same length as the number of characters in the Character Order String**, and the order of the widths must exactly match the order of characters in the string and on the image. That is, the first width corresponds to the first character, the second width to the second character, and so on. A mismatch in count or order should be treated as an invalid file.

#### Visual Mapping Diagram

```text
char_order:     ['A', 'B', 'C', 'D']
char_widths:    [ 5 ,  3 ,  4 ,  6 ]
                  ↑    ↑    ↑    ↑
                  │    │    │    └── width of 'D'
                  │    │    └─────── width of 'C'
                  │    └──────────── width of 'B'
                  └───────────────── width of 'A'
```

### 6. Font Bitmap Data
The bitmap is represented as a 1-bit-per-pixel bit-array. The image arranges all glyphs horizontally, according to the Character Order String. The width of each glyph is defined in the Character Widths array.

Each bit represents a single pixel:
- **1** → white pixel (on)
- **0** → black pixel (off)


#### Glyph Positioning
To locate the glyph for a given character:
- Look up its index in `char_order`.
- Sum the widths of all the preceding glyph to compute the x-offset.

```text
x_offset = Σ width_i for all characters before the target character
```

---

## The Default Character

The default character is stored as two `u16` words (in the header in \[9] and \[10]) representing a 4-byte UTF-8 character.

### Purpose of the Default Character

The default character acts as a fallback when a requested character is not found in the font's Character Order String.
* **Ensures robustness**: Prevents rendering failures when encountering unknown characters, no extra error handling needed.
* **Customizable appearance**: Designers may use a question mark, box, or other symbol.

---

## Validation Checklist

Use this checklist to verify that a `.cbf` file is valid:

1. Header\[0] == 0xCBF0 (magic number)
2. Header\[1] == 1 (a format version that you expect and know how to parse)
3. Sum of widths of all the glyphs equals the width of the font bitmap in the header \[6]:
    - I.e. `Σ char_widths[i] === font_image_width [6]`

---

## Notes

* As of version 1, bitmap in the font is not compressed.
* Implementers should reject files with an unknown version in the header.
* Always validate the magic number and the  and version before parsing.
* The font metadata section (name, author, version, creation date) can be used in font selection UIs and diagnostics/debugging.

---

## License

Specification derived from the open-source Graph1 project.

---
