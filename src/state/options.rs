macro_rules! options {
    (
        $( $field:ident : $type:ty = $default:expr => $uci_name:literal ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy)]
        pub struct Options {
            $( pub $field: $type, )*
        }

        impl Options {
            pub fn set_by_name_value(&mut self, name: &str, value: &str) -> Result<(), String> {
                match name {
                    $(
                        $uci_name => {
                            self.$field = value.parse().map_err(|e: <$type as std::str::FromStr>::Err| e.to_string())?;
                        }
                    )*
                    _ => return Err(format!("Unknown option: {}", name)),
                }
                Ok(())
            }
        }

        impl Default for Options {
            fn default() -> Self {
                Self {
                    $( $field: $default, )*
                }
            }
        }
    };
}

options! {
threads: u16 = 1 => "Threads",
}
