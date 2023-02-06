use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use std::error::Error;
use inkwell::targets::{CodeModel, RelocMode, Target, TargetMachine};

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>
}

impl<'ctx> CodeGen<'ctx> {
    fn compile_sum(&self) -> Result<String, Box<dyn Error>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0).unwrap().into_int_value();
        let y = function.get_nth_param(1).unwrap().into_int_value();
        let z = function.get_nth_param(2).unwrap().into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum");
        let sum = self.builder.build_int_add(sum, z, "sum");

        self.builder.build_return(Some(&sum));

        Target::initialize_x86(&Default::default());

        println!("Target: {}", TargetMachine::get_default_triple());

        let target = Target::from_triple(&TargetMachine::get_default_triple()).unwrap();
        let target_machine = target
            .create_target_machine(
                &TargetMachine::get_default_triple(),
                "generic",
                "",
                OptimizationLevel::Aggressive,
                RelocMode::Default,
                CodeModel::Default
            ).unwrap();

        let res = target_machine.write_to_memory_buffer(
            &self.module,
            inkwell::targets::FileType::Assembly
        )?;

        Ok(String::from_utf8_lossy(res.as_slice()).parse().unwrap())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("sum");
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
    };

    let res = codegen.compile_sum()?;
    println!("{res}");

    Ok(())
}