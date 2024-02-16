document.addEventListener('DOMContentLoaded', function() {
    console.log('DOM loaded with JavaScript');
});

let screen_width = window.innerWidth;
let header_offset = screen_width - 544;

if (screen_width > 544) {
    //get the container class, remove the previous margin-left percentage and add the new margin-left pixel value
    document.getElementById("header").style.left = header_offset + "px";
} else {
    document.getElementById("header").style.left = "0px";
}