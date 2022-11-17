
use core::marker::PhantomData;
use zephyr_sys::raw::{
	k_objects,
	k_msgq,
	k_msgq_attrs,
	k_timeout_t
};
use super::NegErr;
use crate::kobj::*;
use crate::time::Timeout;

// Declare the Zephyr struct to be a kernel object
unsafe impl KObj for k_msgq {
    const OTYPE: k_objects = zephyr_sys::raw::k_objects_K_OBJ_MSGQ;
}

pub use zephyr_sys::raw::k_msgq as KMsgq;

crate::make_static_wrapper!(k_msgq, zephyr_sys::raw::k_msgq);

/// Raw syscall API
pub trait MsgQueueSyscalls {
	unsafe fn k_msgq_alloc_init(
		msgq: &k_msgq,
		msg_size: usize,
		max_msgs: u32,
	    ) -> libc::c_int;
	unsafe fn k_msgq_put(
		msgq: &k_msgq,
		data: *const libc::c_void,
		timeout: k_timeout_t,
	) -> libc::c_int;
	unsafe fn k_msgq_get(
		msgq: &k_msgq,
		data: *mut libc::c_void,
		timeout: k_timeout_t,
	) -> libc::c_int;
	unsafe fn k_msgq_peek(msgq: &k_msgq, data: *mut libc::c_void) -> libc::c_int;
	unsafe fn k_msgq_purge(msgq: &k_msgq);
	unsafe fn k_msgq_num_free_get(msgq: &k_msgq) -> u32;
	unsafe fn k_msgq_get_attrs(msgq: &k_msgq, attrs: *mut k_msgq_attrs);
	unsafe fn k_msgq_num_used_get(msgq: &k_msgq) -> u32;
}

macro_rules! trait_impl {
	($context:ident, $context_struct:path) => {
	    impl MsgQueueSyscalls for $context_struct {
		unsafe fn k_msgq_alloc_init(
			msgq: &k_msgq,
			msg_size: usize,
			max_msgs: u32,
		    ) -> libc::c_int {
			zephyr_sys::syscalls::$context::k_msgq_alloc_init(
				msgq as *const _ as *mut _,
				msg_size,
				max_msgs,
			)
		}

		unsafe fn k_msgq_put(
			msgq: &k_msgq,
			data: *const libc::c_void,
			timeout: k_timeout_t,
		) -> libc::c_int {
			zephyr_sys::syscalls::$context::k_msgq_put(
				msgq as *const _ as *mut _,
				data,
				timeout,
			)
	    	}

		unsafe fn k_msgq_get(
			msgq: &k_msgq,
			data: *mut libc::c_void,
			timeout: k_timeout_t,
		) -> libc::c_int {
			zephyr_sys::syscalls::$context::k_msgq_get(
				msgq as *const _ as *mut _,
				data,
				timeout,
			)
	    	}

		unsafe fn k_msgq_peek(msgq: &k_msgq, data: *mut libc::c_void) -> libc::c_int {
			zephyr_sys::syscalls::$context::k_msgq_peek(
				msgq as *const _ as *mut _,
				data,
			)
	    	}

		unsafe fn k_msgq_purge(msgq: &k_msgq) {
			zephyr_sys::syscalls::$context::k_msgq_purge(
				msgq as *const _ as *mut _,
			)
	    	}

		unsafe fn k_msgq_num_free_get(msgq: &k_msgq) -> u32 {
			zephyr_sys::syscalls::$context::k_msgq_num_free_get(
				msgq as *const _ as *mut _,
			)
	    	}

		unsafe fn k_msgq_get_attrs(msgq: &k_msgq, attrs: *mut k_msgq_attrs) {
			zephyr_sys::syscalls::$context::k_msgq_get_attrs(
				msgq as *const _ as *mut _,
				attrs,
			)
	    	}

		unsafe fn k_msgq_num_used_get(msgq: &k_msgq) -> u32 {
			zephyr_sys::syscalls::$context::k_msgq_num_used_get(
				msgq as *const _ as *mut _,
			)
	    	}
	    }
	};
}

trait_impl!(kernel, crate::context::Kernel);
trait_impl!(user, crate::context::User);
trait_impl!(any, crate::context::Any);

pub enum QueueError {
	Timeout,
	Full,
	Empty,
	NoMem
}

/// Safer API implemented for the msgq kobject.
pub trait MsgQueue<T, C> {
	fn init(&self, max_msgs: u32) -> Result<(), QueueError>;
	fn put(&self, msg: &T, timeout: Timeout) -> Result<(), QueueError>;
	fn get(&self, timeout: Timeout) -> Result<T, QueueError>;
	fn peek(&self) -> Result<&T, QueueError>;
	fn clear(&self);
	fn capacity(&self) -> usize;
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool;
	fn is_full(&self) -> bool;
}

