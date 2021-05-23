import { rm } from 'fs/promises';

const targetFolder = new URL('../target', import.meta.url);

const options = { recursive: true, force: true };

console.time('clean');
await rm(targetFolder, options);
console.timeEnd('clean');