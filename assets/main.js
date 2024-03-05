
document.addEventListener('htmx:beforeSwap', function (event) {
    if (event.detail.xhr.status === 200) {
        if (event.detail.pathInfo.finalRequestPath.includes('/md')) {
            event.detail.serverResponse = marked.parse(event.detail.serverResponse);
        }
    }
});
