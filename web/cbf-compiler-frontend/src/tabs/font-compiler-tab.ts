
import {validateFontConfig} from "../validate-font-config.ts";

console.log("font-compiler-tab.ts");

function showMessage(msg: string, success = false): void {
  const el = document.getElementById("formMessage")!;
  el.style.display = "block";
  el.style.color = success ? "green" : "red";
  el.innerText = msg;
}

export function setupFontCompilerTab()  {

  const form = document.getElementById("font-assets-form") as HTMLFormElement;
  const pngInput = document.getElementById("png_input") as HTMLInputElement;
  const jsonInput = document.getElementById("json_input") as HTMLInputElement;

  form.addEventListener("submit", async (event) => {
    event.preventDefault();

    const pngFile = pngInput.files?.[0];
    const jsonFile = jsonInput.files?.[0];

    if (!pngFile || !jsonFile) {
      showMessage("Please select both PNG and JSON files.", false);
      return;
    }

    const isPNG = pngFile && pngFile.type === 'image/png';
    const isJSON = jsonFile && jsonFile.type === 'application/json';

    if (!isPNG) {
      showMessage('The first file must be a PNG.', false);
      return;
    }
    if (!isJSON) {
      showMessage('The second file must be a JSON.', false);
      return;
    }

    try {
      const jsonText = await jsonFile.text();
      const jsonData = JSON.parse(jsonText);
      const result = validateFontConfig(jsonData);
      if (!result.valid) {
        showMessage(`JSON is invalid: ${result.message}`, false);
        return;
      }
    } catch (err) {
      showMessage('Failed to read or parse JSON file.', false);
      return;
    }



    showMessage("Files are valid and ready to be uploaded!", true);

    const formData = new FormData();
    formData.append("png_input", pngFile);
    formData.append("json_input", jsonFile);

    try {
      const res = await fetch("http://127.0.0.1:3033/upload", {
      // const res = await fetch("/da_font_upload.php", {
        method: "POST",
        body: formData,
      });

      if (!res.ok) {
        const errorBody = await res.text();
        throw new Error(`Upload failed: ${res.statusText} ${errorBody}` );
      }

      const blob = await res.blob();
      const url = URL.createObjectURL(blob);

      const a = document.createElement("a");
      a.href = url;
      a.download = "result.zip";
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);

      showMessage("Upload successful! Downloading ZIP...", true);
    } catch (err: any) {
      showMessage("Upload error: " + err.message, false);
      console.error(err);
    }
  });

}
