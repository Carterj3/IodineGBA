
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