#![allow(non_snake_case)]

use project;

#[test]
fn add() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn add(this: &js::Set, value: &JsValue) -> js::Set {
                this.add(value)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new Set([]);

                wasm.add(set, 100);


                assert.equal(set.size, 1);
                assert.equal(Array.from(set)[0], 100);
            }
        "#)
        .test()
}

#[test]
fn clear() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn clear(this: &js::Set) {
                this.clear();
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new Set([1, 2, 3]);

                wasm.clear(set);

                assert.equal(set.size, 0);
            }
        "#)
        .test()
}