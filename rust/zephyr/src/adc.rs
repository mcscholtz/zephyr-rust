use std::io;

use zephyr_sys::raw::{
    adc_channel_cfg,
    adc_sequence,
    k_poll_signal,
    adc_driver_api
};

use super::NegErrno;
use crate::device::Device;

/// Raw syscall API
pub trait AdcSyscalls {
    unsafe fn adc_channel_setup(device: *const Device, channel_cfg: *const adc_channel_cfg) -> io::Result<()>;
    unsafe fn adc_read(device: *const Device, sequence: *const adc_sequence) -> io::Result<()>;
    #[cfg(adc_async)]
    unsafe fn adc_read_async(device: *mut Device, sequence: *const adc_sequence, async_: *mut k_poll_signal) -> io::Result<()>;
    unsafe fn adc_ref_internal(device: *const Device) -> u16;
}

macro_rules! trait_impl {
    ($context:ident, $context_struct:path) => {
        impl AdcSyscalls for $context_struct {
            #[inline(always)]
            unsafe fn adc_channel_setup(
                device: *const Device,
                channel_cfg: *const adc_channel_cfg
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::adc_channel_setup(
                    device,
                    channel_cfg
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn adc_read(
                device: *const Device,
                sequence: *const adc_sequence
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::adc_read(
                    device,
                    sequence
                )
                .zero_or_neg_errno()
            }

            #[cfg(adc_async)]
            #[inline(always)]
            unsafe fn adc_read_async(
                device: *const Device,
                sequence: *const adc_sequence,
                async_: *mut k_poll_signal
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::adc_read_async(
                    device,
                    sequence,
                    async_
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn adc_ref_internal(
                device: *const Device,
            ) -> u16 {
                // This is not actually a syscall, but it matches the API
                let dev: &Device = &*device;
                let api: &adc_driver_api = &*(dev.api as *const adc_driver_api);
                api.ref_internal
            }
        }
    };
}

trait_impl!(kernel, crate::context::Kernel);
trait_impl!(user, crate::context::User);
trait_impl!(any, crate::context::Any);
