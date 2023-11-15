"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const binding_1 = require("binding");
(0, binding_1.callbackGenericA)((a, b, c) => { });
(0, binding_1.callbackGenericB)((a, b, c) => {
    return 0;
});
(0, binding_1.callbackGenericC)((a, b, c) => {
    return 0;
});
(0, binding_1.callbackGenericD)((a, b, c) => { });
(0, binding_1.callbackGenericE)((a, b, c) => {
    return "";
});
(0, binding_1.callbackGenericF)((a, b, c) => {
    return "";
});
const struct = new binding_1.CallbacksStruct();
struct.callbackGenericA((a, b, c) => { });
struct.callbackGenericB((a, b, c) => {
    return 0;
});
struct.callbackGenericC((a, b, c) => {
    return 0;
});
struct.callbackGenericD((a, b, c) => { });
struct.callbackGenericE((a, b, c) => {
    return "";
});
struct.callbackGenericF((a, b, c) => {
    return "";
});
struct.callbackGenericAMut((a, b, c) => { });
struct.callbackGenericBMut((a, b, c) => {
    return 0;
});
struct.callbackGenericCMut((a, b, c) => {
    return 0;
});
struct.callbackGenericDMut((a, b, c) => { });
struct.callbackGenericEMut((a, b, c) => {
    return "";
});
struct.callbackGenericFMut((a, b, c) => {
    return "";
});
//# sourceMappingURL=callbacks.js.map