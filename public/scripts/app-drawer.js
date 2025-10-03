function openAppDrawer() {
    const drawer = document.getElementById("app_drawer");
    const underlay = document.getElementById("app_drawer_underlay");
    drawer.style.transform = "translateX(0)";
    underlay.style.display = "block";
}

function closeAppDrawer() {
    const drawer = document.getElementById("app_drawer");
    const underlay = document.getElementById("app_drawer_underlay");
    drawer.style.transform = "translateX(100%)";
    underlay.style.display = "none";
}