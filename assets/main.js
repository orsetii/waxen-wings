
document.addEventListener('htmx:beforeSwap', function(event) {
  if (event.detail.xhr.status === 200) {
    if (event.detail.pathInfo.finalRequestPath.includes('/md')) {
      const startIndex = event.detail.serverResponse.indexOf('ARTICLE_START');
      const endIndex = event.detail.serverResponse.indexOf('ARTICLE_END') + 'ARTICLE_END'.length;

      const articleContent = event.detail.serverResponse.slice(startIndex + 'ARTICLE_START'.length, endIndex).trim();
      const articleHTML = marked.parse(articleContent);

      event.detail.serverResponse = event.detail.serverResponse.slice(0, startIndex) + articleHTML + event.detail.serverResponse.slice(endIndex);
    }
  }
});

