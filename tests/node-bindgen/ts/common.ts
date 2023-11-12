interface Out {
    equal(value: any): Out;
    beTrue(): Out;
    type(typeName: string): Out;
    typeNot(typeName: string): Out;
    msg(msg: string): Out;
}

export class Group {
    constructor(protected group: string) {
        console.log(`Starting tests for: ${group}`);
    }

    public test(name: string): Test {
        return new Test(name);
    }
}
export class Test {
    protected started: number = Date.now();

    constructor(protected name: string) {}

    public fail(msg: string) {
        fail(`[FAIL] ${this.name}: ${msg}`);
    }

    public success() {
        console.log(`[OK in ${Date.now() - this.started}ms] ${this.name}`);
    }

    public assert(smth: any): Out {
        return assert(smth, this);
    }
}

function fail(msg: string) {
    console.error(msg);
    try {
        throw new Error("Stack");
    } catch (e) {
        console.error((e as Error).stack);
    }
    process.exit(1);
}

export function assert(smth: any, test?: Test): Out {
    let errorMessage: string | undefined;
    const failCB: (msg: string) => void =
        test !== undefined ? test.fail.bind(test) : fail;
    const out: Out = {
        equal: (value: any): Out => {
            if (value !== smth) {
                if (errorMessage === undefined) {
                    failCB(
                        `No error message. Value: ${smth} (type: ${typeof smth}) not equal to: ${value} (type: ${typeof value})`
                    );
                } else {
                    failCB(
                        `${errorMessage}. Value: ${smth} (type: ${typeof smth}) not equal to: ${value} (type: ${typeof value})`
                    );
                }
            }
            return out;
        },
        beTrue: (): Out => {
            if (smth !== true) {
                if (errorMessage === undefined) {
                    failCB(`No error message. Condition isn't true`);
                } else {
                    failCB(`${errorMessage}. Condition isn't true`);
                }
            }
            return out;
        },
        type: (typeName: string): Out => {
            if (typeof smth !== typeName) {
                if (errorMessage === undefined) {
                    failCB(
                        `No error message. Value: ${smth} (type: ${typeof smth}) has different type to: ${typeName})`
                    );
                } else {
                    failCB(
                        `${errorMessage}. Value: ${smth} (type: ${typeof smth}) has different type to: ${typeName})`
                    );
                }
            }
            return out;
        },
        typeNot: (typeName: string): Out => {
            if (typeof smth === typeName) {
                if (errorMessage === undefined) {
                    failCB(
                        `No error message. Value: ${smth} (type: ${typeof smth}) has prohibited type: ${typeName})`
                    );
                } else {
                    failCB(
                        `${errorMessage}. Value: ${smth} (type: ${typeof smth}) has prohibited type: ${typeName})`
                    );
                }
            }
            return out;
        },
        msg: (msg: string): Out => {
            errorMessage = msg;
            return out;
        },
    };
    return out;
}
