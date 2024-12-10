
async function get_tabs() {
    fetch(
        "tab_list",
        {
            method: "GET"
        }
    ).then(resp => resp.text())
    .then(data => {
        const tabs = JSON.parse(data).res;

        document.getElementById("list_container").innerHTML = "";
        for (i = 0; i < tabs.length; i++) {
            document.getElementById("list_container").innerHTML = document.getElementById("list_container").innerHTML + "<p>" + tabs[i].title + "</p>";
        }
    })
}

window.onload = get_tabs();
