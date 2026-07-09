#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub struct TestLib {
    __library: ::libloading::Library,
    pub foo: Result<
        unsafe extern "C" fn(x: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub foo_float: Result<unsafe extern "C" fn(x: f32) -> f32, ::libloading::Error>,
}
impl TestLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let foo = __library.get(b"_Z3fooIiET_S0_\0").map(|sym| *sym);
        let foo_float = __library.get(b"_Z3fooIfET_S0_\0").map(|sym| *sym);
        Ok(TestLib {
            __library,
            foo,
            foo_float,
        })
    }
    pub unsafe fn foo(&self, x: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        (self.foo.as_ref().expect("Expected function, got error."))(x)
    }
    pub unsafe fn foo_float(&self, x: f32) -> f32 {
        (self.foo_float.as_ref().expect("Expected function, got error."))(x)
    }
}
