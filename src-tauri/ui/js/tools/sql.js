$(document).ready(function() {
  const $input = $('#sql-input');
  const $output = $('#sql-output');
  const $dialect = $('#sql-dialect');
  const $error = $('#sql-error');

  const process = async () => {
    const val = $input.val().trim();
    if (!val) {
      $output.val('');
      $error.addClass('hidden');
      return;
    }

    try {
      const formatted = await window.tauriApi.formatSql(val, $dialect.val());
      $output.val(formatted);
      $error.addClass('hidden');
    } catch (e) {
      $error.removeClass('hidden').text(`Error: ${e}`);
      $output.val('');
    }
  };

  $input.on('input', process);
  $dialect.on('change', process);

  $('#sql-clear-btn').on('click', () => {
    $input.val('').focus();
    $output.val('');
    $error.addClass('hidden');
  });

  $('#sql-copy-btn').on('click', function() {
    const text = $output.val();
    if (text) {
      window.copyToClipboard(text, $(this));
    }
  });
});
