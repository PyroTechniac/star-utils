export declare function readFileSync(filePath: string): Buffer;
export declare function readFile(filePath: string): Promise<Buffer>;
export declare function writeFileSync(filePath: string, data: Buffer): void;
export declare function writeFile(filePath: string, data: Buffer): Promise<void>;