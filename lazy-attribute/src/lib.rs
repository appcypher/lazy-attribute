#![warn(missing_docs)]
#![doc = include_str!("lib.md")]

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use lazy_attribute_core::lazy;

//--------------------------------------------------------------------------------------------------
// Re-export Modules
//--------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod __internal {
    #[cfg(feature = "async")]
    pub use async_once_cell;
    pub use once_cell;
}

//--------------------------------------------------------------------------------------------------
// Test
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    #[test]
    fn lazy() {
        let t = trybuild::TestCases::new();

        t.pass("test/01-correct-func.rs");

        #[cfg(feature = "async")]
        t.pass("test/02-correct-async-func.rs");

        t.compile_fail("test/03-unsupported-args.rs");
    }
}