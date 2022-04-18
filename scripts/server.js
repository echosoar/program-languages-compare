const static = require('node-static');
const path = require('path');
const file = new static.Server(path.join(__dirname, '../'));

require('http').createServer((request, response) => {
    request.addListener('end',  () => {
        file.serve(request, response);
    }).resume();
}).listen(8080);