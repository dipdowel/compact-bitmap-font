export function setupTabs() {
  const tabButtons = document.querySelectorAll('nav.tab-bar button');
  const sections = document.querySelectorAll('section.tab-section');

  tabButtons.forEach(button => {
    button.addEventListener('click', () => {
      const targetId = button.getAttribute('data-tab');

      sections.forEach(section => {
        section.classList.toggle('active', section.id === targetId);
      });

      tabButtons.forEach(btn => {
        btn.classList.toggle('active-tab', btn === button);
      });
    });
  });

  // Activate default tab on load
  const defaultButton = document.querySelector('nav.tab-bar button[data-tab="font-compiler-section"]');
  if (defaultButton) defaultButton.classList.add('active-tab');
}
