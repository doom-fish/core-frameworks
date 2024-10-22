use std::{ffi::c_void, ptr};

#[repr(transparent)]
pub struct TrampolineRefcon(*mut c_void);

#[repr(transparent)]
pub struct TrampolineLeftCallback<Ret = (), Param = TrampolineRefcon>(
    extern "C" fn(TrampolineRefcon, Param) -> Ret,
);
#[repr(transparent)]
pub struct TrampolineRightCallback<Ret = (), Param = TrampolineRefcon>(
    extern "C" fn(Param, TrampolineRefcon) -> Ret,
);

pub fn create_left_trampoline<'a, Ret, Param, F: 'a + Send + FnOnce(Param) -> Ret>(
    wrapped_closure: F,
) -> (TrampolineLeftCallback<Ret, Param>, TrampolineRefcon) {
    pub extern "C" fn caller<Ret, Param, F>(closure_ptr: TrampolineRefcon, param: Param) -> Ret
    where
        F: FnOnce(Param) -> Ret,
    {
        println!("caller");
        unsafe {
            let closure: F = ptr::read(closure_ptr.0.cast());
            closure(param)
        }
    }
    (
        TrampolineLeftCallback(caller::<Ret, Param, F>),
        TrampolineRefcon(Box::into_raw(Box::new(wrapped_closure)).cast()),
    )
}

pub fn create_right_trampoline<'a, Ret, Param, F: 'a + Send + FnOnce(Param) -> Ret>(
    wrapped_closure: F,
) -> (TrampolineRightCallback<Ret, Param>, TrampolineRefcon) {
    pub extern "C" fn caller<Ret, Param, F>(param: Param, closure_ptr: TrampolineRefcon) -> Ret
    where
        F: FnOnce(Param) -> Ret,
    {
        println!("caller");
        unsafe {
            let closure: F = ptr::read(closure_ptr.0.cast());
            closure(param)
        }
    }
    (
        TrampolineRightCallback(caller::<Ret, Param, F>),
        TrampolineRefcon(Box::into_raw(Box::new(wrapped_closure)).cast()),
    )
}
