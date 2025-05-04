# Compact Bitmap Font (CBF) Format Specification

The **Compact Bitmap Font (CBF)** format is a binary format for storing pixel-based bitmap fonts, designed for minimal memory overhead and simple parsing and rendering. This document provides a technical specification of the CBF format and `.cbf` files.

---

## Overview

CBF files store pixel fonts using a compact header followed by metadata, character information, and 1-bit image data. 

While the format prioritizes minimal memory overhead, UTF-8 encoding is used for its string fields (e.g., font name, author signature, character order) because:

* **Cross-language compatibility**: UTF-8 is standard across modern platforms.
* **Efficiency for ASCII**: UTF-8 is 1 byte per ASCII character, which matches plain ASCII.
* **Support for internationalization**: UTF-8 enables optional Unicode support without changing the format.

Also, there is no need for padding or null terminators as the size of each UTF-8 field in the file is defined in the header.

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
| 0     | `cbf_magic_number`      | CBF Magic Number, must be `0xCBF0`                     |
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

### 4. Character Order

UTF-8 string that defines the sequence of characters represented in the bitmap. Size defined in header \[4].

#### Purpose and Benefits

* **Fast glyph lookup**: When rendering a UTF-8 encoded string, each character can be located via a simple lookup in this list.
* **Layout flexibility**: Glyphs in the image can appear in any order; this field tells the renderer how to associate image segments with characters.
* **Sparse font support**: The font can include only a specific subset of characters (e.g., digits, capital letters only, etc.) rather than needing to cover full Unicode ranges.

### 5. Character Widths

Array of `u8`, each representing the width (in pixels) of the corresponding character in `char_order`. Length = header \[5]. This array **must have the same number of elements as the number of characters in `char_order`**, and the order of widths must exactly match the order of characters. That is, the first width corresponds to the first character, the second width to the second character, and so on. A mismatch in count or order should be treated as an invalid file.

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

1-bit-per-pixel packed bitmap. The image arranges all glyphs horizontally, in the order specified by `char_order`. The width of each glyph is defined in the `char_widths` array.

Each bit represents a single pixel:
- **1** → white pixel (on)
- **0** → black pixel (off)

Bits are packed in row-major order, meaning pixels are serialized from left to right across each row, and rows are stored from top to bottom. Rows are byte-aligned; padding may be required at the end of each row to align to the next byte boundary if image width is not a multiple of 8.

#### Example
A row with 10 pixels: `1010011101` would be stored as two bytes: `[0b10100111, 0b01000000]`.

#### Glyph Positioning
To locate the glyph for a given character:
- Look up its index in `char_order`.
- Sum the widths (plus kerning) of all preceding characters to compute its x-offset.

Formula (in pixels):
```text
x_offset = Σ (width_i + kerning) for all characters before the target character
````

representing the font's visual appearance. The image arranges all glyphs horizontally, in the order specified by `char_order`. The width of each glyph is defined in the `char_widths` array.

Each bit represents a single pixel:

* **1** → white pixel (on)
* **0** → black pixel (off)

Bits are packed in row-major order. Rows are byte-aligned; padding may be required at the end of each row to align to the next byte boundary if image width is not a multiple of 8. of the glyphs arranged horizontally, representing the font source image. Characters are stored in the order defined by `char_order`. White (1) = pixel on, Black (0) = pixel off.

---

## Rendering Semantics


---

## The Default Character

The default character is stored as two `u16` words (header \[9] and \[10]) representing a 4-byte UTF-8 character.

### Purpose of the Default Character

The default character acts as a fallback glyph rendered when a requested character is not found in the font's character set:
* **Ensures robustness**: Prevents rendering failures when encountering unknown characters.
* **Customizable appearance**: Designers may use a question mark, box, or other symbol.
* **Simplifies rendering logic**: Always returns a glyph, reducing error handling.
* **Essential for dynamic or external text**: E.g., user input, chat messages, or translations.
* **Ensures total mapping**: During runtime, every character has a guaranteed fallback glyph, avoiding lookup errors in the renderer.


---

## Validation Checklist

Use this checklist to verify that a `.cbf` file is valid:

* [ ] Header\[0] == 0xCBF0 (magic number)
* [ ] Header\[1] == 1 (a format version that you expect and know how to parse)


---

## Notes

* Bitmaps are compact and byte-aligned but not compressed.
* The format is stable and not currently designed for extensibility; implementers should reject files with unknown header versions.
* Always validate the magic number and version before parsing.
* The font metadata section (name, author, version, creation date) can be used in font selection UIs and diagnostics/debugging.

---

## License

Specification derived from the open-source Graph1 project.

---
