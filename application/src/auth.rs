use domain::auth::permissions::Permission;
use sea_orm::ActiveModelBehavior;
use serde::{Deserialize, Serialize};

use crate::dtos::user::GetUserDTO;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserClaims {
    pub user: GetUserDTO,
    pub permissions: Vec<Permission>,
}

pub trait HasBaseId {
    fn set_base_id(self, id: i32) -> Self;
}
pub trait UserStamp {
    fn stamp_user(self, claims: Option<UserClaims>) -> Self;
}

impl<T> UserStamp for T
where
    T: ActiveModelBehavior + HasBaseId,
{
    fn stamp_user(self, claims: Option<UserClaims>) -> Self {
        if let Some(claims) = claims {
            self.set_base_id(claims.user.base.id)
        } else {
            self
        }
    }
}

pub trait FilterByClaims {
    fn filter_by_claims(self, claims: Option<UserClaims>) -> Self;
}

#[macro_export]
macro_rules! impl_filter_by_claims {
    ($entity:ident, $column:ident) => {
        use domain::auth::permissions::Permission;

        impl FilterByClaims for sea_orm::Select<$entity::Entity> {
            fn filter_by_claims(self, claims: Option<UserClaims>) -> Self {
                if let Some(claims) = claims {
                    let has_read_bases = claims
                        .permissions
                        .iter()
                        .any(|x| matches!(x, Permission::ReadAllBases));

                    match has_read_bases {
                        true => self,
                        false => self.filter($entity::Column::$column.eq(claims.user.base.id)),
                    }
                } else {
                    self
                }
            }
        }
    };
}
