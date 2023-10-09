pub mod fs;
pub mod icons;
pub mod paths;
pub mod theme;

#[macro_export]
macro_rules! struct_with_funcs {
    ($(
        $param:ident: $ty:path
    ),+) => {
        paste::paste! {
            $(
                pub fn [< with_ $param >](mut self, $param: $ty) -> Self {
                    self.$param = $param;
                    self
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! struct_with_into_funcs {
    ($(
        $param:ident: $ty:path
    ),+) => {
        paste::paste! {
            $(
                pub fn [< with_ $param >](mut self, $param: impl Into<$ty>) -> Self {
                    self.$param = $param.into();
                    self
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! struct_with_some_funcs {
    ($(
        $param:ident: $ty:path
    ),+) => {
        paste::paste! {
            $(
                pub fn [< with_ $param >](mut self, $param: $ty) -> Self {
                    self.$param = Some($param);
                    self
                }
            )+
        }
    };
}
