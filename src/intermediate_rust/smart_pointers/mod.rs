use std::io::Result;

mod box_t;
mod rc_t;
mod ref_cell_t;

#[allow(dead_code)]
pub enum SmartPointer {
    Nothing,
    BoxT,
    RcT,
    RefCellT,
}

pub fn smart_pointer(smart_pointer: SmartPointer) -> Result<()> {
    match smart_pointer {
        SmartPointer::BoxT => box_t::box_t(),
        SmartPointer::RcT => rc_t::rc_t(),
        SmartPointer::RefCellT => ref_cell_t::ref_cell_t(),
        _ => Ok(()),
    }
}
