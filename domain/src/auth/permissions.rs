use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum Permission {
    // User permissions
    #[sea_orm(string_value = "users:read")]
    UsersRead,
    #[sea_orm(string_value = "users:create")]
    UsersCreate,
    #[sea_orm(string_value = "users:update")]
    UsersUpdate,
    #[sea_orm(string_value = "users:delete")]
    UsersDelete,

    // Register permissions
    #[sea_orm(string_value = "registers:read")]
    RegistersRead,
    #[sea_orm(string_value = "registers:create")]
    RegistersCreate,
    #[sea_orm(string_value = "registers:update")]
    RegistersUpdate,
    #[sea_orm(string_value = "registers:delete")]
    RegistersDelete,

    // Official permissions
    #[sea_orm(string_value = "officials:read")]
    OfficialsRead,
    #[sea_orm(string_value = "officials:create")]
    OfficialsCreate,
    #[sea_orm(string_value = "officials:update")]
    OfficialsUpdate,
    #[sea_orm(string_value = "officials:delete")]
    OfficialsDelete,

    // Commission permissions
    #[sea_orm(string_value = "commissions:read")]
    CommissionsRead,
    #[sea_orm(string_value = "commissions:create")]
    CommissionsCreate,
    #[sea_orm(string_value = "commissions:update")]
    CommissionsUpdate,
    #[sea_orm(string_value = "commissions:delete")]
    CommissionsDelete,

    // Transport permissions
    #[sea_orm(string_value = "transports:read")]
    TransportsRead,
    #[sea_orm(string_value = "transports:create")]
    TransportsCreate,
    #[sea_orm(string_value = "transports:update")]
    TransportsUpdate,
    #[sea_orm(string_value = "transports:delete")]
    TransportsDelete,

    // Persona permissions
    #[sea_orm(string_value = "personas:read")]
    PersonasRead,
    #[sea_orm(string_value = "personas:create")]
    PersonasCreate,
    #[sea_orm(string_value = "personas:update")]
    PersonasUpdate,
    #[sea_orm(string_value = "personas:delete")]
    PersonasDelete,

    // Part permissions
    #[sea_orm(string_value = "parts:read")]
    PartsRead,
    #[sea_orm(string_value = "parts:create")]
    PartsCreate,
    #[sea_orm(string_value = "parts:update")]
    PartsUpdate,
    #[sea_orm(string_value = "parts:complete")]
    PartsComplete,

    // Seclusion permissions
    #[sea_orm(string_value = "seclusions:read")]
    SeclusionsRead,
    #[sea_orm(string_value = "seclusions:create")]
    SeclusionsCreate,
    #[sea_orm(string_value = "seclusions:update")]
    SeclusionsUpdate,
    #[sea_orm(string_value = "seclusions:add_visit")]
    SeclusionsAddVisit,

    // Admin permissions
    #[sea_orm(string_value = "admin:all")]
    AdminAll,
    #[sea_orm(string_value = "admin:users")]
    AdminUsers,
    #[sea_orm(string_value = "admin:roles")]
    AdminRoles,
    #[sea_orm(string_value = "admin:permissions")]
    AdminPermissions,

    // Lookup/Catalog permissions
    #[sea_orm(string_value = "lookups:read")]
    LookupsRead,
    #[sea_orm(string_value = "lookups:create")]
    LookupsCreate,
    #[sea_orm(string_value = "lookups:update")]
    LookupsUpdate,
}

impl Permission {
    /// Convierte el enum a string para almacenar en DB
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UsersRead => "users:read",
            Self::UsersCreate => "users:create",
            Self::UsersUpdate => "users:update",
            Self::UsersDelete => "users:delete",

            Self::RegistersRead => "registers:read",
            Self::RegistersCreate => "registers:create",
            Self::RegistersUpdate => "registers:update",
            Self::RegistersDelete => "registers:delete",

            Self::OfficialsRead => "officials:read",
            Self::OfficialsCreate => "officials:create",
            Self::OfficialsUpdate => "officials:update",
            Self::OfficialsDelete => "officials:delete",

            Self::CommissionsRead => "commissions:read",
            Self::CommissionsCreate => "commissions:create",
            Self::CommissionsUpdate => "commissions:update",
            Self::CommissionsDelete => "commissions:delete",

            Self::TransportsRead => "transports:read",
            Self::TransportsCreate => "transports:create",
            Self::TransportsUpdate => "transports:update",
            Self::TransportsDelete => "transports:delete",

            Self::PersonasRead => "personas:read",
            Self::PersonasCreate => "personas:create",
            Self::PersonasUpdate => "personas:update",
            Self::PersonasDelete => "personas:delete",

