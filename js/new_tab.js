
async function create_tab() {
    var title = document.getElementById("tab_name").value;

    document.getElementById("error_msg").innerHTML = "";
    if (title == "") {
        document.getElementById("error_msg").innerHTML = "Please fill in the name of your tab";
        return;
    }
    console.log(title);
    const resp = await fetch(
        "new_tab",
        {
            method: "POST",
            body: JSON.stringify({ title: title }),
        }
    );
    console.log(resp);
}
