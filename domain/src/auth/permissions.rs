use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum Permission {
    // Module permissions
    #[sea_orm(string_value = "module:personal")]
    ModulePersonal,
    #[sea_orm(string_value = "module:prevention")]
    ModulePrevention,
    #[sea_orm(string_value = "module:users")]
    ModuleUsers,
    #[sea_orm(string_value = "module:park")]
    ModulePark,

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
    #[sea_orm(string_value = "personas:update_traits")]
    PersonasUpdateTraits,
    #[sea_orm(string_value = "personas:update_academic")]
    PersonasUpdateAcademic,
    #[sea_orm(string_value = "personas:update_courses")]
    PersonasUpdateCourses,
    #[sea_orm(string_value = "personas:update_labor")]
    PersonasUpdateLabor,
    #[sea_orm(string_value = "personas:update_operational")]
    PersonasUpdateOperational,
    #[sea_orm(string_value = "personas:update_health")]
    PersonasUpdateHealth,
    #[sea_orm(string_value = "personas:update_records")]
    PersonasUpdateRecords,
    #[sea_orm(string_value = "personas:update_situation")]
    PersonasUpdateSituation,
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
