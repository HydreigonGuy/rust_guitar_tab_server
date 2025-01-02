
var row = 1;
var max_row = 1;

var deleted = [];
var editing = [];

async function create_tab() {
    var title = document.getElementById("tab_name").value;
    var tabs = [[], [], [], [], [], []];
    const visibility = parseInt(document.getElementById("visibillity_selecter").value);

    document.getElementById("error_msg").innerHTML = "";
    if (title == "") {
        document.getElementById("error_msg").innerHTML = "Please fill in the name of your tab";
        return;
    }
    if (editing.length != 0) { // put all editing tabs back to complete
        for (var i = 1; i <= max_row - 1; i++) {
            if (editing.indexOf(i) != -1) {
                edit_row(i);
            }
            if (editing.indexOf(i) != -1) {
                return;
            }
        }
    }
    for (var i = 1; i <= max_row - 1; i++) {
        if (deleted.indexOf(i) == -1) {
            for (var j = 1; j <= 6; j++) {
                var n = document.getElementById("row_" + i + "_string_" + j).innerHTML;

                if (n == "-")
                    tabs[j - 1].push(100);
                else if (n == "X")
                    tabs[j - 1].push(101);
                else if (n == "~")
                    tabs[j - 1].push(102);
                else if (n == "/")
                    tabs[j - 1].push(103);
                else
                    tabs[j - 1].push(parseInt(n));
            }
        }
    }
    const resp = await fetch(
        "new_tab",
        {
            method: "POST",
            body: JSON.stringify({ title: title, tab: tabs, visibility: visibility }),
        }
    );
    console.log(resp);
}

function check_if_value_is_valid(value) {
    const valid_inputs = ["", "X", "~", "/"]
    if (valid_inputs.includes(value))
        return true;
    var c = parseInt(value);
    if (isNaN(c) || c < 0 || c > 99)
        return false;
    return true;
}

function update_row(number) {
    var n = document.getElementById("string_select_" + number).value;

    if (n == "")
        n = "-";
    document.getElementById("row_" + row + "_string_" + number).innerHTML = n;
    document.getElementById("string_select_" + number).value = "";
}

function next_row() {
    document.getElementById("tab_error_msg").innerHTML = ""

    for (var i = 1; i <= 6; i++)
        if (!check_if_value_is_valid(document.getElementById("string_select_" + i).value)) {
            document.getElementById("tab_error_msg").innerHTML = "please use values between 0 and 99, X, ~, /, or leave it empty"
            return
        }
    document.getElementById("prev_tabs").innerHTML = document.getElementById("prev_tabs").innerHTML +
        '<div class="static_tab_row" id="row_'
        + row + '_container"><button class="edit_column_button" onclick="edit_row('
        + row + ')">e</button><button class="delete_column_button" onclick="delete_row('
        + row + ')">-</button><p id="row_'
        + row + '_string_1" class="static_tab_bit"></p><p id="row_'
        + row + '_string_2" class="static_tab_bit"></p><p id="row_'
        + row + '_string_3" class="static_tab_bit"></p><p id="row_'
        + row + '_string_4" class="static_tab_bit"></p><p id="row_'
        + row + '_string_5" class="static_tab_bit"></p><p id="row_'
        + row + '_string_6" class="static_tab_bit"></p></div>';
    for (var i = 1; i <= 6; i++)
        update_row(i);
    row += 1;
    if (row > max_row)
        max_row = row;
}

function delete_row(row) {
    document.getElementById("row_" + row + "_container").innerHTML = "";
    deleted.push(row);
}

function edit_row(row) {
    document.getElementById("tab_error_msg").innerHTML = ""
    if (deleted.indexOf(row) != -1) {
        return;
    }
    if (editing.indexOf(row) != -1) {
        for (var i = 1; i <= 6; i++)
            if (document.getElementById("row_" + row + "_string_" + i + "_input").value >= 100 || document.getElementById("row_" + row + "_string_" + i + "_input").value < 0) {
                document.getElementById("tab_error_msg").innerHTML = "please use values between 0 and 99"
                return
            }
        for (var i = 1; i <= 6; i++) {
            var n = document.getElementById("row_" + row + "_string_" + i + "_input").value;
            if (n == 0)
                n = "-";
            document.getElementById("row_" + row + "_string_" + i).innerHTML = n;
        }
        editing.splice(editing.indexOf(row), 1);
    } else {
        for (var i = 1; i <= 6; i++) {
            var n = document.getElementById("row_" + row + "_string_" + i).innerHTML;
            if (n == "-")
                n = 0;
            document.getElementById("row_" + row + "_string_" + i).innerHTML = '<input id="row_' + row + '_string_' + i + '_input" type="number" value="' + n + '" min="0" max="24"/>';
        }
        editing.push(row);
    }
}
