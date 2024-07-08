use std::fmt::Display;

use anyhow::{bail, Error};
use serde::{Deserialize, Serialize};

use crate::repositories::repository::{
    APTRepository, APTRepositoryFileType, APTRepositoryPackageType,
};

use proxmox_schema::api;

#[api(
    properties: {
        handle: {
            description: "Handle referencing a standard repository.",
            type: String,
        },
    },
)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
/// Reference to a standard repository and configuration status.
pub struct APTStandardRepository {
    /// Handle referencing a standard repository.
    pub handle: APTRepositoryHandle,

    /// Configuration status of the associated repository, where `None` means
    /// not configured, and `Some(bool)` indicates enabled or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,

    /// Display name of the repository.
    pub name: String,

    /// Description of the repository.
    pub description: String,
}

#[api]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
/// Handles for Proxmox repositories.
pub enum APTRepositoryHandle {
    /// The enterprise repository for production use.
    Enterprise,
    /// The repository that can be used without subscription.
    NoSubscription,
    /// The test repository.
    Test,
    /// Ceph Quincy enterprise repository.
    CephQuincyEnterprise,
    /// Ceph Quincy no-subscription repository.
    CephQuincyNoSubscription,
    /// Ceph Quincy test repository.
    CephQuincyTest,
    // TODO: Add separate enum for ceph releases and use something like
    // `CephTest(CephReleaseCodename),` once the API macro supports it.
    /// Ceph Reef enterprise repository.
    CephReefEnterprise,
    /// Ceph Reef no-subscription repository.
    CephReefNoSubscription,
    /// Ceph Reef test repository.
    CephReefTest,
}

impl From<APTRepositoryHandle> for APTStandardRepository {
    fn from(handle: APTRepositoryHandle) -> Self {
        APTStandardRepository {
            handle,
            status: None,
            name: handle.name(),
            description: handle.description(),
        }
    }
}

impl TryFrom<&str> for APTRepositoryHandle {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Error> {
        match string {
            "enterprise" => Ok(APTRepositoryHandle::Enterprise),
            "no-subscription" => Ok(APTRepositoryHandle::NoSubscription),
            "test" => Ok(APTRepositoryHandle::Test),
            "ceph-quincy-enterprise" => Ok(APTRepositoryHandle::CephQuincyEnterprise),
            "ceph-quincy-no-subscription" => Ok(APTRepositoryHandle::CephQuincyNoSubscription),
            "ceph-quincy-test" => Ok(APTRepositoryHandle::CephQuincyTest),
            "ceph-reef-enterprise" => Ok(APTRepositoryHandle::CephReefEnterprise),
            "ceph-reef-no-subscription" => Ok(APTRepositoryHandle::CephReefNoSubscription),
            "ceph-reef-test" => Ok(APTRepositoryHandle::CephReefTest),
            _ => bail!("unknown repository handle '{}'", string),
        }
    }
}

impl Display for APTRepositoryHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            APTRepositoryHandle::Enterprise => write!(f, "enterprise"),
            APTRepositoryHandle::NoSubscription => write!(f, "no-subscription"),
            APTRepositoryHandle::Test => write!(f, "test"),
            APTRepositoryHandle::CephQuincyEnterprise => write!(f, "ceph-quincy-enterprise"),
            APTRepositoryHandle::CephQuincyNoSubscription => {
                write!(f, "ceph-quincy-no-subscription")
            }
            APTRepositoryHandle::CephQuincyTest => write!(f, "ceph-quincy-test"),
            APTRepositoryHandle::CephReefEnterprise => write!(f, "ceph-reef-enterprise"),
            APTRepositoryHandle::CephReefNoSubscription => write!(f, "ceph-reef-no-subscription"),
            APTRepositoryHandle::CephReefTest => write!(f, "ceph-reef-test"),
        }
    }
}

pub trait APTRepositoryHandleImpl {
    /// Get the description for the repository.
    fn description(self) -> String;
    /// Get the display name of the repository.
    fn name(self) -> String;
    /// Get the standard file path for the repository referenced by the handle.
    fn path(self, product: &str) -> String;
    /// Get package type, possible URIs and the component associated with the handle.
    ///
    /// The first URI is the preferred one.
    fn info(self, product: &str) -> (APTRepositoryPackageType, Vec<String>, String);
    /// Get the standard repository referenced by the handle.
    ///
    /// An URI in the result is not '/'-terminated (under the assumption that no valid
    /// product name is).
    fn to_repository(self, product: &str, suite: &str) -> APTRepository;
}

impl APTRepositoryHandleImpl for APTRepositoryHandle {
    fn description(self) -> String {
        match self {
            APTRepositoryHandle::Enterprise => {
                "This is the default, stable, and recommended repository, available for all \
                Proxmox subscription users."
            }
            APTRepositoryHandle::NoSubscription => {
                "This is the recommended repository for testing and non-production use. \
                Its packages are not as heavily tested and validated as the production ready \
                enterprise repository. You don't need a subscription key to access this repository."
            }
            APTRepositoryHandle::Test => {
                "This repository contains the latest packages and is primarily used for test labs \
                and by developers to test new features."
            }
            APTRepositoryHandle::CephQuincyEnterprise => {
                "This repository holds the production-ready Proxmox Ceph Quincy packages."
            }
            APTRepositoryHandle::CephQuincyNoSubscription => {
                "This repository holds the Proxmox Ceph Quincy packages intended for \
                non-production use. The deprecated 'main' repository is an alias for this in \
                Proxmox VE 8."
            }
            APTRepositoryHandle::CephQuincyTest => {
                "This repository contains the Ceph Quincy packages before they are moved to the \
                main repository."
            }
            APTRepositoryHandle::CephReefEnterprise => {
                "This repository holds the production-ready Proxmox Ceph Reef packages."
            }
            APTRepositoryHandle::CephReefNoSubscription => {
                "This repository holds the Proxmox Ceph Reef packages intended for \
                non-production use."
            }
            APTRepositoryHandle::CephReefTest => {
                "This repository contains the Ceph Reef packages before they are moved to the \
                main repository."
            }
        }
        .to_string()
    }

