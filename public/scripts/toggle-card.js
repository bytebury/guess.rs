function toggleCard(event) {
  if (event.target.classList.contains('voted')) {
    event.target.classList.remove('voted');
    return;
  }

  for (const element of card_list.querySelectorAll('.card')) {
    element.classList.remove('voted');
  }
  event.target.classList.add('voted');
}
