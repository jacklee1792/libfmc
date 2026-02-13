use super::ops;
use crate::*;

/// View of a single corner slot on the cube.
#[derive(Copy, Clone)]
pub struct CornerRef<'a> {
    pub(super) corners: &'a Corners,
    pub(super) slot: Corner,
}

impl<'a> CornerRef<'a> {
    /// The corner at the slot.
    pub fn piece(&self) -> Corner {
        let cp = ops::lane_cp(self.corners.0, self.slot as usize);
        Corner::from(cp)
    }

    /// Corner orientation with respect to UD axis at the slot.
    pub fn coud(&self) -> CO {
        let co = ops::lane_coud(self.corners.0, self.slot as usize);
        CO::from(co)
    }

    /// Corner orientation with respect to FB axis at the slot.
    pub fn cofb(&self) -> CO {
        let slot = self.slot as usize;
        let mut co = self.coud();
        if ops::lane_htrbad(self.corners.0, slot) {
            co = co + ops::COUD_TO_COFB[slot].into()
        }
        co
    }

    /// Corner orientation with respect to LR axis at the slot.
    pub fn colr(&self) -> CO {
        let slot = self.slot as usize;
        let mut co = self.coud();
        if ops::lane_htrbad(self.corners.0, slot) {
            co = co + ops::COUD_TO_COLR[slot].into()
        }
        co
    }
}
