'use strict';

/* https://gist.github.com/alemures/ec82c1995f5fe83df0e0dd228af58005  */

var INITIAL_CAPACITY = 16;

/**
 * @param {Array|Number} arg Content or initial capacity.
 */
function ArrayDeque(arg) {
    this._elements = new Array(typeof arg === 'number' ?
        this._nextHighestPowerOfTwo(arg) : Array.isArray(arg) ?
            this._nextHighestPowerOfTwo(arg.length) : INITIAL_CAPACITY);
    this._head = 0;
    this._tail = 0;

    this._modulusMask = this._elements.length - 1;

    if (Array.isArray(arg)) {
        this.pushAll(arg);
    }
}

ArrayDeque.prototype.push = function (element) {
    this._elements[this._tail] = element;
    this._tail = (this._tail + 1) & this._modulusMask;
    if (this._tail === this._head) {
        this._grow();
    }
};

ArrayDeque.prototype.pop = function () {
    if (this._tail === this._head) {
        return undefined;
    }

    var nextTail = (this._tail - 1) & this._modulusMask;
    var result = this._elements[nextTail];
    this._elements[nextTail] = undefined;
    this._tail = nextTail;
    return result;
};

ArrayDeque.prototype.unshift = function (element) {
    this._head = (this._head - 1) & this._modulusMask;
    this._elements[this._head] = element;
    if (this._head === this._tail) {
        this._grow();
    }
};

ArrayDeque.prototype.shift = function () {
    if (this._tail === this._head) {
        return undefined;
    }

    var result = this._elements[this._head];
    this._elements[this._head] = undefined;
    this._head = (this._head + 1) & this._modulusMask;
    return result;
};

ArrayDeque.prototype.first = function () {
    return this._elements[this._head];
};

ArrayDeque.prototype.last = function () {
    return this._elements[(this._tail - 1) & this._modulusMask];
};

ArrayDeque.prototype.get = function (index) {
    return index >= 0 && index < this.size() ?
        this._elements[(index + this._head) & this._modulusMask] : undefined;
};

ArrayDeque.prototype.pushAll = function (array) {
    for (var i = 0; i < array.length; i++) {
        this.push(array[i]);
    }
};

ArrayDeque.prototype.clear = function () {
    this._elements = new Array(INITIAL_CAPACITY);
    this._head = 0;
    this._tail = 0;
};

ArrayDeque.prototype.size = function () {
    if (this._head === this._tail) {
        return 0;
    } else if (this._head < this._tail) {
        return this._tail - this._head;
    } else {
        return (this._modulusMask + 1) - (this._head - this._tail);
    }
};

ArrayDeque.prototype.isEmpty = function () {
    return this.size() === 0;
};

ArrayDeque.prototype.toArray = function () {
    var array = new Array(this.size());
    for (var i = 0; i < array.length; i++) {
        array[i] = this._elements[(i + this._head) & this._modulusMask];
    }

    return array;
};

ArrayDeque.prototype.toString = function () {
    return this.toArray().toString();
};

ArrayDeque.prototype._grow = function () {
    var length = this._elements.length;
    this._elements.length <<= 1;
    this._modulusMask = this._elements.length - 1;

    this._move(this._elements, 0, length, this._head);
    this._tail = length + this._head;
};

ArrayDeque.prototype._move = function (array, srcPos, destPos, length) {
    for (var i = 0; i < length; ++i) {
        array[destPos + i] = array[srcPos + i];
        array[srcPos + i] = undefined;
    }
};

ArrayDeque.prototype._nextHighestPowerOfTwo = function (value) {
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    return value + 1;
};