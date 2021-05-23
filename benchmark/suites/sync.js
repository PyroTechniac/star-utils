const { suite, add, cycle, complete, save } = require('benny');
const { readFileSync: rustReadFileSync } = require('../../');
const { readFileSync: nodeReadFileSync } = require('fs');
const { join } = require('path');

const filePath = join(__dirname, '..', 'file.txt');

module.exports = () => suite(
    'Sync',
    add('Node read file sync', () => {
        nodeReadFileSync(filePath);
    }),
    add('Rust read file sync', () => {
        rustReadFileSync(filePath);
    }),
    cycle(),
    complete(),
    save({
        folder: 'bench-results',
        file: 'sync',
        version: require('../../package.json').version
    })
);