use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::{Linkage, Module};
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::types::BasicType;
use inkwell::values::{AnyValue, BasicMetadataValueEnum, BasicValue, FunctionValue};
use inkwell::{AddressSpace, OptimizationLevel};
use std::error::Error;
use std::path::Path;

pub enum Tokens {
    r#Use,
    Identifier(String),
}

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

type GreaterThan = unsafe extern "C" fn(i64, i64) -> bool;

use inkwell::IntPredicate;

impl<'ctx> CodeGen<'ctx> {
    fn define_print(&self) -> FunctionValue<'ctx> {
        let string_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let print_type = self.context.i32_type();

        return self.module.add_function(
            "puts",
            print_type.fn_type(&[string_type.into()], false),
            Some(Linkage::External),
        );
    }

    fn get_print_fn(&self) -> FunctionValue {
        self.module
            .get_function("puts")
            .ok_or("Must define print before getting print function!")
            .unwrap()
    }

    fn print(&self, text: &str) {
        let print = self
            .module
            .get_function("puts")
            .ok_or("Must define print before printing to console!")
            .unwrap();

        let message = self.builder.build_global_string_ptr(text, "msg");

        self.builder
            .build_call(print, &[message.as_pointer_value().into()], "string");
    }

    fn test_program(&self) -> Option<JitFunction<GreaterThan>> {
        self.define_print();

        let i64 = self.context.i64_type();
        let bool = self.context.bool_type();

        let fn_type = bool.fn_type(&[i64.into(), i64.into()], false);
        let function = self.module.add_function("main", fn_type, None);

        let num_a = function.get_nth_param(0)?.into_int_value();
        let num_b = function.get_nth_param(1)?.into_int_value();

        let entry = self.context.append_basic_block(function, "entry");
        let then_block = self.context.append_basic_block(function, "then");
        let else_block = self.context.append_basic_block(function, "else");

        self.builder.position_at_end(entry);

        let val = i64.const_int(12, false);
        let ptr_type = i64.ptr_type(AddressSpace::Generic);
        let ptr_val = val.const_to_pointer(ptr_type);
        let loaded_val = self.builder.build_load(ptr_val, "load_value");
        let add = self.builder.build_int_add(
            loaded_val.into_int_value(),
            loaded_val.into_int_value(),
            "addvars",
        );

        let string_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let print_fn = self.get_print_fn();

        let as_str = i64.const_int(0, false).print_to_string();
        string_type.const_array(as_str.to_bytes());

        self.builder.build_call(print_fn, &[loaded_val.as_basic_value_enum().into()], "print_added_value");
        //self.print(add.as_basic_value_enum().into_int_value().print_to_string().to_str().unwrap());

        let comp = self
            .builder
            .build_int_compare(IntPredicate::EQ, num_a, num_b, "compare");

        let val = format!(
            "Values: {}, {}",
            num_a.print_to_string().to_string(),
            num_b.print_to_string().to_string()
        );
        let message = val.as_str();
        self.print(message);

        self.builder
            .build_conditional_branch(comp, then_block, else_block);

        self.builder.position_at_end(then_block);
        let true_val = bool.const_int(1, false);
        self.builder.build_return(Some(&true_val));

        self.builder.position_at_end(else_block);
        let false_val = bool.const_int(0, false);
        self.builder.build_return(Some(&false_val));

        unsafe { self.execution_engine.get_function("main").ok() }
    }

    fn get_back(&self) -> Module<'ctx> {
        self.module.to_owned()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("test.ll");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Less)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let test = codegen.test_program().ok_or("Unable to compile")?;

    codegen.module.print_to_file("out.ll");

    /*     unsafe {
        println!("{}", test.call(50, 50));
    } */

    Target::initialize_all(&InitializationConfig::default());

    let target_triple = TargetMachine::get_default_triple();

    let target = Target::from_triple(&target_triple).map_err(|e| format!("{:?}", e))?;
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .ok_or_else(|| "Unable to create target machine!".to_string())?;

    target_machine
        .write_to_file(&codegen.get_back(), FileType::Object, "test".as_ref())
        .map_err(|e| format!("{:?}", e))?;

    Ok(())
}
