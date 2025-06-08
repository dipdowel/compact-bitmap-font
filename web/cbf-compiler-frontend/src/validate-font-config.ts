export function validateFontConfig(json: any): { valid: boolean; message?: string } {
    if (typeof json !== 'object' || json === null) {
        return { valid: false, message: 'Invalid JSON structure.' };
    }

    if (typeof json.char_order !== 'string') {
        return { valid: false, message: '"char_order" must be a string.' };
    }

    if (typeof json.default_char !== 'string') {
        return { valid: false, message: '"default_char" must be a string.' };
    }

    if (typeof json.spacing !== 'object' || json.spacing === null) {
        return { valid: false, message: '"spacing" must be an object.' };
    }

    if (typeof json.spacing.kerning_px !== 'number' || typeof json.spacing.leading_px !== 'number') {
        return { valid: false, message: '"spacing.kerning_px" and "spacing.leading_px" must be numbers.' };
    }

    if (typeof json.meta !== 'object' || json.meta === null) {
        return { valid: false, message: '"meta" must be an object.' };
    }

    const metaFields = [
        'font_ver',
        'date_year',
        'date_month',
        'date_day',
        'font_name',
        'author_signature'
    ];

    for (const key of metaFields) {
        if (!(key in json.meta)) {
            return { valid: false, message: `"meta.${key}" is missing.` };
        }
    }

    if (!Array.isArray(json.sample_text)) {
        return { valid: false, message: '"sample_text" must be an array.' };
    }

    return { valid: true };
}
