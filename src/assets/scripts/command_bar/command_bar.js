(function() {
  const existing = document.getElementById('cef-command-bar');
  const path = window.location.href;

  if (!existing) {
    const bar = document.createElement('div');
    bar.id = 'cef-command-bar';
    bar.style.cssText = `
      position: fixed;
      bottom: 0;
      left: 0;
      right: 0;
      height: 20px;
      background: rgba(0, 0, 0, 0.85);
      color: #aaffaa;
      font-family: monospace;
      font-size: 13px;
      padding-left: 8px;
      display: flex;
      align-items: center;
      z-index: 999999;
    `;
    bar.textContent = path;
    document.body.appendChild(bar);
  } else {
    existing.textContent = path;
  }
})();
