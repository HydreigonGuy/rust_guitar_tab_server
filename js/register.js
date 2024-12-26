
async function register() {
    var body = "username=" + document.getElementById("username").value
        + "&password=" + document.getElementById("password").value;
    const resp = await fetch(
        "register",
        {
            method: "POST",
            body: body,
        }
    );
    console.log(resp);
    if (resp.status == 200) {
        document.cookie = "token=" + resp.headers.get("token");
        window.location.href = "/"
    }
}
