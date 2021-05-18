const { Suite } = require('benchmark');
const { readFile: rustReadFile, readFileSync: rustReadFileSync } = require('../');
const { readFile: nodeReadFile } = require('fs/promises');
const { readFileSync: nodeReadFileSync } = require('fs');
const { join } = require('path');

const filePath = join(__dirname, 'file.txt');
const expected = Buffer.from('Hello, world!\nNode + rust is cool.\n'.repeat(64));

function validate(result) {
    return new Promise((resolve, reject) => {
        expected.equals(result) ? resolve() : reject(new Error(`${result} does not match ${expected}`));
    });
}

const suite = new Suite()
    .add('Node read file', async () => {
        await validate(await nodeReadFile(filePath));
    })
    .add('Node read file sync', async () => {
        await validate(nodeReadFileSync(filePath));
    })
    .add('Rust read file sync', async () => {
        await validate(rustReadFileSync(filePath));
    })
    .add('Rust read file', async () => {
        await validate(await rustReadFile(filePath));
    })
    .on('cycle', event => {
        console.log(String(event.target));
    })
    .on('complete', () => {
        console.log(`Fastest was ${suite.filter('fastest').map('name')}`);
        process.exit();
    })
    .run({ async: true });