impl<T: Copy + Default, C: MsgQueueSyscalls> MsgQueue<T, C> for k_msgq {

	fn init(&self, max_msgs: u32) -> Result<(), QueueError> {
		unsafe {
			match C::k_msgq_alloc_init(
				self,
				core::mem::size_of::<T>(),
				max_msgs,
			).neg_err() {
				Ok(_) => Ok(()),
				Err(e) => Err(QueueError::NoMem),
			}
		}
	}

	fn put(&self, msg: &T, timeout: Timeout) -> Result<(), QueueError> {
		unsafe {
			match C::k_msgq_put(
				self,
				msg as *const T as *const libc::c_void,
				timeout.0,
			).neg_err() {
				Ok(_) => Ok(()),
				Err(e) => Err(QueueError::Full),
			}
		}
	}

	fn get(&self, timeout: Timeout) -> Result<T, QueueError> {
		let mut msg = T::default();
		unsafe {
			match C::k_msgq_get(
				self,
				&mut msg as *mut T as *mut libc::c_void,
				timeout.0,
			).neg_err() {
				Ok(_) => Ok(msg),
				Err(e) => Err(QueueError::Empty),
			}
		}
	}

	fn peek(&self) -> Result<&T, QueueError> {
		let mut msg: *mut T = core::ptr::null_mut();
		unsafe {
			match C::k_msgq_peek(
				self,
				&mut msg as *mut *mut T as *mut libc::c_void,
			).neg_err() {
				Ok(_) => Ok(&*msg),
				Err(e) => Err(QueueError::Empty),
			}
		}
	}

	fn clear(&self) {
		unsafe {
			C::k_msgq_purge(
				self,
			)
		}
	}

	fn capacity(&self) -> usize {
		unsafe { C::k_msgq_num_free_get(self) as usize }
	}

	fn len(&self) -> usize {
		unsafe { C::k_msgq_num_used_get(self) as usize }
	}

	fn is_empty(&self) -> bool {
		<zephyr_sys::raw::k_msgq as MsgQueue<T, C>>::len() == 0
	}

	fn is_full(&self) -> bool {
		<zephyr_sys::raw::k_msgq as MsgQueue<T, C>>::len(self) == <zephyr_sys::raw::k_msgq as MsgQueue<T, C>>::capacity(self)
	}
}

/*
impl<T: Copy + Default, C: MsgQueueSyscalls> MsgQueue<T, C> for MessageQueue<T, C> {
	fn new(max_msgs: u32) -> Result<Self, QueueError> {
		let mut msgq = k_msgq::default();
		let msg_size = core::mem::size_of::<T>();
		unsafe {
			match C::k_msgq_alloc_init(&mut msgq as *mut k_msgq, msg_size, max_msgs).neg_err() {
				Ok(_) => Ok(MessageQueue {
					msgq,
					msg_size,
					max_msgs,
					_phantom_t: PhantomData,
					_phantom_c: PhantomData,
				}),
				Err(e) => Err(QueueError::NoMem),
			}
		}
	}

	fn put(&self, msg: &T, timeout: Timeout) -> Result<(), QueueError> {
		unsafe {
			match C::k_msgq_put(self.deref_self_mut(), msg as *const T as *const libc::c_void, timeout.into()).neg_err() {
				Ok(_) => Ok(()),
				Err(e) => Err(QueueError::Full),
			}
		}
	}

	fn get(&self, timeout: Timeout) -> Result<T, QueueError> {
		let mut msg = T::default();
		unsafe {
			match C::k_msgq_get(self.deref_self_mut(), &mut msg as *mut T as *mut libc::c_void, timeout).neg_err() {
				Ok(_) => Ok(msg),
				Err(e) => Err(QueueError::Timeout)
			}
		}
	}

	fn peek(&self) -> Result<&T, QueueError> {
		let mut msg: *mut T = core::ptr::null_mut();
		unsafe {
			match C::k_msgq_peek(self.deref_self(), &mut msg as *mut *mut T as *mut libc::c_void).neg_err() {
				Ok(_) => Ok(&*msg),
				Err(e) => Err(QueueError::Empty)
			}
		}
	}

	fn clear(&self) {
		unsafe {
			C::k_msgq_purge(self.deref_self());
		}
	}

	fn capacity(&self) -> usize {
		unsafe {
			let mut attrs = k_msgq_attrs {
				msg_size: 0,
				max_msgs: 0,
				used_msgs: 0,
			};
			C::k_msgq_get_attrs(self, &mut attrs);
			attrs.max_msgs as usize
		}
	}

	fn len(&self) -> usize {
		unsafe {
			let mut attrs = k_msgq_attrs {
				msg_size: 0,
				max_msgs: 0,
				used_msgs: 0,
			};
			C::k_msgq_get_attrs(self, &mut attrs);
			attrs.used_msgs as usize
		}
	}

	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn is_full(&self) -> bool {
		unsafe {
			let mut attrs = k_msgq_attrs {
				msg_size: 0,
				max_msgs: 0,
				used_msgs: 0,
			};
			C::k_msgq_get_attrs(self, &mut attrs);
			attrs.used_msgs == attrs.max_msgs
		}
	}
}
*/

