const path = require("path");
const ffi = require("ffi");
const ref = require("ref");

const mode = process.env.NODE_ENV === "production" ? "release" : "debug";

const voidPtr = ref.refType(ref.types.void);
const self = voidPtr;
const callback = voidPtr;

const libcore = ffi.Library(path.join(__dirname, `../target/${mode}/libcore`), {
  new: [self, []],
  input: [ref.types.void, [self]],
  update: [ref.types.void, [self]],
  render: [ref.types.void, [self, callback]],
  delete: [ref.types.void, [self]]
});

module.exports = class Core {
  constructor() {
    this._self = libcore.new();
  }

  input() {
    libcore.input(this._self);
  }

  update() {
    libcore.update(this._self);
  }

  render(callback) {
    libcore.render(this._self, ffi.Callback(ref.types.void, [], callback));
  }

  delete() {
    libcore.delete(this._self);
  }
};
