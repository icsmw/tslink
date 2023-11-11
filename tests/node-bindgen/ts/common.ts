interface Out {
    equal(value: any): Out;
    beTrue(): Out;
    type(typeName: string): Out;
    typeNot(typeName: string): Out;
    msg(msg: string): Out;
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

export function assert(smth: any): Out {
    let errorMessage: string | undefined;
    const out: Out = {
        equal: (value: any): Out => {
            if (value !== smth) {
                if (errorMessage === undefined) {
                    fail(
                        `No error message. Value: ${smth} (type: ${typeof smth}) not equal to: ${value} (type: ${typeof value})`
                    );
                } else {
                    fail(
                        `${errorMessage}. Value: ${smth} (type: ${typeof smth}) not equal to: ${value} (type: ${typeof value})`
                    );
                }
            }
            return out;
        },
        beTrue: (): Out => {
            if (smth !== true) {
                if (errorMessage === undefined) {
                    fail(`No error message. Condition isn't true`);
                } else {
                    fail(`${errorMessage}. Condition isn't true`);
                }
            }
            return out;
        },
        type: (typeName: string): Out => {
            if (typeof smth !== typeName) {
                if (errorMessage === undefined) {
                    fail(
                        `No error message. Value: ${smth} (type: ${typeof smth}) has different type to: ${typeName})`
                    );
                } else {
                    fail(
                        `${errorMessage}. Value: ${smth} (type: ${typeof smth}) has different type to: ${typeName})`
                    );
                }
            }
            return out;
        },
        typeNot: (typeName: string): Out => {
            if (typeof smth === typeName) {
                if (errorMessage === undefined) {
                    fail(
                        `No error message. Value: ${smth} (type: ${typeof smth}) has prohibited type: ${typeName})`
                    );
                } else {
                    fail(
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
