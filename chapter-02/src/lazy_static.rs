//macro_rules! __lazy_static_internal {
//    ($(#[$attr:meta])* ($($vis:tt)*) static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
//        __lazy_static_internal!(@MAKE TY, $(#[$attr])*, ($($vis)*), $N);
//        __lazy_static_internal!(@TAIL, $N : $T = $e);
//    };
//    (@TAIL, $N:ident : $T:ty = $e:expr) => {
//        impl $crate::__Deref for $N {
//            type Target = $T;
//
//            fn deref(&self) -> &Self::Target {
//                fn __static_ref_initialize() -> $T { $e }
//
//                fn __stability() -> &'static $T {
//                    __lazy_static_create!(LAZY, $T);
//                    LAZY.get(__static_ref_initialize)
//                }
//                __stability()
//            }
//        }
//
//        impl $crate::LazyStatic for $N {
//            fn initialize(lazy: &Self) {
//                let _ = &**lazy;
//            }
//        }
//    };
//    (@MAKE TY, $(#[$attr:meta])*, ($($vis:tt)*), $N:ident) => {
//        $(#[$attr])*
//        $($vis)* struct $N {__private_field: ()}
//        $($vis)* static $N: $N = $N {__private_field: ()};
//    };
//}
//
//macro_rules! __lazy_static_create {
//    ($NAME:ident, $T:ty) => {
//        static $NAME: $crate::lazy::Lazy<$T> = $crate::lazy::Lazy::INIT;
//    };
//}
//
//#[macro_export]
//macro_rules! lazy_static {
//    ($(#[$attr:meta])* static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
//        __lazy_static_internal!($(#[$attr])* () static ref $N : $T = $e; $($t)*);
//    };
//}
//
//lazy_static! {
//    static ref EXAMPLE: u8 = 42;
//}
//
//pub struct EXO {
//    __private_field: (),
//}
//
//pub static EXO: EXO = EXO {
//    __private_field: (),
//};
