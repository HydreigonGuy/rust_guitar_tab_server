
function logout() {
    document.cookie = 'token=';
    window.location.href = "/login";
}
