
async function register() {
    document.getElementById("loader").className = "loader";
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
    document.getElementById("loader").className = "";
}
