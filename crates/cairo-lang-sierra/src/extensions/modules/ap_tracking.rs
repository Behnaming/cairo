use crate::extensions::lib_func::{
    LibfuncSignature, SierraApChange, SignatureSpecializationContext,
};
use crate::extensions::{NoGenericArgsGenericLibfunc, SpecializationError};
use crate::ids::GenericLibfuncId;

/// Revoke the ap tracking.
/// This Libfunc is changes to ap_tracking state to unknown,
/// allowing a path with known ap tracking to converge with a path with unknown ap tracking.
#[derive(Default)]
pub struct RevokeApTrackingLibfunc {}
impl NoGenericArgsGenericLibfunc for RevokeApTrackingLibfunc {
    const ID: GenericLibfuncId = GenericLibfuncId::new_inline("revoke_ap_tracking");

    fn specialize_signature(
        &self,
        _context: &dyn SignatureSpecializationContext,
    ) -> Result<LibfuncSignature, SpecializationError> {
        Ok(LibfuncSignature::new_non_branch(vec![], vec![], SierraApChange::Unknown))
    }
}
