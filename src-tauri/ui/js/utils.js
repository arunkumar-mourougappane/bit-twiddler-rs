// Shared utilities (global - no ES module exports for Tauri static serving)
window.escHtml = (s) => String(s)
  .replace(/&/g, '&amp;')
  .replace(/</g, '&lt;')
  .replace(/>/g, '&gt;')
  .replace(/"/g, '&quot;');

window.copyToClipboard = async (text, $btnElement) => {
  try {
    await navigator.clipboard.writeText(text);
    const originalText = $btnElement.text();
    $btnElement.text("Copied!");
    setTimeout(() => { $btnElement.text(originalText); }, 1500);
  } catch (err) {
    console.error('Failed to copy', err);
  }
};
