$(document).ready(function() {
  const $input = $('#yaml-input');
  const $output = $('#yaml-output');
  const $error = $('#yaml-error');
  const $outputLabel = $('#yaml-output-label');

  let inputMode = 'yaml'; // 'yaml' or 'json'

  const updateModeUI = () => {
    const isYaml = inputMode === 'yaml';
    $('#yaml-input-type-yaml')
      .toggleClass('bg-blue-600 text-white shadow-sm', isYaml)
      .toggleClass('text-gray-500 hover:text-gray-300', !isYaml);

    $('#yaml-input-type-json')
      .toggleClass('bg-blue-600 text-white shadow-sm', !isYaml)
      .toggleClass('text-gray-500 hover:text-gray-300', isYaml);

    $outputLabel.text(isYaml ? 'JSON Output' : 'YAML Output');
    $output.toggleClass('text-blue-300', isYaml).toggleClass('text-emerald-300', !isYaml);

    convert();
  };

  const convert = async () => {
    const val = $input.val().trim();
    if (!val) {
      $output.val('');
      $error.addClass('hidden');
      return;
    }

    try {
      const result = await window.tauriApi.convertYaml(val, inputMode === 'yaml' ? 'yaml-to-json' : 'json-to-yaml');
      $output.val(result);
      $error.addClass('hidden');
    } catch (e) {
      $error.removeClass('hidden').text(`Error: ${e}`);
      $output.val('');
    }
  };

  $('#yaml-input-type-yaml').on('click', () => {
    inputMode = 'yaml';
    updateModeUI();
  });

  $('#yaml-input-type-json').on('click', () => {
    inputMode = 'json';
    updateModeUI();
  });

  $input.on('input', convert);

  $('#yaml-clear-btn').on('click', () => {
    $input.val('').focus();
    $output.val('');
    $error.addClass('hidden');
  });

  $('#yaml-copy-btn').on('click', function() {
    const text = $output.val();
    if (text) {
      window.copyToClipboard(text, $(this));
    }
  });

  // Initialize
  updateModeUI();
});
