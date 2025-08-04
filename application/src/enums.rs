use domain::entities::sea_orm_active_enums::InclusionTypeEnum;
use sea_orm::Iterable;
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::{schema::SchemaType, ObjectBuilder, Type},
    PartialSchema, ToSchema,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InclusionType(pub InclusionTypeEnum);

impl From<InclusionType> for InclusionTypeEnum {
    fn from(persona_type: InclusionType) -> Self {
        persona_type.0
    }
}

impl Default for InclusionType {
    fn default() -> Self {
        InclusionType(InclusionTypeEnum::Complaint)
    }
}

impl From<InclusionTypeEnum> for InclusionType {
    fn from(type_enum: InclusionTypeEnum) -> Self {
        InclusionType(type_enum)
    }
}

impl sea_orm::IntoActiveValue<InclusionTypeEnum> for InclusionType {
    fn into_active_value(self) -> sea_orm::ActiveValue<InclusionTypeEnum> {
        sea_orm::ActiveValue::Set(self.0)
    }
}

impl ToSchema for InclusionType {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("InclusionType")
    }
}

impl PartialSchema for InclusionType {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        ObjectBuilder::new()
            .schema_type(SchemaType::new(Type::String))
            .enum_values(Some(
                InclusionTypeEnum::iter()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<String>>(),
            ))
            .into()
    }
}
