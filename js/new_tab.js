
var row = 1

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

function update_row(number) {
    var n = document.getElementById("string_select_" + number).value;

    if (n == 0)
        n = "-";
    document.getElementById("row_" + row + "_string_" + number).innerHTML = n;
    document.getElementById("string_select_" + number).value = 0;
}

function next_row() {
    document.getElementById("tab_error_msg").innerHTML = ""

    document.getElementById("prev_tabs").innerHTML = document.getElementById("prev_tabs").innerHTML +
        '<div class="static_tab_row"><p id="row_'
        + row + '_string_1" class="static_tab_bit"></p><p id="row_'
        + row + '_string_2" class="static_tab_bit"></p><p id="row_'
        + row + '_string_3" class="static_tab_bit"></p><p id="row_'
        + row + '_string_4" class="static_tab_bit"></p><p id="row_'
        + row + '_string_5" class="static_tab_bit"></p><p id="row_'
        + row + '_string_6" class="static_tab_bit"></p></div>';
    for (var i = 1; i <= 6; i++)
        if (document.getElementById("string_select_" + i).value >= 100 || document.getElementById("string_select_" + i).value < 0) {
            document.getElementById("tab_error_msg").innerHTML = "please use values between 0 and 99"
            return
        }
    for (var i = 1; i <= 6; i++)
        update_row(i);
    row += 1;
}
