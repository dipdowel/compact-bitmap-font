import './style.css';
import { validateFontConfig } from './validate-font-config';

document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('fontForm') as HTMLFormElement;
    const pngInput = document.getElementById('pngInput') as HTMLInputElement;
    const jsonInput = document.getElementById('jsonInput') as HTMLInputElement;
    const message = document.getElementById('formMessage') as HTMLDivElement;

    form.addEventListener('submit', async (e) => {
        e.preventDefault();

        const pngFile = pngInput.files?.[0];
        const jsonFile = jsonInput.files?.[0];

        const isPNG = pngFile && pngFile.type === 'image/png';
        const isJSON = jsonFile && jsonFile.type === 'application/json';

        if (!pngFile || !jsonFile) {
            showMessage('Please select both files.', false);
            return;
        }

        if (!isPNG) {
            showMessage('The first file must be a PNG.', false);
            return;
        }

        if (!isJSON) {
            showMessage('The second file must be a JSON.', false);
            return;
        }

        // ✅ Validate the JSON
        try {
            const jsonText = await jsonFile.text();
            const jsonData = JSON.parse(jsonText);
            const result = validateFontConfig(jsonData);

            if (!result.valid) {
                showMessage(`JSON invalid: ${result.message}`, false);
                return;
            }

            showMessage('Files are valid and ready to be uploaded!', true);
        } catch (err) {
            showMessage('Failed to read or parse JSON file.', false);
        }
    });

    function showMessage(msg: string, success: boolean) {
        message.style.display = 'block';
        message.style.color = success ? 'lime' : 'tomato';
        message.textContent = msg;
    }
});
