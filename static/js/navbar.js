//wait for the DOM to be loaded
document.addEventListener('DOMContentLoaded', function() {
    document.getElementById('home').addEventListener('click', function() {
        // redirect to the home page
        window.location.href = '/';
    });
    document.getElementById('repeaters').addEventListener('click', function() {
        // redirect to the repeaters page
        window.location.href = '/repeaters';
    });
    document.getElementById('events').addEventListener('click', function() {
        // redirect to the events page
        window.location.href = '/events';
    });
    document.getElementById('resources').addEventListener('click', function() {
        // redirect to the resources page
        window.location.href = '/resources';
    });
});