    fn name(self) -> String {
        match self {
            APTRepositoryHandle::Enterprise => "Enterprise",
            APTRepositoryHandle::NoSubscription => "No-Subscription",
            APTRepositoryHandle::Test => "Test",
            APTRepositoryHandle::CephQuincyEnterprise => "Ceph Quincy Enterprise",
            APTRepositoryHandle::CephQuincyNoSubscription => "Ceph Quincy No-Subscription",
            APTRepositoryHandle::CephQuincyTest => "Ceph Quincy Test",
            APTRepositoryHandle::CephReefEnterprise => "Ceph Reef Enterprise",
            APTRepositoryHandle::CephReefNoSubscription => "Ceph Reef No-Subscription",
            APTRepositoryHandle::CephReefTest => "Ceph Reef Test",
        }
        .to_string()
    }

    fn path(self, product: &str) -> String {
        match self {
            APTRepositoryHandle::Enterprise => {
                format!("/etc/apt/sources.list.d/{}-enterprise.list", product)
            }
            APTRepositoryHandle::NoSubscription => "/etc/apt/sources.list".to_string(),
            APTRepositoryHandle::Test => "/etc/apt/sources.list".to_string(),
            APTRepositoryHandle::CephQuincyEnterprise
            | APTRepositoryHandle::CephQuincyNoSubscription
            | APTRepositoryHandle::CephQuincyTest
            | APTRepositoryHandle::CephReefEnterprise
            | APTRepositoryHandle::CephReefNoSubscription
            | APTRepositoryHandle::CephReefTest => "/etc/apt/sources.list.d/ceph.list".to_string(),
        }
    }

    fn info(self, product: &str) -> (APTRepositoryPackageType, Vec<String>, String) {
        match self {
            APTRepositoryHandle::Enterprise => (
                APTRepositoryPackageType::Deb,
                match product {
                    "pve" => vec![
                        "https://enterprise.proxmox.com/debian/pve".to_string(),
                        "https://enterprise.proxmox.com/debian".to_string(),
                    ],
                    _ => vec![format!("https://enterprise.proxmox.com/debian/{}", product)],
                },
                format!("{}-enterprise", product),
            ),
            APTRepositoryHandle::NoSubscription => (
                APTRepositoryPackageType::Deb,
                match product {
                    "pve" => vec![
                        "http://download.proxmox.com/debian/pve".to_string(),
                        "http://download.proxmox.com/debian".to_string(),
                    ],
                    _ => vec![format!("http://download.proxmox.com/debian/{}", product)],
                },
                format!("{}-no-subscription", product),
            ),
            APTRepositoryHandle::Test => (
                APTRepositoryPackageType::Deb,
                match product {
                    "pve" => vec![
                        "http://download.proxmox.com/debian/pve".to_string(),
                        "http://download.proxmox.com/debian".to_string(),
                    ],
                    _ => vec![format!("http://download.proxmox.com/debian/{}", product)],
                },
                format!("{}test", product),
            ),
            APTRepositoryHandle::CephQuincyEnterprise => (
                APTRepositoryPackageType::Deb,
                vec!["https://enterprise.proxmox.com/debian/ceph-quincy".to_string()],
                "enterprise".to_string(),
            ),
            APTRepositoryHandle::CephQuincyNoSubscription => (
                APTRepositoryPackageType::Deb,
                vec!["http://download.proxmox.com/debian/ceph-quincy".to_string()],
                "no-subscription".to_string(),
            ),
            APTRepositoryHandle::CephQuincyTest => (
                APTRepositoryPackageType::Deb,
                vec!["http://download.proxmox.com/debian/ceph-quincy".to_string()],
                "test".to_string(),
            ),
            APTRepositoryHandle::CephReefEnterprise => (
                APTRepositoryPackageType::Deb,
                vec!["https://enterprise.proxmox.com/debian/ceph-reef".to_string()],
                "enterprise".to_string(),
            ),
            APTRepositoryHandle::CephReefNoSubscription => (
                APTRepositoryPackageType::Deb,
                vec!["http://download.proxmox.com/debian/ceph-reef".to_string()],
                "no-subscription".to_string(),
            ),
            APTRepositoryHandle::CephReefTest => (
                APTRepositoryPackageType::Deb,
                vec!["http://download.proxmox.com/debian/ceph-reef".to_string()],
                "test".to_string(),
            ),
        }
    }

    fn to_repository(self, product: &str, suite: &str) -> APTRepository {
        let (package_type, uris, component) = self.info(product);

        APTRepository {
            types: vec![package_type],
            uris: vec![uris.into_iter().next().unwrap()],
            suites: vec![suite.to_string()],
            components: vec![component],
            options: vec![],
            comment: String::new(),
            file_type: APTRepositoryFileType::List,
            enabled: true,
        }
    }
}
