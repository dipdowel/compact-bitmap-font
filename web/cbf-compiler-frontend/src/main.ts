import './style.css';
import { setupTabs } from './tabs/tabs';
import { setupFontCompilerTab } from './tabs/font-compiler-tab';
import { setupFontViewerTab } from './tabs/font-viewer-tab';
import { setupDocsAndGuidesTab } from './tabs/docs-and-guides-tab';

document.addEventListener('DOMContentLoaded', () => {
    setupTabs();
    setupFontCompilerTab();
    setupFontViewerTab();
    setupDocsAndGuidesTab();
});