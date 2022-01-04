use anyhow::Result;

#[derive(Debug, Clone)]
pub enum SupportedOps {
    Blur(Blur),
    Convert(Convert),
    Crop(Crop),
    Flip(Flip),
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

    pub fn flip(arg1: &str) -> Result<Self> {
        let flip = Flip {
            orientation: arg1.to_string(),
        };
        Ok(SupportedOps::Flip(flip))
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
