// Tauri IPC bridge - replaces Electron contextBridge window.api
(function() {
  const invoke = window.__TAURI__.core.invoke;
  const listen = window.__TAURI__.event.listen;

  window.tauriApi = {
    generateHashes: (text) => invoke('generate_hashes', { text }),
    hashFile: (filePath) => invoke('hash_file', { filePath }),
    stopFileWatch: () => invoke('stop_file_watch'),
    onFileHashUpdate: (callback) => listen('file-hash-update', (event) => callback(event.payload)),
    generateQR: (text) => invoke('generate_qr', { text }),
    renderMarkdown: (md) => invoke('render_markdown', { md }),
    openFileDialog: () => invoke('open_file_dialog'),
    convertYaml: (input, mode) => invoke('convert_yaml', { input, mode }),
    formatSql: (sql, language) => invoke('format_sql', { sql, language }),
    describeCron: (expression) => invoke('describe_cron', { expression }),
  };
})();
