
async function get_tab() {
    fetch(
        "/tab/" + window.location.pathname.split("/")[2],
        {
            method: "GET"
        }
    ).then(resp => resp.text())
    .then(data => {
        const tab = JSON.parse(data);

        document.getElementById("tab_title").innerHTML = tab.title;
        document.getElementById("tab_contents").innerHTML = tab.tab;
    })
}

window.onload = get_tab();
