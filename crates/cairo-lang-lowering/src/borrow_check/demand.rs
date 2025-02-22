use cairo_lang_utils::ordered_hash_set::OrderedHashSet;

use super::BorrowChecker;
use crate::diagnostic::LoweringDiagnosticKind::*;
use crate::VariableId;

/// Demanded variables from a certain point in the flow until the end of the function.
#[derive(Clone, Default)]
pub struct Demand {
    pub vars: OrderedHashSet<VariableId>,
}

impl Demand {
    /// Updates the demand when some variables are used right before the current flow.
    pub fn variables_used(&mut self, borrow_checker: &mut BorrowChecker<'_>, vars: &[VariableId]) {
        for var in vars {
            if !self.vars.insert(*var) {
                // Variable already used. If it's not dup, that is an issue.
                if !borrow_checker.lowered.variables[*var].duplicatable {
                    borrow_checker
                        .diagnostics
                        .report(borrow_checker.dummy_stable_ptr, VariableMoved);
                }
            }
        }
    }

    /// Updates the demand when some variables are introduced right before the current flow.
    pub fn variables_introduced(
        &mut self,
        borrow_checker: &mut BorrowChecker<'_>,
        vars: &[VariableId],
    ) {
        for var in vars {
            if !self.vars.swap_remove(var) {
                // Variable introduced, but not demanded. If it's not drop, that is an issue.
                // Currently disabled, since Drop is not properly implemented everywhere.
                #[allow(clippy::overly_complex_bool_expr)]
                if false && !borrow_checker.lowered.variables[*var].droppable {
                    borrow_checker
                        .diagnostics
                        .report(borrow_checker.dummy_stable_ptr, VariableNotDropped);
                }
            }
        }
    }
}
