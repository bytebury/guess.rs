const cookies = Object.fromEntries(
  document.cookie.split("; ").map(c => {
    const [k, v] = c.split("=");
    return [k, decodeURIComponent(v)];
  })
);

if (cookies["guess_rs_display_name"] === 'Guest') {
  const nameSwapModal = document.querySelector("#name_swap");

  if (nameSwapModal) {
    nameSwapModal.setAttribute("hx-trigger", "load");
  }
}
