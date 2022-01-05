#[derive(Debug, Clone)]
pub enum SupportedOps {
    Blur(crate::process::blur::OperationArg),
    Convert(crate::process::convert::OperationArg),
    Crop(crate::process::crop::OperationArg),
    Flip(crate::process::flip::OperationArg),
    Rotate(crate::process::rotate::OperationArg),
    Tile(crate::process::tile::OperationArg),
}

impl SupportedOps {
    pub fn is_convert(&self) -> bool {
        matches!(self, SupportedOps::Convert(_))
    }
}

#[macro_export]
/// Define an image processing operation.
macro_rules! define_operation {
    ($(#[$outer:meta])* $name:ident $(($image:ident $(, $path:ident)?))? $(, $arg_name:ident : $arg_type:ty)*, $code:block) => {
        $(#[$outer])*
        pub fn execute($($($path: &str, )?)? operation_image: &DynamicImage, operation_args: OperationArg) -> Result<Option<DynamicImage>> {
            let OperationArg($($arg_name),*) = operation_args;
            $(
                let $image = operation_image;
            )?
            $code
        }

        pub fn to_operation (iter: &mut std::slice::Iter<&str>) -> Result<OperationArg> {
            $(
                let argument = iter.next();
                if argument == None {
                    return Err(anyhow!("Missing argument `{}` for `{}` operation", stringify!($arg_name), stringify!($name)));
                }
                let $arg_name: $arg_type = std::str::FromStr::from_str(argument.unwrap())?;
            )*
            Ok(OperationArg(
                $(
                    $arg_name
                ),*
            ))
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct OperationArg($(pub $arg_type),*);
    };
}
