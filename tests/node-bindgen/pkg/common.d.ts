interface Out {
    equal(value: any): Out;
    beTrue(): Out;
    type(typeName: string): Out;
    typeNot(typeName: string): Out;
    msg(msg: string): Out;
}
export declare class Group {
    protected group: string;
    constructor(group: string);
    test(name: string): Test;
}
export declare class Test {
    protected name: string;
    protected started: number;
    constructor(name: string);
    fail(msg: string): void;
    success(): void;
    assert(smth: any): Out;
}
export declare function assert(smth: any, test?: Test): Out;
export {};
