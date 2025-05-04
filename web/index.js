const CANVAS_WIDTH = 1200;
const CANVAS_HEIGHT = 600;

const marginX = 10, marginY = 10;

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");
let currentFont = null;

async function loadFont(url) {
    const res = await fetch(url);
    const buf = await res.arrayBuffer();
    const view = new DataView(buf);

    if (view.getUint16(0, true) !== 0xF0CB || view.getUint16(2, true) !== 1) {
        throw new Error("Invalid file");
    }

    const header = [];
    for (let i = 0; i < 14; i++) header.push(view.getUint16(i * 2, true));

    const [, , fontNameSize, authorSize, charOrderSize, charWidthsSize,
        fontImageWidth, fontImageHeight, spacingProps, , , , year, monthDay
    ] = header;

    let offset = 28;

    const readStr = (len) => {
        const str = new TextDecoder().decode(new Uint8Array(buf, offset, len));
        offset += len;
        return str;
    };

    const fontName = readStr(fontNameSize);
    const author = readStr(authorSize);
    const charOrderStr = readStr(charOrderSize);
    const charOrder = Array.from(charOrderStr);

    const widths = new Uint8Array(buf, offset, charWidthsSize);
    offset += charWidthsSize;

    const kerning = spacingProps & 0xFF;
    const leading = spacingProps >> 8;

    const rowBytes = Math.ceil(fontImageWidth / 8);
    const bitmap = new Uint8Array(buf, offset);

    return {
        fontName, author, charOrder, widths, bitmap,
        fontImageWidth, fontImageHeight, kerning, leading,
        year, month: monthDay >> 8, day: monthDay & 0xFF
    };
}

async function updateFont() {
    try {
        const fontFile = document.getElementById("fontSelect").value;
        currentFont = await loadFont(fontFile);
        const md = currentFont;
        document.getElementById("metadata").innerHTML =
            `Font: ${md.fontName}<br/>Author: ${md.author}<br />` +
            `Created: ${md.year}-${String(md.month).padStart(2, '0')}-${String(md.day).padStart(2, '0')}<br/>` +
            `Glyphs: ${md.charOrder.length}<br/>` +
            `Bitmap: ${md.fontImageWidth}x${md.fontImageHeight}<br />` +
            `Kerning: ${md.kerning}, Leading: ${md.leading}`;

        updateRender();

    } catch (err) {
        console.error(err);
        alert("Failed to load font: " + err.message);
    }
}


function updateRender() {
    const scale = parseInt(document.getElementById("scaleSelect").value, 10);
    const text = document.getElementById("inputText").value;

    if (!currentFont) {
        return
    }

    canvas.width = CANVAS_WIDTH;
    canvas.height = CANVAS_HEIGHT;
    ctx.fillStyle = "#0d1117";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    const {fontImageWidth, fontImageHeight, bitmap, charOrder, widths, kerning} = currentFont;
    const glyphMap = {};

    let x = 0;
    for (let i = 0; i < widths.length; i++) {
        glyphMap[charOrder[i]] = {
            x,
            w: widths[i]
        };
        x += widths[i] + 1;
    }

    let drawX = 0;
    ctx.fillStyle = "#ffffff";

    for (const ch of Array.from(text)) {
        const g = glyphMap[ch];
        if (!g) continue;

        // Draw glyph pixels
        for (let y = 0; y < fontImageHeight; y++) {
            for (let x = 0; x < g.w; x++) {
                const bitIndex = y * fontImageWidth + g.x + x;
                const byte = bitmap[bitIndex >> 3];
                const bit = (byte >> (7 - (bitIndex % 8))) & 1;
                if (!bit) {
                    ctx.fillRect((drawX + x) * scale, y * scale, scale, scale);
                }
            }
        }
        drawX += g.w + kerning;
    }

    // Apply the margins
    const imageData = ctx.getImageData(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    ctx.clearRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    ctx.putImageData(imageData, marginX, marginY);



}

document.getElementById("fontSelect").addEventListener("change", updateFont);
document.getElementById("scaleSelect").addEventListener("change", updateRender);
document.getElementById("inputText").addEventListener("input", updateRender);

updateFont();


document.title = "CBF Font Viewer  [ " + new Date().toLocaleTimeString() + " ]";