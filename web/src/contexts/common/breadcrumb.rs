use db::models::{BlockModel, EpochModel, ValidatorModel};
use serde::Serialize;

#[derive(Serialize)]
pub struct Breadcrumb {
    pub parts: Vec<BreadcrumbPart>,
}

impl From<Vec<BreadcrumbPart>> for Breadcrumb {
    fn from(parts: Vec<BreadcrumbPart>) -> Self {
        Breadcrumb { parts }
    }
}

#[derive(Serialize)]
pub struct BreadcrumbPart {
    pub text: String,
    pub link: Option<String>,
    pub icon: Option<String>,
}

impl BreadcrumbPart {
    pub fn from_text(text: &str) -> BreadcrumbPart {
        BreadcrumbPart {
            text: text.into(),
            link: None,
            icon: None,
        }
    }

    pub fn from_text_with_icon(text: &str, icon: &str) -> BreadcrumbPart {
        BreadcrumbPart {
            text: text.into(),
            link: None,
            icon: Some(icon.into()),
        }
    }

    pub fn from_link(text: &str, link: &str, icon: &str) -> BreadcrumbPart {
        BreadcrumbPart {
            text: text.into(),
            link: Some(link.into()),
            icon: Some(icon.into()),
        }
    }
}

impl From<EpochModel> for BreadcrumbPart {
    fn from(model: EpochModel) -> Self {
        Self::from_text(format!("Epoch {} details", model.epoch).as_str())
    }
}

impl From<BlockModel> for BreadcrumbPart {
    fn from(model: BlockModel) -> Self {
        Self::from_text(format!("Block {} details", model.slot).as_str())
    }
}

impl From<ValidatorModel> for BreadcrumbPart {
    fn from(model: ValidatorModel) -> Self {
        Self::from_text(format!("Validator {} details", model.validator_index).as_str())
    }
}
