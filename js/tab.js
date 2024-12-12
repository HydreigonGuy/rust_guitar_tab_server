
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
        var tab_contents = "";
        for (i = 0; i < tab.tab[0].length; i++) {
            tab_contents = tab_contents + '<div class="static_tab_row">'
            for (j = 0; j < 6; j++) {
                if (tab.tab[j][i] == 0)
                    tab_contents = tab_contents + '<p class="static_tab_bit">-</p>'
                else
                tab_contents = tab_contents + '<p class="static_tab_bit">' + tab.tab[j][i] + "</p>"
            }
            tab_contents = tab_contents + "</div>"
        }
        document.getElementById("tab_contents").innerHTML = tab_contents;
    })
}

window.onload = get_tab();
