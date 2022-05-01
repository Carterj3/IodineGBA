
function objectKeys(object) {
    return object == null ? [] : Object.keys(object);
}

function isLeafEqual(lhs, rhs) {
    if (lhs == null || rhs == null) {
        return lhs == rhs;
    }

    const lhsType = typeof lhs;
    const rhsType = typeof rhs;

    if (lhsType != rhsType) {
        return false;
    }

    if (lhsType == "object") {
        if (Array.isArray(lhs)) {
            if (lhs.length != rhs.length) {
                return false;
            }

            for (let i = 0; i < rhs.length; i++) {
                if (lhs[i] != rhs[i]) {
                    return false;
                }
            }

            return true;
        } else if (ArrayBuffer.isView(lhs)) {
            if (lhs.length != rhs.length) {
                return false;
            }

            for (let i = 0; i < rhs.length; i++) {
                if (lhs[i] != rhs[i]) {
                    return false;
                }
            }

            return true;
        }

        throw new Error(`Non-Array Leaf type: '${lhsType}' - object: ${lhs}`);
    }

    return lhs == rhs;
}


function findSnapshotDifferences(snapshots) {
    let changedKeys = [];

    for (let key of objectKeys(snapshots[0])) {
        const lhs = snapshots[0][key];
        for (let i = 1; i < snapshots.length; i++) {
            if (!isLeafEqual(lhs, snapshots[i][key])) {
                changedKeys.push(key);
                break;
            }
        }
    }

    return [
        changedKeys,
        objectKeys(snapshots[0]).filter(key => !changedKeys.includes(key)),
    ];
}

function getSaveStateStats() {
    function getTypeAndSize(value, key) {
        const valueType = typeof value;
        switch (valueType) {
            case "object":
                if (value === null) {
                    throw new Error(`Null found at Key: ${key}`);
                } else if (Array.isArray(value)) {
                    let [subType, subSize] = getTypeAndSize(value[0], key);
                    return [`Array<${subType}>`, value.length * subSize];
                } else if (ArrayBuffer.isView(value)) {
                    if (value instanceof Uint8Array) {
                        return [`Uint8Array`, value.length];
                    } else if (value instanceof Uint16Array) {
                        return [`Uint16Array`, value.length];
                    } else if (value instanceof Uint32Array) {
                        return [`Uint32Array`, value.length];
                    } else if (value instanceof Int8Array) {
                        return [`Int8Array`, value.length];
                    } else if (value instanceof Int16Array) {
                        return [`Int16Array`, value.length];
                    } else if (value instanceof Int32Array) {
                        return [`Int32Array`, value.length];
                    }
                    throw new Error(`Unhandled ArrayBuffer at Key: ${key}`);
                } else {
                    const objKeys = objectKeys(value);
                    let [subType, subSize] = getTypeAndSize(value[objKeys[0]], key);
                    return [`Object<${subType}>`, objKeys.length * subSize];
                }
                break;
            case "boolean":
                return ["boolean", 1];
            case "number":
                return ["number", 1];
            case "string":
                return ["string", value.length];
            default:
                throw new Error(`Unhandled type: '${valueType}' - key: '${key}'`);
        }
    }

    const state = fastSave();
    const sizes = {}
    for (let key of objectKeys(state)) {
        sizes[key] = getTypeAndSize(state[key], key);
    }

    return sizes;
}