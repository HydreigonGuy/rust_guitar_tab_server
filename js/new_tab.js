
function create_tab() {
    var title = document.getElementById("tab_name").value;

    if (title == "") {
        document.getElementById("error_msg").innerHTML = "Please fill in the name of your tab";
    } else {
        document.getElementById("error_msg").innerHTML = "";
        console.log(title);
    }
}
