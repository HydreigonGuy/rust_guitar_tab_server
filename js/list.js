
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
            document.getElementById("list_container").innerHTML = document.getElementById("list_container").innerHTML + '<a class="tab_link" href="/list/' + tabs[i].id + '">' + tabs[i].title + "</a>";
        }
    })
}

window.onload = get_tabs();
