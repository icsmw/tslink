interface Out {
    equal(value: any): Out;
    beTrue(): Out;
    type(typeName: string): Out;
    typeNot(typeName: string): Out;
    msg(msg: string): Out;
}
export declare function assert(smth: any): Out;
export {};
