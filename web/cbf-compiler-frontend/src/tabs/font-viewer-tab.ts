export function setupFontViewerTab() {
// Handle font upload
    const fontUpload = document.getElementById("fontUpload") as HTMLInputElement;
    fontUpload.addEventListener("change", (event) => {
        const file = (event.target as HTMLInputElement).files?.[0];
        if (file) loadFontFromFile(file);
    });

}
/**
 * Font spacing properties (typography).
 * Mirrors Rust's `Spacing` struct.
 */
interface Spacing {
    kerning_px: number;
    leading_px: number;
}

/**
 * Metadata associated with the font.
 * Mirrors Rust's `PixelFontMeta`.
 */
interface PixelFontMeta {
    fontName: string;
    author: string;
    year: number;
    month: number;
    day: number;
}

/**
 * A glyph's location and width in the bitmap.
 */
interface GlyphRect {
    x: number;
    w: number;
}

/**
 * Loaded and parsed font data.
 * Mirrors Rust's `PixelFont`.
 */
interface PixelFont {
    fontImageWidth: number;
    fontImageHeight: number;
    bitmap: Uint8Array;
    charOrder: string[];
    widths: Uint8Array;
    spacing: Spacing;
    defaultChar: string;
    meta: PixelFontMeta;
    byteSize: number;
}

const CANVAS_WIDTH = 1200;
const CANVAS_HEIGHT = 600;
const marginX = 10;
const marginY = 10;

let currentFont: PixelFont | null = null;

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

const inputText = document.getElementById("inputText") as HTMLInputElement;
const fontUpload = document.getElementById("fontUpload") as HTMLInputElement;
const scaleSelect = document.getElementById("scaleSelect") as HTMLSelectElement;
const metadata = document.getElementById("metadata")!;

function decodeDefaultChar(defaultCharHigher:number, defaultCharLower:number):string {
    let bytes;

    if (defaultCharHigher === 0) {
        // Single-byte character (like ASCII)
        bytes = new Uint8Array([defaultCharLower]);
    } else {
        // Likely multi-byte UTF-8
        bytes = new Uint8Array([defaultCharHigher, defaultCharLower]);
    }

    return new TextDecoder().decode(bytes);
}


/**
 * Parses a raw ArrayBuffer in .cbf format into a PixelFont.
 */
function loadFontFromBuffer(buf: ArrayBuffer): PixelFont {
    const view = new DataView(buf);

    if (view.getUint16(0, true) !== 0xF0CB || view.getUint16(2, true) !== 1) {
        throw new Error("Invalid CBF file");
    }

    const header: number[] = [];
    for (let i = 0; i < 14; i++) {
        header.push(view.getUint16(i * 2, true));
    }

    // @ts-ignore
    const [magicNumber, cbfVersion , fontNameSize, authorSize, charOrderSize, charWidthsSize,
        // @ts-ignore
        fontImageWidth, fontImageHeight, spacingProps, defaultCharLower , defaultCharHigher ,fontVersion , year, monthDay
    ] = header;

    let offset = 28;

    const readStr = (len: number): string => {
        const str = new TextDecoder().decode(new Uint8Array(buf, offset, len));
        offset += len;
        return str;
    };

    const fontName = readStr(fontNameSize);
    const author = readStr(authorSize);
    const charOrder = Array.from(readStr(charOrderSize));
    const widths = new Uint8Array(buf, offset, charWidthsSize);
    offset += charWidthsSize;

    const kerning = spacingProps & 0xff;
    const leading = spacingProps >> 8;
    const bitmap = new Uint8Array(buf, offset);

    // Decode the default char from 2 bytes
    const defaultChar = decodeDefaultChar(defaultCharHigher, defaultCharLower);
    return {
        fontImageWidth,
        fontImageHeight,
        bitmap,
        charOrder,
        widths,
        spacing: { kerning_px: kerning, leading_px: leading },
        defaultChar,
        meta: {
            fontName,
            author,
            year,
            month: monthDay >> 8,
            day: monthDay & 0xff,
        },
        byteSize: buf.byteLength
    };
}

