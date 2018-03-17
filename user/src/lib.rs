extern crate unreachable;

pub fn un_everything() {
    unsafe { unreachable::unreachable(); }
}
