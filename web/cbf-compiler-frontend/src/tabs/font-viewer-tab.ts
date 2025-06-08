export function setupFontViewerTab() {
  // To be implemented
}

console.log("font-viewer-tab.ts");


/**
 * Font spacing properties (typography).
 * Equivalent to Rust's `Spacing` struct.
 */
interface Spacing {
    /** Horizontal spacing between characters */
    kerning_px: number;
    /** Vertical spacing between lines of characters */
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
 * A single glyph's location and size within the bitmap.
 */
interface GlyphRect {
    x: number;
    w: number;
}

/**
 * Pixel font data loaded from a CBF file.
 * Equivalent to Rust's `PixelFont` structure.
 */
interface PixelFont {
    fontImageWidth: number;
    fontImageHeight: number;
    bitmap: Uint8Array;
    charOrder: string[];
    widths: Uint8Array;
    spacing: Spacing;
    meta: PixelFontMeta;
    byteSize: number;
}

/** Greetings list to prefill input text */
const GREETINGS = [
    "Hi! Welcome to Cranky Bellhop Fiesta! :-)",
    "Hey! Explore Cosmic Bubblegum Fiasco! :D",
    "Yo! Check out Caffeinated Banana Frenzy! ;-)",
    "Hi! Dive into Cornfield Banana Freefall! <(^.^)>",
    "Hey! Browse Cobweb Ballet Festival! *_*",
    "Yo! Step into Caffeine Boost Factory! >_>",
    "Hi! Enjoy Clumsy Buffalo Foxtrot! <3",
    "Hey! Discover Cookie Blast Frenzy! ^_^",
    "Yo! Tour Creepy Basement Funhouse! :-O",
    "Hi! Explore Coconut Blanket Fortress! :P",
    "Hey! Dive into Cheetah Ballet Flashmob! :3",
    "Yo! Enter Croissant Baking Fiesta! (^)o(^)",
    "Hi! Check out Cartoon Banana Firetruck! XD",
    "Hey! Browse Cybernetic Burrito Forge! o_O",
    "Yo! Join Cackling Beagle Fandango! T_T",
    "Hi! Discover Crystal Broccoli Farm! ;-P",
    "Hey! Explore Cosmic Blizzard Fervor! >.<",
    "Yo! Check Chilly Biscuit Fountain! :)",
    "Hi! Try Circuit Board Fairground! :-|",
    "Hey! Browse Curvy Beanstalk Fantasy! ;)"
];

const CANVAS_WIDTH = 1200;
const CANVAS_HEIGHT = 600;
const marginX = 10;
const marginY = 10;

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;
let currentFont: PixelFont | null = null;

/**
 * Loads a CBF font file and decodes its binary structure.
 *
 * @param url URL to the font file in CBF format
 * @returns A Promise that resolves to a typed PixelFont structure
 */
async function loadFont(url: string): Promise<PixelFont> {
    const res = await fetch(url);
    const buf = await res.arrayBuffer();
    const view = new DataView(buf);

    // Check CBF magic number and version
    if (view.getUint16(0, true) !== 0xF0CB || view.getUint16(2, true) !== 1) {
        throw new Error("Invalid font file format.");
    }

    // Read fixed-size header (14 * 2 = 28 bytes)
    const header: number[] = [];
    for (let i = 0; i < 14; i++) {
        header.push(view.getUint16(i * 2, true));
    }

    const [, , fontNameSize, authorSize, charOrderSize, charWidthsSize,
        fontImageWidth, fontImageHeight, spacingProps, , , , year, monthDay
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

    const kerning = spacingProps & 0xFF;
    const leading = spacingProps >> 8;

    const bitmap = new Uint8Array(buf, offset);

    return {
        fontImageWidth,
        fontImageHeight,
        bitmap,
        charOrder,
        widths,
        spacing: { kerning_px: kerning, leading_px: leading },
        meta: {
            fontName,
            author,
            year,
            month: monthDay >> 8,
            day: monthDay & 0xFF
        },
        byteSize: buf.byteLength
    };
}

/**
 * Updates the UI and canvas after loading a new font.
 */
async function updateFont(): Promise<void> {
    try {
        const fontFile = (document.getElementById("fontSelect") as HTMLSelectElement).value;
        currentFont = await loadFont(fontFile);

        const md = currentFont.meta;
        document.getElementById("metadata")!.innerHTML =
            `Font: ${md.fontName}<br/>Author: ${md.author}<br />` +
            `Created: ${md.year}-${String(md.month).padStart(2, '0')}-${String(md.day).padStart(2, '0')}<br/>` +
            `Glyphs: ${currentFont.charOrder.length}<br/>` +
            `Total pixels: ${currentFont.fontImageWidth * currentFont.fontImageHeight}<br />` +
            `File size: ${currentFont.byteSize} byte(s)<br />` +
            `Defaults: kerning: ${currentFont.spacing.kerning_px}, leading: ${currentFont.spacing.leading_px}`;

        updateRender();
    } catch (err) {
        console.error(err);
        alert("Failed to load font: " + (err as Error).message);
    }
}

/**
 * Renders the current input text using the loaded font onto the canvas.
 */
function updateRender(): void {
    if (!currentFont) return;

    const scale = parseInt((document.getElementById("scaleSelect") as HTMLSelectElement).value, 10);
    const text = (document.getElementById("inputText") as HTMLInputElement).value;

    canvas.width = CANVAS_WIDTH;
    canvas.height = CANVAS_HEIGHT;

    ctx.fillStyle = "#0d1117";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    const { fontImageWidth, fontImageHeight, bitmap, charOrder, widths, spacing } = currentFont;

    // Create glyph lookup
    const glyphMap: Record<string, GlyphRect> = {};
    let xOffset = 0;
    for (let i = 0; i < widths.length; i++) {
        glyphMap[charOrder[i]] = {
            x: xOffset,
            w: widths[i]
        };
        xOffset += widths[i];
    }

    let drawX = 0;
    ctx.fillStyle = "#ffffff";

    for (const ch of Array.from(text)) {
        const g = glyphMap[ch];
        if (!g) continue; // FIXME: Replace with a default char fallback.

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

        drawX += g.w + spacing.kerning_px;
    }

    // Shift the rendered image for margins
    const imageData = ctx.getImageData(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    ctx.clearRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    ctx.putImageData(imageData, marginX, marginY);
}

// === UI bindings ===

document.getElementById("fontSelect")?.addEventListener("change", updateFont);
document.getElementById("scaleSelect")?.addEventListener("change", updateRender);

const inputText = document.getElementById("inputText") as HTMLInputElement;
inputText.addEventListener("input", updateRender);
inputText.value = GREETINGS[Math.floor(Math.random() * GREETINGS.length)];

updateFont();
document.title = `CBF Font Viewer [ ${new Date().toLocaleTimeString()} ]`;
