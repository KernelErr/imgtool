use anyhow::Result;

#[derive(Debug, Clone)]
pub enum SupportedOps {
    Blur(Blur),
    Convert(Convert),
    Crop(Crop),
    Flip(crate::process::flip::OperationArg),
    Rotate(Rotate),
}

impl SupportedOps {
    pub fn is_convert(&self) -> bool {
        matches!(self, SupportedOps::Convert(_))
    }

    pub fn blur(arg1: &str) -> Result<Self> {
        let blur = Blur {
            sigma: arg1.parse::<f32>()?,
        };
        Ok(SupportedOps::Blur(blur))
    }

    pub fn convert(arg1: &str, arg2: &str) -> Result<Self> {
        let convert = Convert {
            format: arg1.to_string(),
            delete: arg2.parse::<bool>()?,
        };
        Ok(SupportedOps::Convert(convert))
    }

    pub fn crop(arg1: &str, arg2: &str, arg3: &str, arg4: &str) -> Result<Self> {
        let crop = Crop {
            x: arg1.parse::<u32>()?,
            y: arg2.parse::<u32>()?,
            width: arg3.parse::<u32>()?,
            height: arg4.parse::<u32>()?,
        };
        Ok(SupportedOps::Crop(crop))
    }

    pub fn rotate(arg1: &str) -> Result<Self> {
        let rotate = Rotate {
            angle: arg1.parse::<i32>()?,
        };
        Ok(SupportedOps::Rotate(rotate))
    }
}

#[derive(Debug, Clone)]
pub struct Blur {
    pub sigma: f32,
}

#[derive(Debug, Clone)]
pub struct Convert {
    pub format: String,
    pub delete: bool,
}

#[derive(Debug, Clone)]
pub struct Crop {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct Flip {
    pub orientation: String,
}

#[derive(Debug, Clone)]
pub struct Rotate {
    pub angle: i32,
}

#[macro_export]
macro_rules! define_operation {
    ($name:ident, $image:ident $(, $arg_name:ident : $arg_type:ty)*, $code:block) => {
        pub fn execute(operation_image: &Image, operation_args: OperationArg) -> Result<Option<DynamicImage>> {
            let OperationArg($($arg_name),*) = operation_args;
            let $image = operation_image;
            $code
        }

        pub fn to_operation (iter: &mut std::slice::Iter<String>) -> Result<OperationArg> {
            $(
                let argument = iter.next();
                if argument == None {
                    return Err(anyhow!("Missing argument `{}` for `{}` operation", stringify!($arg_name), stringify!($name)));
                }
                let $arg_name: $arg_type = FromStr::from_str(argument.unwrap().as_str()).unwrap();
            )*
            Ok(OperationArg(
                $(
                    $arg_name
                ),*
            ))
        }

        #[derive(Debug, Clone)]
        pub struct OperationArg($(pub $arg_type),*);
    };
}
