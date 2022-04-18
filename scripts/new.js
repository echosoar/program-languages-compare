const table = require('./table');
const fs = require('fs');
const path = require('path');

const data = [];
for(const classify of table) {
    if (!classify.child) {
        continue;
    }
    for(const main of classify.child) {
        if (!main.child) {
            continue;
        }
        for(const sub of main.child) {
            data.push(`// - ${main.class}-${sub.class}`)
        }
    }
}

fs.writeFileSync(path.join(__dirname, '../data/data.txt'), data.join('\n\n'));