#[derive(Debug)]
pub struct CWMODE_CUR {
    pub mode: u128,
}

impl<'de> serde::Deserialize<'de> for CWMODE_CUR {
    fn deserialize<D>(deserializer: D) -> serde::export::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum CWMODE_CURField {
            field0,
            ignore,
        }
        struct CWMODE_CURFieldVisitor;
        impl<'de> serde::de::Visitor<'de> for CWMODE_CURFieldVisitor {
            type Value = CWMODE_CURField;
            fn expecting(
                &self,
                formatter: &mut serde::export::Formatter,
            ) -> serde::export::fmt::Result {
                serde::export::Formatter::write_str(formatter, "field identifier")
            }
            fn visit_u64<E>(self, value: u64) -> serde::export::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    0u64 => serde::export::Ok(CWMODE_CURField::field0),
                    _ => serde::export::Err(::serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(value),
                        &"field index 0 <= i < 1",
                    )),
                }
            }
            fn visit_u128<E>(self, value: u128) -> serde::export::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    0u128 => serde::export::Ok(CWMODE_CURField::field0),
                    _ => serde::export::Err(::serde::de::Error::invalid_value(
                        serde::de::Unexpected::Other("u128"),
                        &"field index 0 <= i < 1",
                    )),
                }
            }
            fn visit_str<E>(self, value: &str) -> serde::export::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "mode" => serde::export::Ok(CWMODE_CURField::field0),
                    _ => serde::export::Ok(CWMODE_CURField::ignore),
                }
            }
            fn visit_bytes<E>(self, value: &[u8]) -> serde::export::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    b"mode" => serde::export::Ok(CWMODE_CURField::field0),
                    _ => serde::export::Ok(CWMODE_CURField::ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for CWMODE_CURField {
            #[inline]
            fn deserialize<D>(deserializer: D) -> serde::export::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(deserializer, CWMODE_CURFieldVisitor)
            }
        }
        struct CWMODE_CURVisitor<'de> {
            marker: serde::export::PhantomData<CWMODE_CUR>,
            lifetime: serde::export::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for CWMODE_CURVisitor<'de> {
            type Value = CWMODE_CUR;
            fn expecting(
                &self,
                formatter: &mut serde::export::Formatter,
            ) -> serde::export::fmt::Result {
                serde::export::Formatter::write_str(formatter, "struct CWMODE_CUR")
            }
            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> serde::export::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let field0 = match match serde::de::SeqAccess::next_element::<u128>(&mut seq) {
                    serde::export::Ok(val) => val,
                    serde::export::Err(err) => {
                        return serde::export::Err(err);
                    }
                } {
                    serde::export::Some(value) => value,
                    serde::export::None => {
                        return serde::export::Err(::serde::de::Error::invalid_length(
                            0usize,
                            &"struct CWMODE_CUR with 1 elements",
                        ));
                    }
                };
                serde::export::Ok(CWMODE_CUR { mode: field0 })
            }
            #[inline]
            fn visit_map<A>(self, mut map: A) -> serde::export::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut field0: serde::export::Option<u128> = serde::export::None;
                while let serde::export::Some(key) =
                    match serde::de::MapAccess::next_key::<CWMODE_CURField>(&mut map) {
                        serde::export::Ok(val) => val,
                        serde::export::Err(err) => {
                            return serde::export::Err(err);
                        }
                    }
                {
                    match key {
                        CWMODE_CURField::field0 => {
                            if serde::export::Option::is_some(&field0) {
                                return serde::export::Err(
                                    <A::Error as serde::de::Error>::duplicate_field("mode"),
                                );
                            }
                            field0 = serde::export::Some(
                                match serde::de::MapAccess::next_value::<u128>(&mut map) {
                                    serde::export::Ok(val) => val,
                                    serde::export::Err(err) => {
                                        return serde::export::Err(err);
                                    }
                                },
                            );
                        }
                        _ => {
                            let _ = match serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(
                                &mut map,
                            ) {
                                serde::export::Ok(val) => val,
                                serde::export::Err(err) => {
                                    return serde::export::Err(err);
                                }
                            };
                        }
                    }
                }
                let field0 = match field0 {
                    serde::export::Some(field0) => field0,
                    serde::export::None => match serde::private::de::missing_field("mode") {
                        serde::export::Ok(val) => val,
                        serde::export::Err(err) => {
                            return serde::export::Err(err);
                        }
                    },
                };
                serde::export::Ok(CWMODE_CUR { mode: field0 })
            }
        }
        const FIELDS: &'static [&'static str] = &["mode"];
        serde::Deserializer::deserialize_struct(
            deserializer,
            "CWMODE",
            FIELDS,
            CWMODE_CURVisitor {
                marker: serde::export::PhantomData::<CWMODE_CUR>,
                lifetime: serde::export::PhantomData,
            },
        )
    }
}

fn main() {
    let s = "+CWMODE_CUR: 1";

    let r = serde_at::from_str::<CWMODE_CUR>(s);

    println!("{:?}", r);
}