            Self::PartsRead => "parts:read",
            Self::PartsCreate => "parts:create",
            Self::PartsUpdate => "parts:update",
            Self::PartsComplete => "parts:complete",

            Self::SeclusionsRead => "seclusions:read",
            Self::SeclusionsCreate => "seclusions:create",
            Self::SeclusionsUpdate => "seclusions:update",
            Self::SeclusionsAddVisit => "seclusions:add_visit",

            Self::AdminAll => "admin:all",
            Self::AdminUsers => "admin:users",
            Self::AdminRoles => "admin:roles",
            Self::AdminPermissions => "admin:permissions",

            Self::LookupsRead => "lookups:read",
            Self::LookupsCreate => "lookups:create",
            Self::LookupsUpdate => "lookups:update",
        }
    }

    /// Convierte string de DB a enum
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "users:read" => Some(Self::UsersRead),
            "users:create" => Some(Self::UsersCreate),
            "users:update" => Some(Self::UsersUpdate),
            "users:delete" => Some(Self::UsersDelete),

            "registers:read" => Some(Self::RegistersRead),
            "registers:create" => Some(Self::RegistersCreate),
            "registers:update" => Some(Self::RegistersUpdate),
            "registers:delete" => Some(Self::RegistersDelete),

            "officials:read" => Some(Self::OfficialsRead),
            "officials:create" => Some(Self::OfficialsCreate),
            "officials:update" => Some(Self::OfficialsUpdate),
            "officials:delete" => Some(Self::OfficialsDelete),

            "commissions:read" => Some(Self::CommissionsRead),
            "commissions:create" => Some(Self::CommissionsCreate),
            "commissions:update" => Some(Self::CommissionsUpdate),
            "commissions:delete" => Some(Self::CommissionsDelete),

            "transports:read" => Some(Self::TransportsRead),
            "transports:create" => Some(Self::TransportsCreate),
            "transports:update" => Some(Self::TransportsUpdate),
            "transports:delete" => Some(Self::TransportsDelete),

            "personas:read" => Some(Self::PersonasRead),
            "personas:create" => Some(Self::PersonasCreate),
            "personas:update" => Some(Self::PersonasUpdate),
            "personas:delete" => Some(Self::PersonasDelete),

            "parts:read" => Some(Self::PartsRead),
            "parts:create" => Some(Self::PartsCreate),
            "parts:update" => Some(Self::PartsUpdate),
            "parts:complete" => Some(Self::PartsComplete),

            "seclusions:read" => Some(Self::SeclusionsRead),
            "seclusions:create" => Some(Self::SeclusionsCreate),
            "seclusions:update" => Some(Self::SeclusionsUpdate),
            "seclusions:add_visit" => Some(Self::SeclusionsAddVisit),

            "admin:all" => Some(Self::AdminAll),
            "admin:users" => Some(Self::AdminUsers),
            "admin:roles" => Some(Self::AdminRoles),
            "admin:permissions" => Some(Self::AdminPermissions),

            "lookups:read" => Some(Self::LookupsRead),
            "lookups:create" => Some(Self::LookupsCreate),
            "lookups:update" => Some(Self::LookupsUpdate),

            _ => None,
        }
    }

    /// Obtiene todos los permisos disponibles
    pub fn all() -> Vec<Self> {
        vec![
            Self::UsersRead,
            Self::UsersCreate,
            Self::UsersUpdate,
            Self::UsersDelete,
            Self::RegistersRead,
            Self::RegistersCreate,
            Self::RegistersUpdate,
            Self::RegistersDelete,
            Self::OfficialsRead,
            Self::OfficialsCreate,
            Self::OfficialsUpdate,
            Self::OfficialsDelete,
            Self::CommissionsRead,
            Self::CommissionsCreate,
            Self::CommissionsUpdate,
            Self::CommissionsDelete,
            Self::TransportsRead,
            Self::TransportsCreate,
            Self::TransportsUpdate,
            Self::TransportsDelete,
            Self::PersonasRead,
            Self::PersonasCreate,
            Self::PersonasUpdate,
            Self::PersonasDelete,
            Self::PartsRead,
            Self::PartsCreate,
            Self::PartsUpdate,
            Self::PartsComplete,
            Self::SeclusionsRead,
            Self::SeclusionsCreate,
            Self::SeclusionsUpdate,
            Self::SeclusionsAddVisit,
            Self::AdminAll,
            Self::AdminUsers,
            Self::AdminRoles,
            Self::AdminPermissions,
            Self::LookupsRead,
            Self::LookupsCreate,
            Self::LookupsUpdate,
        ]
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
