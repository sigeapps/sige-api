use crate::api::ApiContext;

pub struct PersonaRecordService;

impl PersonaRecordService {
    pub async fn find_summary(ctx: ApiContext) -> Result<Vec<PersonaSummary>, DbErr> {
        let query = persona::Entity::find().group_by(persona::Column::Id);
    }
}
