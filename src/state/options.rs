pub trait UciOption {
    fn uci_type() -> &'static str;
}

impl UciOption for u8 { fn uci_type() -> &'static str { "spin" } }
impl UciOption for u16 { fn uci_type() -> &'static str { "spin" } }
impl UciOption for bool { fn uci_type() -> &'static str { "check" } }
impl UciOption for String { fn uci_type() -> &'static str { "string" } }

macro_rules! options {
    (@format_option $name:literal, $type:ty, $value:expr, $min:expr, $max:expr) => {
        format!("option name {} type {} default {} min {} max {}",
            $name,
            <$type as UciOption>::uci_type(),
            $value,
            $min,
            $max
        )
    };

    (@format_option $name:literal, $type:ty, $value:expr,) => {
        format!("option name {} type {} default {}",
            $name,
            <$type as UciOption>::uci_type(),
            $value
        )
    };

    (
        $( $field:ident : $type:ty = $default:expr => $uci_name:literal $( [$min:expr, $max:expr] )? ),* $(,)?
    ) => {
        #[derive(Debug, Clone)]
        pub struct Options {
            $( pub $field: $type, )*
        }

        impl Options {
            pub fn format_uci_options(&self) -> Vec<String> {
                vec![$(
                    options!(@format_option $uci_name, $type, self.$field, $($min, $max)?),
                )*]
            }

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
    threads: u16 = 1 => "Threads" [1, 256],
    debug: bool = false => "Debug",
    output_reasoning: bool = false => "OutputReasoning",
    apimodel: String = String::from("openai/gpt-oss-20b") => "APIModel",
    apibaseurl: String = String::from("<unset>") => "APIBaseURL",
    apikey: String = String::from("<unset>") => "APIKey",
    apimaxtries: u8 = 3 => "APIMaxTries",
    fenasmd: bool = false => "FenAsMarkdown",
    additional_instructions: String = String::new() => "AdditionalInstructions",
    additional_instructions_file: String = String::new() => "AdditionalInstructionsFile",
}
