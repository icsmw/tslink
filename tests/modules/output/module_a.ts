export interface FieldB {
    One?: string;
    Two?: [number, number];
    Three?: FieldA;
}
export enum FieldA {
    One,
    Two,
    Three,
}
export interface StructA {
    a: FieldA;
    b: FieldB;
}
