use sea_orm::{sea_query::Alias, *};
use std::sync::Arc;

use crate::dtos::{
    personal::persona::{
        child::Child, conyuge::Conyuge, course::Course, educational::Educational, health::Health,
        labor::Labor, operational::Operational, personal::GetPersonalDTO, record::Record,
        relative::Relative, situation::GetSituationDTO, traits::Traits, CreatePersonaDTO,
        GetPersonaDTO, GetPersonaSummaryDTO, UpdatePersonaDTO,
    },
    CommonQueryFilterDTO,
};
use domain::entities::{
    country_verification, persona, persona_children, persona_conyuge, persona_course,
    persona_education, persona_health, persona_operational, persona_record, persona_relative,
    persona_situation, persona_traits, persona_work_experience,
};

#[derive(Debug, Clone)]
pub struct PersonaService {
    pub db: Arc<DatabaseConnection>,
}

impl PersonaService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        PersonaService { db }
    }

    pub async fn create(self, mut dto: CreatePersonaDTO) -> Result<i32, DbErr> {
        let transaction = self.db.begin().await?;

        dto.personal.others = dto.others.others;

        let persona_id = dto
            .personal
            .into_active_model()
            .insert(&transaction)
            .await?
            .id;

        dto.traits.persona_id = persona_id;

        dto.traits.into_active_model().insert(&transaction).await?;

        if let Some(mut conyuge) = dto.conyuge {
            conyuge.persona_id = persona_id;

            conyuge.into_active_model().insert(&transaction).await?;
        }

        dto.health.persona_id = persona_id;

        dto.health.into_active_model().insert(&transaction).await?;

        dto.situation.persona_id = persona_id;
        dto.situation
            .into_active_model()
            .insert(&transaction)
            .await?;

        async {
            for mut operational in dto.operational {
                operational.persona_id = persona_id;

                operational.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut relative in dto.relatives {
                relative.persona_id = persona_id;

                relative.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut course in dto.courses {
                course.persona_id = persona_id;

                course.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut record in dto.records {
                record.persona_id = persona_id;

                record.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut education in dto.education {
                education.persona_id = persona_id;

                education.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut labor in dto.work_experiencies {
                labor.persona_id = persona_id;

                labor.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut children in dto.childrens {
                children.persona_id = persona_id;

                children.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        transaction.commit().await?;

        Ok(persona_id)
    }

    pub async fn update(self, id: i32, mut dto: UpdatePersonaDTO) -> Result<i32, DbErr> {
        let transaction = self.db.begin().await?;

        if let Some(others) = dto.others {
            dto.personal.others = others.others;
        }

        dto.personal.id = id;

        dto.personal.into_active_model().save(&transaction).await?;

        if let Some(mut traits) = dto.traits {
            traits.persona_id = id;

            persona_traits::Entity::update_many()
                .set(traits.into_active_model())
                .filter(persona_traits::Column::PersonaId.eq(id))
                .exec(&transaction)
                .await?;
        }

        if let Some(mut conyuge) = dto.conyuge {
            conyuge.persona_id = id;

            if conyuge.id == 0 {
                conyuge.into_active_model().insert(&transaction).await?;
            } else {
                persona_conyuge::Entity::update_many()
                    .set(conyuge.into_active_model())
                    .filter(persona_conyuge::Column::PersonaId.eq(id))
                    .exec(&transaction)
                    .await?;
            }
        }

        if let Some(mut health) = dto.health {
            health.persona_id = id;

            persona_health::Entity::update_many()
                .set(health.into_active_model())
                .filter(persona_health::Column::PersonaId.eq(id))
                .exec(&transaction)
                .await?;
        }

        if let Some(mut situation) = dto.situation {
            situation.persona_id = id;

            persona_situation::Entity::update_many()
                .set(situation.into_active_model())
                .filter(persona_situation::Column::PersonaId.eq(id))
                .exec(&transaction)
                .await?;
        }

        async {
            for mut operational in dto.operational {
                operational.persona_id = id;

                operational.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut relative in dto.relatives {
                relative.persona_id = id;

                relative.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut course in dto.courses {
                course.persona_id = id;

                course.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut record in dto.records {
                record.persona_id = id;

                record.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut education in dto.education {
                education.persona_id = id;

                education.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut labor in dto.work_experiencies {
                labor.persona_id = id;

                labor.into_active_model().save(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut children in dto.childrens {
                children.persona_id = id;

                let child = children.into_active_model();

                if child.id.is_set() {
                    child.update(&transaction).await?;
                } else {
                    child.insert(&transaction).await?;
                }
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<GetPersonaDTO, DbErr> {
        let persona = persona::Entity::find_by_id(id)
            .into_partial_model::<GetPersonalDTO>()
            .one(&*self.db)
            .await?;

        if persona.is_none() {
            return Err(DbErr::RecordNotFound("Persona not found".to_string()));
        }

        let traits = persona_traits::Entity::find()
            .filter(persona_traits::Column::PersonaId.eq(id))
            .into_partial_model::<Traits>()
            .one(&*self.db)
            .await?;

        let relatives = persona_relative::Entity::find()
            .filter(persona_relative::Column::PersonaId.eq(id))
            .into_partial_model::<Relative>()
            .all(&*self.db)
            .await?;

        let childrens = persona_children::Entity::find()
            .filter(persona_children::Column::PersonaId.eq(id))
            .into_partial_model::<Child>()
            .all(&*self.db)
            .await?;

        let education = persona_education::Entity::find()
            .filter(persona_education::Column::PersonaId.eq(id))
            .into_partial_model::<Educational>()
            .all(&*self.db)
            .await?;

        let conyuge = persona_conyuge::Entity::find()
            .filter(persona_conyuge::Column::PersonaId.eq(id))
            .into_partial_model::<Conyuge>()
            .one(&*self.db)
            .await?;

        let courses = persona_course::Entity::find()
            .filter(persona_course::Column::PersonaId.eq(id))
            .into_partial_model::<Course>()
            .all(&*self.db)
            .await?;

        let work_experiencies = persona_work_experience::Entity::find()
            .filter(persona_work_experience::Column::PersonaId.eq(id))
            .into_partial_model::<Labor>()
            .all(&*self.db)
            .await?;

        let health = persona_health::Entity::find()
            .filter(persona_health::Column::PersonaId.eq(id))
            .into_partial_model::<Health>()
            .one(&*self.db)
            .await?;

        let operational = persona_operational::Entity::find()
            .filter(persona_operational::Column::PersonaId.eq(id))
            .into_partial_model::<Operational>()
            .all(&*self.db)
            .await?;

        let records = persona_record::Entity::find()
            .filter(persona_record::Column::PersonaId.eq(id))
            .into_partial_model::<Record>()
            .all(&*self.db)
            .await?;

        let situation = persona_situation::Entity::find()
            .filter(persona_situation::Column::PersonaId.eq(id))
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Organism.def(),
                Alias::new("organism_origin"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Division2.def(),
                Alias::new("division"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::State2.def(),
                Alias::new("state"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Base2.def(),
                Alias::new("base"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Hierarchy2.def(),
                Alias::new("hierarchy"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Charge2.def(),
                Alias::new("charge"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Division1.def(),
                Alias::new("division_origin"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::State1.def(),
                Alias::new("state_origin"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Base1.def(),
                Alias::new("base_origin"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Hierarchy1.def(),
                Alias::new("hierarchy_origin"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Charge1.def(),
                Alias::new("charge_origin"),
            )
            .into_partial_model::<GetSituationDTO>()
            .one(&*self.db)
            .await?;

        let persona = GetPersonaDTO {
            personal: persona.unwrap(),
            traits,
            relatives,
            childrens,
            education,
            conyuge,
            courses,
            work_experiencies,
            health,
            operational,
            records,
            situation,
            others: None,
        };

        Ok(persona)
    }

    pub async fn find_summary(
        &self,
        filter: CommonQueryFilterDTO,
    ) -> Result<Vec<GetPersonaSummaryDTO>, DbErr> {
        let mut query = persona::Entity::find().left_join(country_verification::Entity);

        if let Some(search) = &filter.search {
            query = query.filter(
                persona::Column::Name
                    .contains(search)
                    .or(persona::Column::LastName.contains(search))
                    .or(persona::Column::Ci.contains(search)),
            );
        }

        if let Some(ci) = &filter.ci {
            query = query.filter(persona::Column::Ci.eq(ci))
        }

        let pagination = &filter.into_pagination();

        let personas = query
            .limit(pagination.limit)
            .offset(pagination.offset)
            .into_partial_model::<GetPersonaSummaryDTO>()
            .all(&*self.db)
            .await?;

        Ok(personas)
    }
}
