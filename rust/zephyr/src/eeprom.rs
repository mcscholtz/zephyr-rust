use std::io;

use zephyr_sys::raw::off_t;

use super::NegErrno;
use crate::device::Device;

/// Raw syscall API
pub trait EepromSyscalls {
    unsafe fn eeprom_read(device: *mut Device, offset: off_t, data: &mut [u8]) -> io::Result<()>;
    unsafe fn eeprom_write(device: *mut Device, offset: off_t, data: &[u8]) -> io::Result<()>;
    unsafe fn eeprom_get_size(device: *mut Device) -> usize;
}

macro_rules! trait_impl {
    ($context:ident, $context_struct:path) => {
        impl EepromSyscalls for $context_struct {
            #[inline(always)]
            unsafe fn eeprom_read(
                device: *mut Device,
                offset: off_t,
                data: &mut [u8],
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::eeprom_read(
                    device,
                    offset,
                    data.as_mut_ptr() as *mut _,
                    data.len(),
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn eeprom_write(
                device: *mut Device,
                offset: off_t,
                data: &[u8],
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::eeprom_write(
                    device,
                    offset,
                    data.as_ptr() as *const _,
                    data.len(),
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn eeprom_get_size(device: *mut Device) -> usize {
                zephyr_sys::syscalls::$context::eeprom_get_size(device)
            }
        }
    };
}

trait_impl!(kernel, crate::context::Kernel);
trait_impl!(user, crate::context::User);
trait_impl!(any, crate::context::Any);

