document.addEventListener('DOMContentLoaded', function() {
    GetFiles();
    document.getElementById('fileTable').addEventListener('click', function(e) {
        if (e.target.tagName === 'TH') {
            sortSize();
        }
    });
});

class File {
    constructor(name, size, date) {
        this.name = name;
        this.size = size;
        this.date = date;
    }

}

function GetFiles() {
    let res = fetch('http://127.0.0.1:8000/api/files/list');
    res.then((response) => {
        return response.json();
    }).then((data) => {
        for (let i = 0; i < data.length; i++) {
            console.log(data[i])
            let file = new File(data[i].name, sizeFormatter(data[i].size), data[i].modified);
            UpdateTable(file);
        }
    });
}

function UpdateTable(File) {
    let table = document.getElementById('fileTable');
    let row = table.insertRow(1);
    let cell1 = row.insertCell(0);
    let cell2 = row.insertCell(1);
    let cell3 = row.insertCell(2);
    let cell4 = row.insertCell(3);

    let httpUrl = 'http://127.0.0.1:8000/api/files/download/' + File.name.trim();
    let downloadHTML = '<a href="' + httpUrl + '"><button>Download</button></a>';

    cell1.innerHTML = File.name;
    cell2.innerHTML = File.size;
    cell3.innerHTML = File.date;
    cell4.innerHTML = downloadHTML;
}

function sizeFormatter(size) {
    let units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let unit = 0;
    while (size > 1024) {
        size /= 1024;
        unit++;
    }
    // if the size.toFixed(2) contains the decimal .00, remove it
    if (size.toFixed(2).slice(-3) === '.00') {
        return size.toFixed(0) + ' ' + units[unit];
    }
    return size.toFixed(2) + ' ' + units[unit];
}

function sortSize() {
    let table = document.getElementById('fileTable');
    //get the size column fr4om the table, and then sort it by size
    let rows = Array.from(table.rows);
    rows.shift();
    rows.sort((a, b) => {
        let aSize = a.cells[1].innerHTML.split(' ')[0];
        let bSize = b.cells[1].innerHTML.split(' ')[0];
        return aSize - bSize;
    });
    //remove the rows from the table
    while (table.rows.length > 1) {
        table.deleteRow(1);
    }
    //add the sorted rows back to the table
    for (let i = 0; i < rows.length; i++) {
        table.appendChild(rows[i]);
    }
}