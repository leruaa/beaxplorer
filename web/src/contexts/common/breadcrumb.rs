use db::models::{BlockModel, EpochModel};
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
}

impl BreadcrumbPart {
    pub fn from_text(text: &str) -> BreadcrumbPart {
        BreadcrumbPart {
            text: text.into(),
            link: None,
        }
    }

    pub fn from_link(text: &str, link: &str) -> BreadcrumbPart {
        BreadcrumbPart {
            text: text.into(),
            link: Some(link.into()),
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
