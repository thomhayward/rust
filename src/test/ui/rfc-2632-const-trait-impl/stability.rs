#![allow(incomplete_features)]
#![feature(allow_internal_unstable)]
#![feature(const_add)]
#![feature(const_trait_impl)]
#![feature(staged_api)]

pub struct Int(i32);

#[rustc_const_unstable(feature = "const_add", issue = "none")]
impl const std::ops::Add for Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Int(self.0 + rhs.0)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_stable(feature = "rust1", since = "1.0.0")]
pub const fn foo() -> Int {
    Int(1i32) + Int(2i32)
    //~^ ERROR can only call other `const fn` within a `const fn`
}

// ok
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature = "bar", issue = "none")]
pub const fn bar() -> Int {
    Int(1i32) + Int(2i32)
}


fn main() {}
