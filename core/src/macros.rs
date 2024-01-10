#[allow(unused_macros)]
macro_rules! code_enum {
    ($($(#[$($type_meta:meta)*])* $type:ident: $repr_type:ty { $($(#[$($var_meta:meta)*])* $var:ident = $val:literal => $str:literal $(| $strs:literal)*,)* })*) => {
        $(
            $(#[$($type_meta)*])*
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            #[repr($repr_type)]
            pub enum $type {
                $($(#[$($var_meta)*])* $var = $val,)*
            }

            const_assert_eq!(size_of::<$type>(), size_of::<$repr_type>());

            impl From<u8> for $type {
                fn from(raw: u8) -> Self {
                    unsafe { transmute(raw) }
                }
            }

            impl From<$type> for u8 {
                fn from(key: $type) -> Self {
                    key as _
                }
            }

            #[cfg(feature = "fromstr")]
            impl core::str::FromStr for $type {
                type Err = $crate::Unknown;

                fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
                    #[cfg(all(feature = "phf", not(feature = "unicase")))]
                    static MAP: phf::Map<&'static str, $type> = phf::phf_map! {
                        $(
                            $str => $type::$var,
                            $($strs => $type::$var,)*
                        )*
                    };

                    #[cfg(all(feature = "phf", feature = "unicase"))]
                    static MAP: phf::Map<unicase::UniCase<&'static str>, $type> = phf::phf_map! {
                        $(
                            UniCase::ascii($str) => $type::$var,
                            $(UniCase::ascii($strs) => $type::$var,)*
                        )*
                    };

                    #[cfg(all(feature = "phf", feature = "unicase"))]
                    let s = &unicase::UniCase::ascii(s);

                    #[cfg(feature = "phf")]
                    {
                        MAP.get(s).cloned().ok_or($crate::Unknown)
                    }

                    #[cfg(not(feature = "phf"))]
                    Ok(match s {
                        $($str $(| $strs)* => $type::$var,)*
                        _ => return Err($crate::Unknown),
                    })
                }
            }

            #[cfg(feature = "display")]
            impl $type {
                /// List of all enum variants
                pub const VARIANTS: &'static [$type] = &[
                    $($type::$var,)*
                ];
            }

            #[cfg(feature = "display")]
            impl AsRef<str> for $type {
                fn as_ref(&self) -> &str {
                    match self {
                        $($type::$var => $str,)*
                    }
                }
            }

            #[cfg(feature = "display")]
            impl core::fmt::Display for $type {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    f.write_str(self.as_ref())
                }
            }
        )*
    };
}

#[allow(unused_macros)]
macro_rules! serde_num {
    ($($type:ty: $rtype:tt, $expect:literal;)*) => {
        $(
            #[cfg(feature = "serde")]
            impl serde::Serialize for $type {
                fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serde_num!(@ser $rtype, serializer, <$rtype>::from(*self))
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> serde::Deserialize<'de> for $type {
                fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct Visitor;

                    impl<'de> serde::de::Visitor<'de> for Visitor {
                        type Value = $type;

                        fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                            f.write_str($expect)
                        }

                        serde_num! {
                            @visit {
                                $rtype, u8: visit_u8;
                                $rtype, i8: visit_i8;
                                $rtype, u16: visit_u16;
                                $rtype, i16: visit_i16;
                                $rtype, u32: visit_u32;
                                $rtype, i32: visit_i32;
                                $rtype, u64: visit_u64;
                                $rtype, i64: visit_i64;
                            }
                        }
                    }

                    serde_num!(@de $rtype, deserializer, Visitor)
                }
            }
        )*
    };

    (@ser u8, $serializer:ident, $value:expr) => {
        $serializer.serialize_u8($value)
    };

    (@ser u16, $serializer:ident, $value:expr) => {
        $serializer.serialize_u16($value)
    };

    (@ser u32, $serializer:ident, $value:expr) => {
        $serializer.serialize_u32($value)
    };

    (@de u8, $deserializer:ident, $visitor:ident) => {
        $deserializer.deserialize_u8($visitor)
    };

    (@de u16, $deserializer:ident, $visitor:ident) => {
        $deserializer.deserialize_u16($visitor)
    };

    (@de u32, $deserializer:ident, $visitor:ident) => {
        $deserializer.deserialize_u32($visitor)
    };

    (@visit { $($rtype:ty, $type:ty: $name:ident;)* }) => {
        $(
            fn $name<E>(self, value: $type) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Self::Value::safe_from(value as $rtype)
                    .ok_or_else(|| serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(value as _), &self))
            }
        )*
    }
}

#[allow(unused_macros)]
macro_rules! raw_ref {
    ($($type:ty;)*) => {
        $(
            impl AsRef<[u8]> for $type {
                fn as_ref(&self) -> &[u8] {
                    unsafe {
                        core::slice::from_raw_parts(
                            self as *const _ as *const _,
                            core::mem::size_of::<Self>(),
                        )
                    }
                }
            }

            impl AsMut<[u8]> for $type {
                fn as_mut(&mut self) -> &mut [u8] {
                    unsafe {
                        core::slice::from_raw_parts_mut(
                            self as *mut _ as *mut _,
                            core::mem::size_of::<Self>(),
                        )
                    }
                }
            }
        )*
    };
}

macro_rules! deref_impl {
    ($($type:ident $(<$($param:ident),*>)* => $field:ident: $field_type:ty,)*) => {
        $(
            impl $(<$($param),*>)* core::ops::Deref for $type $(<$($param),*>)* {
                type Target = $field_type;

                fn deref(&self) -> &Self::Target {
                    &self.$field
                }
            }

            impl $(<$($param),*>)* core::ops::DerefMut for $type $(<$($param),*>)* {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.$field
                }
            }
        )*
    };
}
