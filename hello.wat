(;; wat2wasm hello.wat -o $WASM_FILES/hello.wasm ;;)
(module
  (func $times_two (import "" "times_two") (param i32) (result i32))
  (func $times_three (import "" "times_three") (param i32) (result i32))
  (func (export "answer") (param $p1 i32) (result i32)
     local.get $p1
     (call $times_three)
  )
)