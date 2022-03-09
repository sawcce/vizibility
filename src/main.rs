use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use std::error::Error;
use std::path::Path;

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

        let i64 = self.context.i64_type();

        let then_block = self.context.append_basic_block(function, "block1");

        self.builder.position_at_end(then_block);
        self.builder.build_return(Some(&i64.const_int(0, false)));

        
        let else_block = self.context.append_basic_block(function, "block2");
        self.builder.position_before(&then_block.get_first_instruction().unwrap());
        //self.builder.position_at(else_block, &then_block.get_last_instruction().unwrap());
        self.builder.build_return(Some(&i64.const_int(1, false)));

        println!("{:?}", function.get_first_basic_block().unwrap());
        println!("{:?}", function.get_last_basic_block().unwrap());
        /*

        self.builder
            .build_return(Some(&self.context.i64_type().const_int(0, false)));

        let then_block = self.context.insert_basic_block_after(else_block, "then");
        self.builder.position_at_end(then_block);
        self.builder
            .build_return(Some(&self.context.i64_type().const_int(46, false)));
        then_block.move_after(else_block);

        else_block
        .get_context()
        .create_builder()
        .build_return(Some(&i64.const_int(1, false))); */
        //let value_else = self.context.i64_type().const_int(45, false);

        /*

                let instr = function
                    .get_first_basic_block()
                    .unwrap()
                    .get_first_instruction()
                    .unwrap();
        */
        /*
        let zero = i64.const_int(0, false);

        let entry = self.context.append_basic_block(function, "entry");
        self.builder
            .build_conditional_branch(zero, then_block, else_block);
        self.builder.build_return(Some(&zero));
        self.builder.position_at_end(entry); */

        unsafe { self.execution_engine.get_function("test").ok() }

        /*         let val = type_of.const_int(42, false);

        let x = self.builder.build_or(
            type_of.const_int(0, false),
            type_of.const_int(25, false),
            "dunno",
        ); */

        /*
        let fn_ = self.module.get_function("printf");
        println!("{:?}", fn_); */

        /*        let printfFunc = self.module.add_function(
        "printf",
        Linkage::ExternalLinkage,
        "printf"); */
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

    codegen.module.print_to_file("out.ir");
    codegen.module.write_bitcode_to_path(Path::new("out.exe"));

    unsafe {
        println!("{}", test.call());
    }

    Ok(())
}
