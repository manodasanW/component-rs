#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clashing_extern_declarations,
    unused_variables,
    dead_code,
    clippy::all
)]

mod bindings;
use std::mem::*;
use std::sync::*;
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[implement(bindings::Class)]
struct Class(RwLock<i32>);

impl bindings::IClass_Impl for Class {
    fn Property(&self) -> Result<i32> {
        let reader = self.0.read().unwrap();
        Ok(*reader)
    }
    fn SetProperty(&self, value: i32) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        *writer = value;
        Ok(())
    }
    fn Make(&self, value :i32) -> Result<bindings::Class> {
        Ok(Class(RwLock::new(value)).into())
    }
}

#[implement(IActivationFactory)]
struct ClassFactory();

impl IActivationFactory_Impl for ClassFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Class(RwLock::new(0)).into())
    }
}

#[no_mangle]
unsafe extern "stdcall" fn DllGetActivationFactory(
    _name: ManuallyDrop<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    // TODO: check class name
    let factory: IActivationFactory = ClassFactory().into();
    *result = transmute(factory);
    S_OK
}
