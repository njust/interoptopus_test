use std::ffi::{CString};
use std::io::Write;
use interoptopus::{callback, ffi_service, ffi_type, ffi_service_ctor, ffi_service_method};
use interoptopus::patterns::slice::{ FFISliceMut};
use interoptopus::patterns::string::AsciiPointer;

mod result;
use crate::result::{FFIError, Error};

#[ffi_type(opaque)]
#[derive(Default)]
pub struct CounterService {
    count: i32,
    allocator: ByteAllocator,
}


#[ffi_type]
#[repr(C)]
pub struct Test {
    pub count: i32,
    pub msg: FFISliceMut<'static, u8>,
}

#[ffi_type]
#[repr(C)]
pub struct NestedAsciiPointer<'a> {
    pub msg: AsciiPointer<'a>,
}

callback!(ByteAllocator(bytes: u32) -> FFISliceMut<'static, u8>);
callback!(TestAllocator(count: u32) -> FFISliceMut<'static, Test>);

#[ffi_service(error = "FFIError", prefix = "counter_service_")]
impl CounterService {
    #[ffi_service_ctor]
    pub fn new_with(value: i32, allocator: ByteAllocator) -> Result<Self, Error> {
        Ok(Self {
            count: value,
            allocator
        })
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn inc(&mut self) -> i32 {
        self.count += 1;
        self.count
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn dec(&mut self) -> i32 {
        self.count -= 1;
        self.count
    }

    #[ffi_service_method(on_panic = "undefined_behavior")]
    pub fn as_string(&mut self) -> AsciiPointer {
        self.ffi_str_pointer(format!("Count: {}", self.count).as_str())
    }

    #[ffi_service_method(on_panic = "undefined_behavior")]
    pub fn nested_string(&mut self) -> NestedAsciiPointer {
        let msg = self.ffi_str_pointer(format!("Count: {}", self.count).as_str());
        NestedAsciiPointer {
            msg
        }
    }

    #[ffi_service_method(on_panic = "undefined_behavior")]
    pub fn get_data(&self, mut data: FFISliceMut<u8>) {
        let mut buffer = data.as_slice_mut();
        println!("Got buffer with size: {}", buffer.len());
        buffer.write(b"Hello from rust").unwrap();
    }

    #[ffi_service_method(on_panic = "undefined_behavior")]
    pub fn get_test_data(&self, a: TestAllocator) -> FFISliceMut<Test> {
        let mut data = a.call(100);
        for (i, d) in data.as_slice_mut().iter_mut().enumerate() {
            d.count = i as i32 *2;
            d.msg = self.ffi_str_data(format!("count: {}", i).as_str());
        }
        data
    }
}

impl CounterService {
    fn ffi_str_pointer(&self, s: &str) -> AsciiPointer {
        let c_str = CString::new(s).unwrap();
        let data = c_str.as_bytes_with_nul();
        let mut buffer = self.allocator.call(data.len() as u32);
        buffer.as_slice_mut().copy_from_slice(data);
        AsciiPointer::from_slice_with_nul(buffer.as_slice()).unwrap()
    }

    fn ffi_str_data<'a>(&self, s: &str) -> FFISliceMut<'a, u8> {
        let c_str = CString::new(s).unwrap();
        let data = c_str.as_bytes_with_nul();
        let mut buffer = self.allocator.call(data.len() as u32);
        buffer.as_slice_mut().copy_from_slice(data);
        buffer
    }
}

interoptopus::inventory!(my_inventory, [], [], [], [CounterService]);