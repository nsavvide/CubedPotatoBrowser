document.querySelectorAll('.cef-hint-overlay').forEach(el => el.remove());

(() => {
  const HINT_STYLE = `
    position: absolute;
    background: rgba(255, 255, 0, 0.7);
    color: black;
    font-weight: bold;
    font-size: 11px;
    padding: 1px 4px;
    border-radius: 3px;
    z-index: 999999;
    pointer-events: none;
    line-height: 1;
    transform: translate(-50%, -50%);
    text-align: center;
    white-space: nowrap;
  `;

  const HOME_ROW_HINTS = [
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l',
    'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p',
    'z', 'x', 'c', 'v', 'b', 'n', 'm'
  ];
  const MAX_HINTS = HOME_ROW_HINTS.length ** 2;

  const genHintLabel = (i) => {
    if (i < HOME_ROW_HINTS.length) return HOME_ROW_HINTS[i];
    if (i < MAX_HINTS) {
      const a = Math.floor(i / HOME_ROW_HINTS.length);
      const b = i % HOME_ROW_HINTS.length;
      return HOME_ROW_HINTS[a] + HOME_ROW_HINTS[b];
    }
    return '';
  };

  const elements = [
    ...document.querySelectorAll('a, button, input, textarea, select, [role="button"], [tabindex]')
  ];

  const hints = [];
  let hintCount = 0;

  elements.forEach((el) => {
    const rect = el.getBoundingClientRect();
    if (rect.width === 0 || rect.height === 0) return;
    if (hintCount >= MAX_HINTS) return;

    const label = genHintLabel(hintCount++);
    const hint = document.createElement('div');
    hint.textContent = label;
    hint.classList.add('cef-hint-overlay');
    hint.setAttribute(
      'style',
      HINT_STYLE +
      `top: ${rect.top + window.scrollY + rect.height / 2}px;` +
      `left: ${rect.left + window.scrollX + rect.width / 2}px;`
    );

    document.body.appendChild(hint);
    hints.push({ label, el });
  });

  // Typed input buffer
  let input = '';

  const clearOverlay = () => {
    document.querySelectorAll('.cef-hint-overlay').forEach(el => el.remove());
    window.removeEventListener('keydown', onKeyDown);
  };

  const onKeyDown = (e) => {
    if (e.key === 'Escape') {
      clearOverlay();
      return;
    }

    const key = e.key.toLowerCase();
    if (!HOME_ROW_HINTS.includes(key)) return;

    input += key;

    const matches = hints.filter(h => h.label.startsWith(input));
    if (matches.length === 1 && matches[0].label === input) {
      const target = matches[0].el;
      clearOverlay();

      // Focus/click the target
      if (target instanceof HTMLElement) {
        target.focus();
        target.click();
      }

      // Send message to CEF Rust side
      if (window.cefMessage) {
        window.cefMessage('HintClick', target.href || target.getAttribute('href') || target.action || target.dataset.href || target.value || '');
      } else if (typeof cef !== 'undefined') {
        const msg = cef.createProcessMessage('HintClick');
        msg.argumentList.setString(0, target.href || target.getAttribute('href') || '');
        cef.sendProcessMessage('browser', msg);
      }
    } else if (matches.length === 0) {
      input = ''; // reset if no match
    }
  };

  window.addEventListener('keydown', onKeyDown);
})();
