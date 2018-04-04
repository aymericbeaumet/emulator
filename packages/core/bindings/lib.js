const path = require("path");
const ffi = require("ffi");
const ref = require("ref");

const mode = process.env.NODE_ENV === "production" ? "release" : "debug";

const voidPtr = ref.refType(ref.types.void);
const self = voidPtr;
const callback = voidPtr;

const lib = ffi.Library(path.join(__dirname, `../target/${mode}/libcore`), {
  game_engine_engine_new: [self, []],
  game_engine_engine_input: [ref.types.void, [self, ref.types.uint8]],
  game_engine_engine_update: [ref.types.void, [self]],
  game_engine_engine_render: [ref.types.void, [self, callback]],
  game_engine_engine_delete: [ref.types.void, [self]]
});

module.exports = {
  game: {
    engine: {
      Engine: class Engine {
        constructor() {
          this._self = lib.game_engine_engine_new();
        }

        input(keys) {
          lib.game_engine_engine_input(this._self, keys);
        }

        update() {
          lib.game_engine_engine_update(this._self);
        }

        render(callback) {
          lib.game_engine_engine_render(
            this._self,
            ffi.Callback(
              ref.types.void,
              [voidPtr, ref.types.size_t, ref.types.size_t],
              (pixels, width, height) => {
                callback(pixels.reinterpret(width * height), width, height);
              }
            )
          );
        }

        delete() {
          lib.game_engine_engine_delete(this._self);
        }
      }
    }
  }
};
