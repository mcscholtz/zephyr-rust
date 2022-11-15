use std::io;

pub use zephyr_sys::raw::{
    gpio_pin_t as Pin,
    gpio_flags_t as Flags,
    gpio_port_value_t as PortValue,
    gpio_port_pins_t as PortPins,
};

use super::NegErrno;
use crate::device::Device;

/// Raw syscall API
pub trait GpioSyscalls {
    unsafe fn gpio_pin_interrupt_configure(
        port: *const Device, pin: Pin, flags: Flags) -> io::Result<()>;
    unsafe fn gpio_pin_configure(
        port: *const Device, pin: Pin, flags: Flags) -> io::Result<()>;
    unsafe fn gpio_port_get_raw(
        port: *const Device, value: *mut PortValue) -> io::Result<()>;
    unsafe fn gpio_port_set_masked_raw(
        port: *const Device, mask: PortPins, value: PortValue) -> io::Result<()>;
    unsafe fn gpio_port_set_bits_raw(
        port: *const Device, pins: PortPins) -> io::Result<()>;
    unsafe fn gpio_port_clear_bits_raw(
        port: *const Device, pins: PortPins) -> io::Result<()>;
    unsafe fn gpio_port_toggle_bits(
        port: *const Device, pins: PortPins) -> io::Result<()>;
    unsafe fn gpio_get_pending_int(dev: *const device) -> u32;
}

macro_rules! trait_impl {
    ($context:ident, $context_struct:path) => {
        impl GpioSyscalls for $context_struct {
            #[inline(always)]
            unsafe fn gpio_pin_interrupt_configure(
                port: *const Device, 
                pin: Pin, 
                flags: Flags
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::gpio_pin_interrupt_configure(
                    port,
                    pin,
                    flags
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn gpio_pin_configure(
                port: *const Device, 
                pin: Pin, 
                flags: Flags
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::gpio_pin_configure(
                    port,
                    pin,
                    flags
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn gpio_port_get_raw(
                port: *const Device, 
                value: *mut PortValue
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::gpio_port_get_raw(
                    port,
                    value
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn gpio_port_set_masked_raw(
                port: *const Device, 
                mask: PortPins, 
                value: PortValue
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::gpio_port_set_masked_raw(
                    port,
                    mask,
                    value
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn gpio_port_set_bits_raw(
                port: *const Device, 
                pins: PortPins
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::gpio_port_set_bits_raw(
                    port,
                    pins
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn gpio_port_clear_bits_raw(
                port: *const Device, 
                pins: PortPins
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::gpio_port_clear_bits_raw(
                    port,
                    pins
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn gpio_port_toggle_bits(
                port: *const Device, 
                pins: PortPins
            ) -> io::Result<()> {
                zephyr_sys::syscalls::$context::gpio_port_toggle_bits(
                    port,
                    pins
                )
                .zero_or_neg_errno()
            }

            #[inline(always)]
            unsafe fn gpio_get_pending_int(dev: *const device) -> u32 {
                zephyr_sys::syscalls::$context::gpio_get_pending_int(dev)
            }
        }
    };
}

trait_impl!(kernel, crate::context::Kernel);
trait_impl!(user, crate::context::User);
trait_impl!(any, crate::context::Any);