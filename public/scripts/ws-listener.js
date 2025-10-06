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

document.body.addEventListener('htmx:wsAfterMessage', function() {
  if (toggle_votes_btn && toggle_votes_btn.textContent.trim() === 'Start Voting') {
    disableVoting();
  } else {
    card_list.querySelectorAll('button').forEach(button => {
      button.disabled = false;
    });
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
