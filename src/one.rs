use wasmtime::*;

struct TimesTwo;

impl wasmtime::Callable for TimesTwo {
    fn call(&self, params: &[Val], results: &mut [Val]) -> Result<(), wasmtime::Trap> {
        let mut value = params[0].unwrap_i32();
        value *= 2;
        results[0] = value.into();

        Ok(())
    }
}

pub fn init(store: &Store) -> HostRef<wasmtime::Func> {
    let times_two_type = wasmtime::FuncType::new(
        // Parameters
        Box::new([wasmtime::ValType::I32]),
        // Results
        Box::new([wasmtime::ValType::I32]),
    );

    let xx = HostRef::new(wasmtime::Func::new(
        &store,
        times_two_type,
        std::rc::Rc::new(TimesTwo),
    ));

    xx
}
