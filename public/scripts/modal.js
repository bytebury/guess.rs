document.addEventListener("closeModal", function () {
	closeModal();
});

document.addEventListener("htmx:afterSwap", function (evt) {
	if (evt.target.id === "modal") {
		document.getElementById("modal_wrapper").style.display = "flex";
	}
});

function closeModal() {
	const modal = document.getElementById("modal_wrapper");

	modal.classList.add("closing");

	modal.addEventListener("animationend", function handleAnimationEnd() {
		modal.classList.remove("closing");
		modal.style.display = "none";
		modal.removeEventListener("animationend", handleAnimationEnd);
	});
}

