
async function login() {
    var body = "username=" + document.getElementById("username").value
        + "&password=" + document.getElementById("password").value;
    const resp = await fetch(
        "login",
        {
            method: "POST",
            body: body,
        }
    );
    console.log(resp);
    if (resp.status == 200) {
        window.location.href = "/"
    }
}
