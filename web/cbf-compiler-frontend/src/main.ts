

document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('fontForm') as HTMLFormElement;
    const pngInput = document.getElementById('pngInput') as HTMLInputElement;
    const jsonInput = document.getElementById('jsonInput') as HTMLInputElement;
    const message = document.getElementById('formMessage') as HTMLDivElement;

    form.addEventListener('submit', (e) => {
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

        // If all checks pass
        showMessage('Files are valid and ready to be uploaded!', true);

        // Optionally: process the files here (read contents, send to backend, etc.)
    });

    function showMessage(msg: string, success: boolean) {
        message.style.display = 'block';
        message.style.color = success ? 'lime' : 'tomato';
        message.textContent = msg;
    }
});
