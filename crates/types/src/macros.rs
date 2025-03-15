macro_rules! new_type {
    (
        $(#[$meta:meta])*
        struct $name:ident { $($field_name:ident: $field_type:ty),* $(,)* }
    ) => {
        use serde::Serialize;
        $(#[$meta])*
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
        pub struct $name {
            $(pub $field_name: $field_type),*
        }
        impl SerdeType for $name {
            fn se(&self) -> Vec<u8> {
                let mut buf = Vec::new();
                let val: Self = self.clone();
                val.serialize(&mut rmp_serde::Serializer::new(&mut buf)).unwrap();
                buf
            }
            fn de(ser: Vec<u8>) -> Self {
                let val = rmp_serde::from_slice(ser.as_slice()).unwrap();
                val
            }
        }
    };
}
