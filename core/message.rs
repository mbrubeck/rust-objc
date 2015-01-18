use std::mem;
use std::ptr;

use runtime::{Class, Object, Sel, self};
use {Encode, EncodePtr};

/*
 The Sized bound on Message is unfortunate; ideally, objc objects would not be
 treated as Sized. However, rust won't allow casting a dynamically-sized type
 pointer to an Object pointer, because dynamically-sized types can have fat
 pointers (two words) instead of real pointers.
 */
/// Types that may be sent Objective-C messages.
/// For example: objects, classes, and blocks.
pub unsafe trait Message: Sized + EncodePtr { }

unsafe impl Message for Object { }

unsafe impl Message for Class { }

/// A trait for converting to a pointer to a type that may be sent Objective-C
/// messages.
pub trait ToMessage : Encode {
    type Target: Message;

    fn as_ptr(&self) -> *mut Self::Target;

    fn is_nil(&self) -> bool {
        self.as_ptr().is_null()
    }
}

impl<T> ToMessage for *const T where T: Message {
    type Target = T;

    fn as_ptr(&self) -> *mut T {
        *self as *mut T
    }
}

impl<T> ToMessage for *mut T where T: Message {
    type Target = T;

    fn as_ptr(&self) -> *mut T {
        *self
    }
}

impl<'a, T> ToMessage for &'a T where T: Message {
    type Target = T;

    fn as_ptr(&self) -> *mut T {
        *self as *const T as *mut T
    }
}

impl<'a, T> ToMessage for &'a mut T where T: Message {
    type Target = T;

    fn as_ptr(&self) -> *mut T {
        *self
    }
}

impl<'a, T> ToMessage for Option<&'a T> where T: Message {
    type Target = T;

    fn as_ptr(&self) -> *mut T {
        match *self {
            None => ptr::null_mut(),
            Some(ref obj) => obj.as_ptr(),
        }
    }
}

impl<'a, T> ToMessage for Option<&'a mut T> where T: Message {
    type Target = T;

    fn as_ptr(&self) -> *mut T {
        match *self {
            None => ptr::null_mut(),
            Some(ref obj) => obj.as_ptr(),
        }
    }
}

fn msg_send_fn<R>() -> unsafe extern fn(*mut Object, Sel, ...) -> R {
    unsafe {
        mem::transmute(runtime::objc_msgSend)
    }
}

pub trait MessageArguments {
    unsafe fn send<T, R>(self, obj: &T, sel: Sel) -> R where T: ToMessage;
}

macro_rules! message_args_impl {
    ($($a:ident : $t:ident),*) => (
        impl<$($t),*> MessageArguments for ($($t,)*) {
            unsafe fn send<T, R>(self, obj: &T, sel: Sel) -> R where T: ToMessage {
                let msg_send_fn = msg_send_fn::<R>();
                let msg_send_fn: unsafe extern fn(*mut Object, Sel $(, $t)*) -> R =
                    mem::transmute(msg_send_fn);
                let obj_ptr = obj.as_ptr() as *mut Object;
                let ($($a,)*) = self;
                msg_send_fn(obj_ptr, sel $(, $a)*)
            }
        }
    );
}

message_args_impl!();
message_args_impl!(a: A);
message_args_impl!(a: A, b: B);
message_args_impl!(a: A, b: B, c: C);
message_args_impl!(a: A, b: B, c: C, d: D);
message_args_impl!(a: A, b: B, c: C, d: D, e: E);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L);
