// About modal — open/close logic, version fetching, and link copying
(function () {
  const modal    = document.getElementById('about-modal');
  const backdrop = document.getElementById('about-backdrop');
  const closeBtn = document.getElementById('about-close');
  const openBtn  = document.getElementById('about-open');
  const verEl    = document.getElementById('about-version');
  const card     = modal && modal.querySelector('.about-card');

  if (!modal || !card) return;

  function open() {
    // Show the container first (display: block), then animate on next frame
    // so the browser has an initial state to transition FROM.
    modal.classList.remove('hidden');
    requestAnimationFrame(() => requestAnimationFrame(() => {
      backdrop.classList.remove('opacity-0');
      card.classList.remove('scale-95', 'opacity-0');
    }));
  }

  function close() {
    backdrop.classList.add('opacity-0');
    card.classList.add('scale-95', 'opacity-0');
    setTimeout(() => modal.classList.add('hidden'), 200);
  }

  openBtn.addEventListener('click', open);
  closeBtn.addEventListener('click', close);
  backdrop.addEventListener('click', (e) => {
    if (e.target === backdrop) close();
  });
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && !modal.classList.contains('hidden')) close();
  });

  // Copy URL to clipboard on link button click
  modal.querySelectorAll('.about-copy-link').forEach((btn) => {
    btn.addEventListener('click', () => {
      const url = btn.dataset.url;
      if (!url) return;
      navigator.clipboard.writeText(url).then(() => {
        const label = btn.querySelector('.about-copy-label');
        if (!label) return;
        const orig = label.textContent;
        label.textContent = 'Copied!';
        label.classList.add('text-green-500');
        setTimeout(() => {
          label.textContent = orig;
          label.classList.remove('text-green-500');
        }, 1500);
      });
    });
  });

  // Fetch app version from Tauri at runtime
  if (window.__TAURI__?.app?.getVersion) {
    window.__TAURI__.app.getVersion().then((v) => {
      if (verEl) verEl.textContent = 'v' + v;
    }).catch(() => {});
  }
})();
