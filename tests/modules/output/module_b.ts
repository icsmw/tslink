export enum EntityA {
    One,
    Two,
    Three,
}
export interface EntityB {
    One?: string;
    Two?: [number, number];
    Three?: EntityA;
}
import { FieldB } from "./module_a";
import { StructA } from "./module_a";
import { FieldA } from "./module_a";
export interface OtherStruct {
    a: EntityA;
    b: EntityB;
    c: StructA;
    d: FieldA;
    e: FieldB;
}