/*
#[derive(Clone, Copy, Debug)]
pub enum MsgQueueError {
    NoMem,
}

pub struct MsgQueue<C> {
	msgq: k_msgq,
}

impl<C: MsgQueueSyscalls> MsgQueue<C> {
	pub fn new(msg_size: usize, max_msgs: u32) -> Result<Self, MsgQueueError> {
		let mut msgq = k_msgq::default();
		unsafe {
			match C::msgq_alloc_init(&mut msgq, msg_size, max_msgs) {

			}
		}
		Ok(Self { msgq })
	}

	pub fn put(&mut self, data: &[u8]) -> io::Result<()> {
		unsafe { C::msgq_put(&mut self.msgq, data.as_ptr() as *const _, zephyr_sys::raw::K_FOREVER) }
	}

	pub fn try_put(&mut self, data: &[u8], timeout: Timeout) -> io::Result<()> {
		unsafe { C::msgq_put(&mut self.msgq, data.as_ptr() as *const _, timeout) }
	}
}

// only required for the kernel context
impl Drop for MsgQueue<crate::context::Kernel> {
	fn drop(&mut self) {
		unsafe {
			zephyr_sys::syscalls::kernel::msgq_cleanup(&mut self.msgq);
		}
	}
}
*/
/* FUNCTIONS

extern "C" {
    #[doc = " @brief Initialize a message queue."]
    #[doc = ""]
    #[doc = " This routine initializes a message queue object, prior to its first use."]
    #[doc = ""]
    #[doc = " The message queue's ring buffer must contain space for @a max_msgs messages,"]
    #[doc = " each of which is @a msg_size bytes long. The buffer must be aligned to an"]
    #[doc = " N-byte boundary, where N is a power of 2 (i.e. 1, 2, 4, ...). To ensure"]
    #[doc = " that each message is similarly aligned to this boundary, @a q_msg_size"]
    #[doc = " must also be a multiple of N."]
    #[doc = ""]
    #[doc = " @param msgq Address of the message queue."]
    #[doc = " @param buffer Pointer to ring buffer that holds queued messages."]
    #[doc = " @param msg_size Message size (in bytes)."]
    #[doc = " @param max_msgs Maximum number of messages that can be queued."]
    #[doc = ""]
    #[doc = " @return N/A"]
    pub fn k_msgq_init(
        msgq: *mut k_msgq,
        buffer: *mut libc::c_char,
        msg_size: usize,
        max_msgs: u32,
    );
}
extern "C" {
    #[doc = " @brief Release allocated buffer for a queue"]
    #[doc = ""]
    #[doc = " Releases memory allocated for the ring buffer."]
    #[doc = ""]
    #[doc = " @param msgq message queue to cleanup"]
    #[doc = ""]
    #[doc = " @retval 0 on success"]
    #[doc = " @retval -EBUSY Queue not empty"]
    pub fn k_msgq_cleanup(msgq: *mut k_msgq) -> libc::c_int;
}

*/


/* SYSCALLS

extern "C" {
    pub fn z_anyctx_k_msgq_alloc_init(
        msgq: *mut k_msgq,
        msg_size: usize,
        max_msgs: u32,
    ) -> libc::c_int;
}

extern "C" {
    pub fn z_anyctx_k_msgq_put(
        msgq: *mut k_msgq,
        data: *const libc::c_void,
        timeout: k_timeout_t,
    ) -> libc::c_int;
}

extern "C" {
    pub fn z_anyctx_k_msgq_get(
        msgq: *mut k_msgq,
        data: *mut libc::c_void,
        timeout: k_timeout_t,
    ) -> libc::c_int;
}

extern "C" {
    pub fn z_anyctx_k_msgq_peek(msgq: *mut k_msgq, data: *mut libc::c_void) -> libc::c_int;
}

extern "C" {
    pub fn z_anyctx_k_msgq_purge(msgq: *mut k_msgq);
}

extern "C" {
    pub fn z_anyctx_k_msgq_num_free_get(msgq: *mut k_msgq) -> u32;
}

extern "C" {
    pub fn z_anyctx_k_msgq_get_attrs(msgq: *mut k_msgq, attrs: *mut k_msgq_attrs);
}

extern "C" {
    pub fn z_anyctx_k_msgq_num_used_get(msgq: *mut k_msgq) -> u32;
}



*/