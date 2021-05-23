const asyncSuite = require('./suites/async');
const syncSuite = require('./suites/sync');

async function main() {
    await asyncSuite();
    await syncSuite();
}

main();