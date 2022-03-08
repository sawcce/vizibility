use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use std::error::Error;

pub enum Tokens {
    r#use,
}

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

type testFN = unsafe extern "C" fn() -> u64;

impl<'ctx> CodeGen<'ctx> {
    fn test_program(&self) -> Option<JitFunction<testFN>> {
        let type_of = self.context.i64_type();
        let fn_type = type_of.fn_type(&[], false);
        let function = self.module.add_function("test", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        let val = type_of.const_int(42, false);

        self.builder.position_at_end(basic_block);

        let x = self.builder.build_or(
            type_of.const_int(0, false),
            type_of.const_int(25, false),
            "dunno",
        );

        self.builder.build_return(Some(&x));

        unsafe { self.execution_engine.get_function("test").ok() }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("sum");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let test = codegen.test_program().ok_or("Unable to compile")?;

    unsafe {
        println!("{}", test.call());
    }

    Ok(())
}
