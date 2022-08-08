use super::configure::STEP_SIZE;
use super::*;
use crate::curr;
use crate::fixed_curr;
use crate::nextn;
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::plonk::Expression;
use halo2_proofs::plonk::VirtualCells;

pub(crate) const ROTATION_INDEX_LTYPE: i32 = 0;
pub(crate) const ROTATION_INDEX_MMID: i32 = 1;
pub(crate) const ROTATION_INDEX_OFFSET: i32 = 2;
pub(crate) const ROTATION_INDEX_EID: i32 = 3;
pub(crate) const ROTATION_INDEX_EMID: i32 = 4;

pub(crate) const ROTATION_CONSTANT_ONE: i32 = 0;
pub(crate) const ROTATION_SAME_LTYPE: i32 = 1;
pub(crate) const ROTATION_SAME_MMID: i32 = 2;
pub(crate) const ROTATION_SAME_OFFSET: i32 = 3;
pub(crate) const ROTATION_SAME_EID: i32 = 4;
pub(crate) const ROTATION_ATYPE: i32 = 5;
pub(crate) const ROTATION_VTYPE: i32 = 6;
pub(crate) const ROTATION_REST_MOPS: i32 = 7;
pub(crate) const ROTATION_VALUE: i32 = 8;

impl<F: FieldExt> MemoryTableConfig<F> {
    pub(super) fn is_enabled_block(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        curr!(meta, self.enable)
            * fixed_curr!(meta, self.sel)
            * fixed_curr!(meta, self.block_first_line_sel)
    }

    pub(super) fn is_enabled_following_block(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        curr!(meta, self.enable)
            * fixed_curr!(meta, self.block_first_line_sel)
            * fixed_curr!(meta, self.following_block_sel)
    }

    pub(super) fn is_enabled_line(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        fixed_curr!(meta, self.sel)
    }

    pub(super) fn is_enabled_following_line(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        fixed_curr!(meta, self.following_block_sel)
    }

    pub(super) fn same_ltype_single(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.index.same, ROTATION_INDEX_LTYPE)
    }

    pub(super) fn same_mmid_single(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.index.same, ROTATION_INDEX_MMID)
    }

    pub(super) fn same_offset_single(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_INDEX_OFFSET)
    }

    pub(super) fn same_eid_single(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_INDEX_EID)
    }

    pub(super) fn same_ltype(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        self.same_ltype_single(meta)
    }

    pub(super) fn same_mmid(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_SAME_MMID)
    }

    pub(super) fn same_offset(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_SAME_OFFSET)
    }

    pub(super) fn same_eid(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_SAME_EID)
    }

    pub(super) fn ltype(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.index.data, ROTATION_INDEX_LTYPE)
    }

    pub(super) fn atype(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_ATYPE)
    }

    pub(super) fn vtype(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_VTYPE)
    }

    pub(super) fn prev_vtype(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_VTYPE)
    }

    pub(super) fn value(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_VALUE)
    }

    pub(super) fn prev_value(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_VALUE - STEP_SIZE)
    }

    pub(super) fn rest_mops(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_REST_MOPS)
    }

    pub(super) fn prev_rest_mops(&self, meta: &mut VirtualCells<F>) -> Expression<F> {
        nextn!(meta, self.aux, ROTATION_REST_MOPS - STEP_SIZE)
    }
}