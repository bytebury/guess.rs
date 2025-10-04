document.body.addEventListener('htmx:sseBeforeMessage', function(event) {
  const data = event.detail.data;

  if (data.startsWith('event: disable_voting')) {
    disableVoting();
  } else if (data.startsWith('event: enable_voting')) {
    enableVoting();
  }
});

function disableVoting() {
  card_list.querySelectorAll('button').forEach(button => {
    button.disabled = true;
  });
}

function enableVoting() {
  card_list.querySelectorAll('button').forEach(button => {
    button.disabled = false;
    button.classList.remove('voted');
  });
}
