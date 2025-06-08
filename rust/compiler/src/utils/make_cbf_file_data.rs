/// Serializes font header and body into a `Vec<u8>` using little-endian encoding.
///
/// # Arguments
/// * `font_header` - Slice of 16-bit words representing the font header.
/// * `font_body` - Slice of raw bytes representing the font body.
///
/// # Returns
/// * `Vec<u8>` - Serialized font data.
pub fn make_cbf_file_data(font_header: &[u16], font_body: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(font_header.len() * 2 + font_body.len());

    for &num in font_header {
        buffer.extend_from_slice(&num.to_le_bytes()); // Write u16 as 2 LE bytes
    }

    buffer.extend_from_slice(font_body); // font_body is already bytes

    buffer
}
