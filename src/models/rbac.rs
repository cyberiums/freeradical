use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TenantRole {
    Owner,
    Admin,
    Editor,
    Author,
    Viewer,
}

impl TenantRole {
    pub fn from_str(role: &str) -> Option<Self> {
        match role.to_lowercase().as_str() {
            "owner" => Some(TenantRole::Owner),
            "admin" => Some(TenantRole::Admin),
            "editor" => Some(TenantRole::Editor),
            "author" => Some(TenantRole::Author),
            "viewer" => Some(TenantRole::Viewer),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TenantRole::Owner => "owner".to_string(),
            TenantRole::Admin => "admin".to_string(),
            TenantRole::Editor => "editor".to_string(),
            TenantRole::Author => "author".to_string(),
            TenantRole::Viewer => "viewer".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    ManageSettings,
    ManageMembers,
    ManageBilling,
    PublishContent,
    EditContent,
    DeleteContent,
    ViewAnalytics,
    ViewContent,
}

pub fn get_permissions(role: &TenantRole) -> HashSet<Permission> {
    let mut perms = HashSet::new();

    match role {
        TenantRole::Owner => {
            perms.insert(Permission::ManageSettings);
            perms.insert(Permission::ManageMembers);
            perms.insert(Permission::ManageBilling);
            perms.insert(Permission::PublishContent);
            perms.insert(Permission::EditContent);
            perms.insert(Permission::DeleteContent);
            perms.insert(Permission::ViewAnalytics);
            perms.insert(Permission::ViewContent);
        },
        TenantRole::Admin => {
            perms.insert(Permission::ManageSettings);
            perms.insert(Permission::ManageMembers);
            // No billing
            perms.insert(Permission::PublishContent);
            perms.insert(Permission::EditContent);
            perms.insert(Permission::DeleteContent);
            perms.insert(Permission::ViewAnalytics);
            perms.insert(Permission::ViewContent);
        },
        TenantRole::Editor => {
            perms.insert(Permission::PublishContent);
            perms.insert(Permission::EditContent);
            perms.insert(Permission::DeleteContent);
            perms.insert(Permission::ViewAnalytics);
            perms.insert(Permission::ViewContent);
        },
        TenantRole::Author => {
            // Can edit/publish own content typically, but for simplicity here just Edit
            perms.insert(Permission::EditContent); 
            perms.insert(Permission::ViewContent);
        },
        TenantRole::Viewer => {
            perms.insert(Permission::ViewContent);
        },
    }

    perms
}

pub fn has_permission(role_str: &str, required: Permission) -> bool {
    if let Some(role) = TenantRole::from_str(role_str) {
        let perms = get_permissions(&role);
        perms.contains(&required)
    } else {
        false
    }
}
