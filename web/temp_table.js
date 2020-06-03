function update_table_info()
{
    var xmlhttp = new XMLHttpRequest();
    var url = "get_temperature";

    xmlhttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            var temp_data = JSON.parse(this.responseText);
            for (let value of temp_data["Data"]) {
                insert_row(value["Time"], value["Temp"]);
            }
        }
    };
    xmlhttp.open("GET", url, true);
    xmlhttp.send();
}

function insert_row(time, value)
{
    var table = document.getElementById("temp_table");

    // Create an empty <tr> and add it to the end
    var row = table.insertRow(-1);

    // Insert new cells (<td> elements) at the 1st and 2nd position of the "new" <tr> element:
    var cell1 = row.insertCell(0);
    var cell2 = row.insertCell(1);

    // Add some text to the new cells:
    cell1.innerHTML = get_formatted_date(time);
    cell2.innerHTML = value;
}

function get_formatted_date(unix_timestamp)
{
    var date = new Date(unix_timestamp * 1000);
    var month = int_fill_zero_2(date.getMonth());
    var day = int_fill_zero_2(date.getDay());
    var format_date = date.getFullYear() + "-" + month + "-" + day;
    var hours = int_fill_zero_2(date.getHours());
    var minutes = int_fill_zero_2(date.getMinutes());
    var seconds = int_fill_zero_2(date.getSeconds());
    var formatted_hour = hours + ":" + minutes + ":" + seconds;

    return format_date + " " + formatted_hour;
}

function int_fill_zero_2(integer) {
    return (integer >= 10 ? "" : "0") + integer;
}

update_table_info();