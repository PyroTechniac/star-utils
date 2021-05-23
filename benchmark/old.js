const { Suite } = require('benchmark');
const { readFile: rustReadFile, readFileSync: rustReadFileSync } = require('../');
const { readFile: nodeReadFile } = require('fs/promises');
const { readFileSync: nodeReadFileSync } = require('fs');
const { join } = require('path');
const { once, EventEmitter } = require('events');

const filePath = join(__dirname, 'file.txt');
const expected = Buffer.from('Hello, world!\nNode + rust is cool.\n'.repeat(64));

function validate(result) {
    if (!expected.equals(result)) throw new Error(`${result} does not match ${expected}`);
}

const emitter = new EventEmitter();

function syncSuite() {
    const suite = new Suite('syncSuite');
    suite
        .add('Node read file sync', () => {
            validate(nodeReadFileSync(filePath));
        })
        .add('Rust read file sync', () => {
            validate(rustReadFileSync(filePath));
        })
        .on('cycle', event => {
            console.log(String(event.target));
        })
        .on('complete', () => {
            console.log(`Fastest was ${suite.filter('fastest').map('name')}`);
            emitter.emit('syncComplete');
        })
        .run();
}

function asyncSuite() {
    const suite = new Suite('asyncSuite');
    suite
        .add('Node read file', async () => {
            validate(await nodeReadFile(filePath))
        })
        .add('Rust read file', async () => {
            validate(await rustReadFile(filePath))
        })
        .on('cycle', event => {
            console.log(String(event.target));
        })
        .on('complete', () => {
            console.log(`Fastest was ${suite.filter('fastest').map('name')}`);
            emitter.emit('asyncComplete');
        })
        .run({ async: true });
}

async function bench() {
    const promises = Promise.all([once(emitter, 'syncComplete'), once(emitter, 'asyncComplete')]);
    syncSuite();
    asyncSuite();
    await promises;
    process.exit();
}

bench();