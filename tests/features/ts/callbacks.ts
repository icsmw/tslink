import {
    callbackGenericA,
    callbackGenericB,
    callbackGenericC,
    callbackGenericD,
    callbackGenericE,
    callbackGenericF,
    CallbacksStruct,
} from "binding";

callbackGenericA((a: number, b: number, c: boolean) => {});

callbackGenericB((a: number, b: number, c: boolean): number => {
    return 0;
});

callbackGenericC((a: number, b: number, c: boolean) => {
    return 0;
});

callbackGenericD((a: number, b: number, c: boolean) => {});

callbackGenericE((a: number, b: number, c: boolean) => {
    return "";
});

callbackGenericF((a: number, b: number, c: boolean) => {
    return "";
});

const struct = new CallbacksStruct();

struct.callbackGenericA((a: number, b: number, c: boolean) => {});

struct.callbackGenericB((a: number, b: number, c: boolean): number => {
    return 0;
});

struct.callbackGenericC((a: number, b: number, c: boolean) => {
    return 0;
});

struct.callbackGenericD((a: number, b: number, c: boolean) => {});

struct.callbackGenericE((a: number, b: number, c: boolean) => {
    return "";
});

struct.callbackGenericF((a: number, b: number, c: boolean) => {
    return "";
});

struct.callbackGenericAMut((a: number, b: number, c: boolean) => {});

struct.callbackGenericBMut((a: number, b: number, c: boolean): number => {
    return 0;
});

struct.callbackGenericCMut((a: number, b: number, c: boolean) => {
    return 0;
});

struct.callbackGenericDMut((a: number, b: number, c: boolean) => {});

struct.callbackGenericEMut((a: number, b: number, c: boolean) => {
    return "";
});

struct.callbackGenericFMut((a: number, b: number, c: boolean) => {
    return "";
});
