/* tslint:disable */
/* eslint-disable */
/**
* [`encode`] converts bytes into a base64-encoded byte array.
* @param {Uint8Array} data
* @returns {Uint8Array}
*/
export function encode(data: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} data
* @returns {string}
*/
export function encode_to_utf8(data: Uint8Array): string;
/**
* [`decode`] takes ascii and returns its original binary representation.
* @param {Uint8Array} ascii
* @returns {Uint8Array}
*/
export function decode(ascii: Uint8Array): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly encode: (a: number, b: number, c: number) => void;
  readonly encode_to_utf8: (a: number, b: number, c: number) => void;
  readonly decode: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
