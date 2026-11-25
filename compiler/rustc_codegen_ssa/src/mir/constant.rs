use rustc_middle::mir;
use rustc_middle::ty::layout::{HasTyCtxt, HasTypingEnv};

use super::FunctionCx;
use crate::mir::operand::OperandRef;
use crate::traits::*;

impl<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>> FunctionCx<'a, 'tcx, Bx> {
    pub(crate) fn eval_mir_constant_to_operand(
        &self,
        bx: &mut Bx,
        constant: &mir::ConstOperand<'tcx>,
    ) -> OperandRef<'tcx, Bx::Value> {
        let val = self.eval_mir_constant(constant);
        let ty = self.monomorphize(constant.ty());
        OperandRef::from_const(bx, val, ty)
    }

    pub fn eval_mir_constant(&self, constant: &mir::ConstOperand<'tcx>) -> mir::ConstValue {
        // `MirUsedCollector` visited all required_consts before codegen began, so if we got here
        // there can be no more constants that fail to evaluate.
        self.monomorphize(constant.const_)
            .eval(self.cx.tcx(), self.cx.typing_env(), constant.span)
            .expect("erroneous constant missed by mono item collection")
    }
}
