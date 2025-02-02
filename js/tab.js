
async function get_tab() {
    fetch(
        "/tab/" + window.location.pathname.split("/")[2],
        {
            method: "GET"
        }
    ).then(resp => resp.text())
    .then(data => {
        const tab = JSON.parse(data.replaceAll("\n", ""));

        document.getElementById("tab_title").innerHTML = tab.title;
        document.getElementById("tab_comment").innerHTML = tab.comment;
        var tab_contents = "";
        for (i = 0; i < tab.tab[0].length; i++) {
            tab_contents = tab_contents + '<div class="static_tab_row">'
            for (j = 0; j < 6; j++) {
                if (tab.tab[j][i] > 99) {
                    if (tab.tab[j][i] == 100)
                        tab_contents = tab_contents + '<p class="static_tab_bit">-</p>'
                    if (tab.tab[j][i] == 101)
                        tab_contents = tab_contents + '<p class="static_tab_bit">X</p>'
                    if (tab.tab[j][i] == 102)
                        tab_contents = tab_contents + '<p class="static_tab_bit">~</p>'
                    if (tab.tab[j][i] == 103)
                        tab_contents = tab_contents + '<p class="static_tab_bit">/</p>'
                 } else
                    tab_contents = tab_contents + '<p class="static_tab_bit">' + tab.tab[j][i] + "</p>"
            }
            tab_contents = tab_contents + "</div>"
        }
        document.getElementById("tab_contents").innerHTML = tab_contents;
        if (tab.owner == 1) {
            document.getElementById("owner_options").innerHTML = '<button onclick="delete_tab()">delete</button>'
        }
    })
}

async function delete_tab() {
    fetch(
        "/delete/" + window.location.pathname.split("/")[2],
        {
            method: "GET"
        }
    ).then(resp => resp.text())
    .then(_ => {
        window.location.href = "/";
    })
}

window.onload = get_tab();