/**
 * Loads and parses a .cbf file from a user-uploaded File.
 */
function loadFontFromFile(file: File): void {
    const reader = new FileReader();
    reader.onload = () => {
        try {
            if (!reader.result) {
                throw new Error("Empty result");
            }

            const buf = reader.result as ArrayBuffer;
            currentFont = loadFontFromBuffer(buf);
            updateFontInfoUI();
            updateRender();
        } catch (err) {
            alert("Failed to load font: " + (err as Error).message);
        }
    };
    reader.readAsArrayBuffer(file);
}

/**
 * Updates the metadata section of the UI with the current font info.
 */
function updateFontInfoUI(): void {
    if (!currentFont) return;
    const md = currentFont.meta;
    metadata.innerHTML =
        `Font: ${md.fontName}<br/>Author: ${md.author}<br/>` +
        `Created: ${md.year}-${String(md.month).padStart(2, '0')}-${String(md.day).padStart(2, '0')}<br/>` +
        `Glyphs: ${currentFont.charOrder.length}<br/>` +
        `Total pixels: ${currentFont.fontImageWidth * currentFont.fontImageHeight}<br/>` +
        `File size: ${currentFont.byteSize} byte(s)<br/>` +
        `Kerning: ${currentFont.spacing.kerning_px}, Leading: ${currentFont.spacing.leading_px}`;
}

/**
 * Renders the current input text to the canvas using the loaded font.
 */
function updateRender(): void {
    if (!currentFont) return;

    const scale = parseInt(scaleSelect.value, 10);
    const text = inputText.value;

    canvas.width = CANVAS_WIDTH;
    canvas.height = CANVAS_HEIGHT;
    ctx.fillStyle = "#0d1117";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    const { fontImageWidth, fontImageHeight, bitmap, charOrder, widths, spacing, defaultChar } = currentFont;

    const glyphMap: Record<string, GlyphRect> = {};
    let xOffset = 0;
    for (let i = 0; i < widths.length; i++) {
        glyphMap[charOrder[i]] = { x: xOffset, w: widths[i] };
        xOffset += widths[i];
    }

    let drawX = 0;
    ctx.fillStyle = "#ffffff";

    for (const ch of text) {
        let glyph = glyphMap[ch];

        if (!glyph) {
            glyph = glyphMap[defaultChar];
        }

        for (let y = 0; y < fontImageHeight; y++) {
            for (let x = 0; x < glyph.w; x++) {
                const bitIndex = y * fontImageWidth + glyph.x + x;
                const byte = bitmap[bitIndex >> 3];
                const bit = (byte >> (7 - (bitIndex % 8))) & 1;
                if (!bit) {
                    ctx.fillRect((drawX + x) * scale, y * scale, scale, scale);
                }
            }
        }

        drawX += glyph.w + spacing.kerning_px;
    }

    const imageData = ctx.getImageData(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    ctx.clearRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    ctx.putImageData(imageData, marginX, marginY);
}

// === DOM bindings ===

inputText.addEventListener("input", updateRender);
scaleSelect.addEventListener("change", updateRender);
fontUpload.addEventListener("change", (e) => {
    const file = (e.target as HTMLInputElement).files?.[0];
    if (file) loadFontFromFile(file);
});

// Set random greeting
const GREETINGS = [
    "Hi! Welcome to Cranky Bellhop Fiesta! :-)",
    "Hey! Explore Cosmic Bubblegum Fiasco! :D",
    "Yo! Check out Caffeinated Banana Frenzy! ;-)",
    "Hi! Dive into Cornfield Banana Freefall! <(^.^)>"
];
inputText.value = GREETINGS[Math.floor(Math.random() * GREETINGS.length)];

// Set page title
document.title = "CBF Font Viewer [ " + new Date().toLocaleTimeString() + " ]";
