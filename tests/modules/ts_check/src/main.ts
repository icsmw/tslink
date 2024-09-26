import {
    FieldA,
    EntityA,
    EntityB,
    FieldB,
    StructA,
    OtherStruct,
} from "../../output/index";

export function getOtherStruct(): OtherStruct {
    return {
        a: EntityA.One,
        b: { One: "Test" },
        c: { a: FieldA.One, b: { Two: [1, 2] } },
        d: FieldA.Three,
        e: { Two: [1, 2] },
    };
}

export function getStructA(): StructA {
    return { a: FieldA.One, b: { One: "Test" } };
}

export function getFieldB(): FieldB {
    return { One: "Test" };
}

export function getEntityB(): EntityB {
    return { One: "Test" };
}
