
var a;

async function login() {
    document.getElementById("loader").className = "loader";
    var body = "username=" + document.getElementById("username").value
        + "&password=" + document.getElementById("password").value;
    const resp = await fetch(
        "login",
        {
            method: "POST",
            body: body,
        }
    );
    if (resp.status == 200) {
        document.cookie = "token=" + resp.headers.get("token");
        window.location.href = "/";
    }
    document.getElementById("loader").className = "";
}
