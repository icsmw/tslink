export enum FieldA {
    One,
    Two,
    Three,
}
export interface FieldB {
    One?: string;
    Two?: [number, number];
    Three?: FieldA;
}
export interface StructA {
    a: FieldA;
    b: FieldB;
}
