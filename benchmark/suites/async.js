const { suite, add, cycle, complete, save } = require('benny');
const { readFile: rustReadFile } = require('../../');
const { readFile: nodeReadFile } = require('fs/promises');
const { join } = require('path');

const filePath = join(__dirname, '..', 'file.txt');

module.exports = () => suite(
    'Async',
    add('Node read file', async () => {
        return nodeReadFile(filePath);
    }),
    add('Rust read file', async () => {
        return rustReadFile(filePath);
    }),
    cycle(),
    complete(),
    save({
        folder: 'bench-results',
        file: 'async',
        version: require('../../package.json').version
    })
);