/* tslint:disable */
/* eslint-disable */
/**
 * The `ReadableStreamType` enum.
 *
 * *This API requires the following crate features to be activated: `ReadableStreamType`*
 */

export type ReadableStreamType = "bytes";

export function Background_7011291641563722467(el: HTMLElement): void;

export function CommentToggle_7761011648576428996(el: HTMLElement): void;

export function HomeAnimation_14651908795469971363(el: HTMLElement): void;

export class IntoUnderlyingByteSource {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cancel(): void;
    pull(controller: ReadableByteStreamController): Promise<any>;
    start(controller: ReadableByteStreamController): void;
    readonly autoAllocateChunkSize: number;
    readonly type: ReadableStreamType;
}

export class IntoUnderlyingSink {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    abort(reason: any): Promise<any>;
    close(): Promise<any>;
    write(chunk: any): Promise<any>;
}

export class IntoUnderlyingSource {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cancel(): void;
    pull(controller: ReadableStreamDefaultController): Promise<any>;
}

export function MobileHeader_4700568660252565466(el: HTMLElement): void;

export function OAuth_12876817204350332409(el: HTMLElement): void;

export function PostDisplay_9641767229574460062(el: HTMLElement): void;

export function hydrate(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly hydrate: () => void;
    readonly PostDisplay_9641767229574460062: (a: any) => void;
    readonly Background_7011291641563722467: (a: any) => void;
    readonly CommentToggle_7761011648576428996: (a: any) => void;
    readonly MobileHeader_4700568660252565466: (a: any) => void;
    readonly OAuth_12876817204350332409: (a: any) => void;
    readonly HomeAnimation_14651908795469971363: (a: any) => void;
    readonly __wbg_intounderlyingsource_free: (a: number, b: number) => void;
    readonly intounderlyingsource_cancel: (a: number) => void;
    readonly intounderlyingsource_pull: (a: number, b: any) => any;
    readonly __wbg_intounderlyingbytesource_free: (a: number, b: number) => void;
    readonly __wbg_intounderlyingsink_free: (a: number, b: number) => void;
    readonly intounderlyingbytesource_autoAllocateChunkSize: (a: number) => number;
    readonly intounderlyingbytesource_cancel: (a: number) => void;
    readonly intounderlyingbytesource_pull: (a: number, b: any) => any;
    readonly intounderlyingbytesource_start: (a: number, b: any) => void;
    readonly intounderlyingbytesource_type: (a: number) => number;
    readonly intounderlyingsink_abort: (a: number, b: any) => any;
    readonly intounderlyingsink_close: (a: number) => any;
    readonly intounderlyingsink_write: (a: number, b: any) => any;
    readonly wasm_bindgen_5a64860dddf9f74___convert__closures_____invoke___wasm_bindgen_5a64860dddf9f74___JsValue__core_f0fd674eaa06beef___result__Result_____wasm_bindgen_5a64860dddf9f74___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_5a64860dddf9f74___convert__closures_____invoke___js_sys_d0cefdcfedb2a55c___Array__web_sys_73c0e388fcb07093___features__gen_ResizeObserver__ResizeObserver______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_5a64860dddf9f74___convert__closures_____invoke___js_sys_d0cefdcfedb2a55c___Function_fn_wasm_bindgen_5a64860dddf9f74___JsValue_____wasm_bindgen_5a64860dddf9f74___sys__Undefined___js_sys_d0cefdcfedb2a55c___Function_fn_wasm_bindgen_5a64860dddf9f74___JsValue_____wasm_bindgen_5a64860dddf9f74___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_5a64860dddf9f74___convert__closures_____invoke___wasm_bindgen_5a64860dddf9f74___JsValue______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5a64860dddf9f74___convert__closures_____invoke___wasm_bindgen_5a64860dddf9f74___JsValue______true__3: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5a64860dddf9f74___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly wasm_bindgen_5a64860dddf9f74___convert__closures_____invoke_______true__1_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
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
