
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
    
    fetch(
        "public_tab_list",
        {
            method: "GET"
        }
    ).then(resp => resp.text())
    .then(data => {
        const tabs = JSON.parse(data).res;

        document.getElementById("public_list_container").innerHTML = "";
        for (i = 0; i < tabs.length; i++) {
            document.getElementById("public_list_container").innerHTML = document.getElementById("public_list_container").innerHTML + '<a class="tab_link" href="/list/' + tabs[i].id + '">' + tabs[i].title + "</a>";
        }
    })
}

window.onload = get_tabs();

async function search_by_name() {
    var name = document.getElementById("tab_search_name").value;

    document.getElementById("search_container").innerHTML = "";
    fetch(
        "tab_search",
        {
            method: "POST",
            body: name
        }
    ).then(resp => resp.text())
    .then(data => {
        const tabs = JSON.parse(data).res;

        document.getElementById("search_container").innerHTML = "<h4>Created tabs</h4>"
        for (i = 0; i < tabs.length; i++) {
            document.getElementById("search_container").innerHTML = document.getElementById("search_container").innerHTML + '<a class="tab_link" href="/list/' + tabs[i].id + '">' + tabs[i].title + "</a>";
        }
        fetch(
            "tab_search_pub",
            {
                method: "POST",
                body: name
            }
        ).then(resp => resp.text())
        .then(d => {
            const tabs = JSON.parse(d).res;
    
            document.getElementById("search_container").innerHTML = document.getElementById("search_container").innerHTML + "<h4>Public tabs</h4>"
            for (i = 0; i < tabs.length; i++) {
                document.getElementById("search_container").innerHTML = document.getElementById("search_container").innerHTML + '<a class="tab_link" href="/list/' + tabs[i].id + '">' + tabs[i].title + "</a>";
            }
        })
    })
}
