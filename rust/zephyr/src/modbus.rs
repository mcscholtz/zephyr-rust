use std::io;

use zephyr_sys::raw::{
	modbus_serial_param,
	modbus_server_param,
	modbus_user_callbacks,
	modbus_iface_param,
	modbus_iface_param__bindgen_ty_1 as modbus_iface_param__rtu,
	modbus_iface_param__bindgen_ty_2 as modbus_iface_param__raw,
	modbus_mode,
	modbus_mode_MODBUS_MODE_RTU,
	modbus_mode_MODBUS_MODE_ASCII,
	modbus_mode_MODBUS_MODE_RAW,
	uart_config_parity,
	uart_config_parity_UART_CFG_PARITY_NONE,
	uart_config_parity_UART_CFG_PARITY_ODD,
	uart_config_parity_UART_CFG_PARITY_EVEN,
	uart_config_parity_UART_CFG_PARITY_MARK,
	uart_config_parity_UART_CFG_PARITY_SPACE,
};

use super::NegErrno;
use crate::device::Device;

pub trait ModbusCommonSyscalls {
	fn modbus_iface_get_by_name(iface_name: *const libc::c_char) -> io::Result<()>;
	fn modbus_disable(iface: u8) -> io::Result<()>;
}

pub trait ModbusServerSyscalls {
	fn modbus_init_server(iface: libc::c_int, param: modbus_iface_param) -> io::Result<()>;

	#[no_mangle]
	extern "C" {
		#[doc = " Coil read callback"]
		fn modbus_read_coils_cb(addr: u16, state: *mut bool) -> libc::c_int;
		#[doc = " Coil write callback"]
		fn modbus_write_coils_cb(addr: u16, state: bool) -> libc::c_int;
		#[doc = " Discrete Input read callback"]
		fn modbus_read_dinputs_cb(addr: u16, state: *mut bool) -> libc::c_int;
		#[doc = " Input Register read callback"]
		fn modbus_read_input_regs_cb(addr: u16, reg: *mut u16) -> libc::c_int;
		#[doc = " Holding Register read callback"]
		fn modbus_read_holding_reg_cb(addr: u16, reg: *mut u16) -> libc::c_int;
		#[doc = " Holding Register write callback"]
		fn modbus_write_holding_reg_cb(addr: u16, reg: u16) -> libc::c_int;
	}
}

pub trait ModbusClientSyscalls {
	fn modbus_init_client(iface: libc::c_int, param: modbus_iface_param) -> io::Result<()>;
	fn modbus_read_coils(
		iface: libc::c_int,
		unit_id: u8,
		start_addr: u16,
		coil_tbl: *mut u8,
		num_coils: u16,
	) -> io::Result<()>;
	fn modbus_read_dinputs(
		iface: libc::c_int,
		unit_id: u8,
		start_addr: u16,
		di_tbl: *mut u8,
		num_di: u16,
	) -> io::Result<()>;
	fn modbus_read_holding_regs(
		iface: libc::c_int,
		unit_id: u8,
		start_addr: u16,
		reg_buf: *mut u16,
		num_regs: u16,
	) -> io::Result<()>;
	fn modbus_read_input_regs(
		iface: libc::c_int,
		unit_id: u8,
		start_addr: u16,
		reg_buf: *mut u16,
		num_regs: u16,
	) -> io::Result<()>;
	fn modbus_write_coil(
		iface: libc::c_int,
		unit_id: u8,
		coil_addr: u16,
		coil_state: bool,
	) -> io::Result<()>;
	fn modbus_write_holding_reg(
		iface: libc::c_int,
		unit_id: u8,
		start_addr: u16,
		reg_val: u16,
	) -> io::Result<()>;
	fn modbus_request_diagnostic(
		iface: libc::c_int,
		unit_id: u8,
		sfunc: u16,
		data: u16,
		data_out: *mut u16,
	) -> io::Result<()>;
	fn modbus_write_coils(
		iface: libc::c_int,
		unit_id: u8,
		start_addr: u16,
		coil_tbl: *mut u8,
		num_coils: u16,
	) -> io::Result<()>;
	fn modbus_write_holding_regs(
		iface: libc::c_int,
		unit_id: u8,
		start_addr: u16,
		reg_buf: *mut u16,
		num_regs: u16,
	) -> io::Result<()>;
}


pub struct ModbusServer {
	iface: u8,
}

impl ModbusServer {
	pub fn new(iface: u8) -> Self {
		Self { iface }
	}
}

impl