const path = require("path");
const ffi = require("ffi");
const ref = require("ref");

const mode = process.env.NODE_ENV === "production" ? "release" : "debug";

const voidPtr = ref.refType(ref.types.void);
const self = voidPtr;
const callback = voidPtr;

const lib = ffi.Library(path.join(__dirname, `../target/${mode}/libcore`), {
  GameBoyColor_new: [self, []],
  GameBoyColor_load: [ref.types.void, [self, ref.types.CString]],
  GameBoyColor_input: [ref.types.void, [self, ref.types.uint8]],
  GameBoyColor_render: [ref.types.void, [self, callback]],
  GameBoyColor_delete: [ref.types.void, [self]]
});

module.exports = {
  emulators: {
    gameboycolor: {
      GameBoyColor: class GameBoyColor {
        constructor() {
          this._self = lib.GameBoyColor_new();
        }

        load(filepath) {
          lib.GameBoyColor_load(this._self, filepath);
        }

        input(inputs) {
          lib.GameBoyColor_input(this._self, inputs);
        }

        render(callback) {
          lib.GameBoyColor_render(
            this._self,
            ffi.Callback(
              ref.types.void,
              [voidPtr, ref.types.size_t, ref.types.size_t],
              (pixels, width, height) => {
                callback(
                  pixels.reinterpret(width * height * ref.sizeof.uint32),
                  width,
                  height
                );
              }
            )
          );
        }

        delete() {
          lib.GameBoyColor_delete(this._self);
        }
      }
    }
  }
};
