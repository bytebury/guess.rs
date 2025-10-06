document.body.addEventListener('htmx:wsBeforeMessage', function(event) {
  const data = event.detail.message;

  if (!data.startsWith('event')) return;

  const event_name = data.split(' ')[1].split('\n')[0];

  switch (event_name) {
    case "disable_voting":
      disableVoting();
      break;
    case "enable_voting":
      enableVoting();
      break;
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
