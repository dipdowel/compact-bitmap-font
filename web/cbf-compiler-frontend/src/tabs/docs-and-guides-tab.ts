import { marked } from 'marked';


export async function setupDocsAndGuidesTab() {

    const targetElement = document.getElementById("markdown-container");
    if(!targetElement){
        console.warn("Docs cannot be loaded!");
        return;
    }
    const response = await fetch("/font-designer-guide.cbf.md");
    const markdownText = await response.text();

    targetElement!.innerHTML = await marked(markdownText);
}
