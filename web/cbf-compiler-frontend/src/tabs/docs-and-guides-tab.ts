import { marked } from 'marked';


export async function setupDocsAndGuidesTab() {

    const targetElement = document.getElementById("markdown-container");
    if(!targetElement){
        console.warn("Docs cannot be loaded!");
        return;
    }
    const response = await fetch("/public/font-designer-guide.cbf.md");
    const markdownText = await response.text();

    targetElement!.innerHTML = await marked(markdownText);
    // console.log(markdownText);

}

console.log("docs-and-guides-tab.ts");