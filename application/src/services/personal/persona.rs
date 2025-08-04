use domain::auth::permissions::Permission;
use sea_orm::{sea_query::Alias, *};

use crate::auth::{FilterByClaims, UserClaims};
use crate::dtos::personal::persona::PersonaType;
use crate::{
    api::ApiContext,
    dtos::{
        personal::persona::{
            child::Child,
            conyuge::{Conyuge, GetConyugeDTO},
            course::Course,
            educational::Educational,
            health::Health,
            labor::Labor,
            operational::Operational,
            personal::GetPersonalDTO,
            record::Record,
            relative::Relative,
            situation::{GetSituationDTO, UpdateSituationDTO},
            traits::Traits,
            CreatePersonaDTO, CreatePersonaSummaryDTO, GetPersonaDTO, GetPersonaSummaryDTO,
            UpdatePersonaDTO, UpdatePersonaSummaryDTO,
        },
        CommonQueryFilterDTO, PaginationDTO,
    },
};
use domain::entities::{
    persona, persona_children, persona_conyuge, persona_course, persona_education, persona_health,
    persona_operational, persona_record, persona_relative, persona_situation, persona_traits,
    persona_work_experience,
};

impl FilterByClaims for sea_orm::Select<persona::Entity> {
    fn filter_by_claims(self, claims: Option<UserClaims>) -> Self {
        if let Some(claims) = claims {
            let has_read_bases = claims
                .permissions
                .iter()
                .any(|x| matches!(x, Permission::ReadAllBases));

            println!("has_read_bases: {}", has_read_bases);

            match has_read_bases {
                true => self,
                false => self.filter(persona_situation::Column::BaseId.eq(claims.user.base.id)),
            }
        } else {
            self
        }
    }
}

#[derive(Debug, Clone)]
pub struct PersonaService {}

impl PersonaService {
    // ========================================
    // MÉTODOS RESTful - Nueva API
    // ========================================

    /// Obtener características físicas
    pub async fn get_traits(ctx: &ApiContext, persona_id: i32) -> Result<Option<Traits>, DbErr> {
        persona_traits::Entity::find()
            .filter(persona_traits::Column::PersonaId.eq(persona_id))
            .into_partial_model::<Traits>()
            .one(&ctx.db)
            .await
    }

    /// Crear/Actualizar características físicas
    pub async fn upsert_traits(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: Traits,
    ) -> Result<(), DbErr> {
        dto.persona_id = persona_id;

        let existing = persona_traits::Entity::find()
            .filter(persona_traits::Column::PersonaId.eq(persona_id))
            .one(&ctx.db)
            .await?;

        if existing.is_some() {
            persona_traits::Entity::update_many()
                .set(dto.into_active_model())
                .filter(persona_traits::Column::PersonaId.eq(persona_id))
                .exec(&ctx.db)
                .await?;
        } else {
            dto.into_active_model().insert(&ctx.db).await?;
        }

        Ok(())
    }

    /// Eliminar características físicas
    pub async fn delete_traits(ctx: &ApiContext, persona_id: i32) -> Result<(), DbErr> {
        persona_traits::Entity::delete_many()
            .filter(persona_traits::Column::PersonaId.eq(persona_id))
            .exec(&ctx.db)
            .await?;
        Ok(())
    }

    /// Obtener información de salud
    pub async fn get_health(ctx: &ApiContext, persona_id: i32) -> Result<Option<Health>, DbErr> {
        persona_health::Entity::find()
            .filter(persona_health::Column::PersonaId.eq(persona_id))
            .into_partial_model::<Health>()
            .one(&ctx.db)
            .await
    }

    /// Crear/Actualizar información de salud
    pub async fn upsert_health(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: Health,
    ) -> Result<(), DbErr> {
        dto.persona_id = persona_id;

        let existing = persona_health::Entity::find()
            .filter(persona_health::Column::PersonaId.eq(persona_id))
            .one(&ctx.db)
            .await?;

        if existing.is_some() {
            persona_health::Entity::update_many()
                .set(dto.into_active_model())
                .filter(persona_health::Column::PersonaId.eq(persona_id))
                .exec(&ctx.db)
                .await?;
        } else {
            dto.into_active_model().insert(&ctx.db).await?;
        }

        Ok(())
    }

    /// Eliminar información de salud
    pub async fn delete_health(ctx: &ApiContext, persona_id: i32) -> Result<(), DbErr> {
        persona_health::Entity::delete_many()
            .filter(persona_health::Column::PersonaId.eq(persona_id))
            .exec(&ctx.db)
            .await?;
        Ok(())
    }

    /// Obtener situación actual
    pub async fn get_situation(
        ctx: &ApiContext,
        persona_id: i32,
    ) -> Result<Option<GetSituationDTO>, DbErr> {
        persona_situation::Entity::find()
            .filter(persona_situation::Column::PersonaId.eq(persona_id))
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
                persona_situation::Relation::State.def(),
                Alias::new("state"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Base.def(),
                Alias::new("base"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Hierarchy.def(),
                Alias::new("hierarchy"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Charge.def(),
                Alias::new("charge"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Division1.def(),
                Alias::new("division_origin"),
            )
            .into_partial_model::<GetSituationDTO>()
            .one(&ctx.db)
            .await
    }

    /// Crear/Actualizar situación
    pub async fn upsert_situation(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: UpdateSituationDTO,
    ) -> Result<(), DbErr> {
        dto.persona_id = persona_id;

        let existing = persona_situation::Entity::find()
            .filter(persona_situation::Column::PersonaId.eq(persona_id))
            .one(&ctx.db)
            .await?;

        if existing.is_some() {
            persona_situation::Entity::update_many()
                .set(dto.into_active_model())
                .filter(persona_situation::Column::PersonaId.eq(persona_id))
                .exec(&ctx.db)
                .await?;
        } else {
            dto.into_active_model().insert(&ctx.db).await?;
        }

        Ok(())
    }

    /// Obtener cónyuge
    pub async fn get_spouse(ctx: &ApiContext, persona_id: i32) -> Result<Option<Conyuge>, DbErr> {
        persona_conyuge::Entity::find()
            .filter(persona_conyuge::Column::PersonaId.eq(persona_id))
            .into_partial_model::<Conyuge>()
            .one(&ctx.db)
            .await
    }

    /// Crear/Actualizar cónyuge
    pub async fn upsert_spouse(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: GetConyugeDTO,
    ) -> Result<(), DbErr> {
        dto.persona_id = persona_id;

        let existing = persona_conyuge::Entity::find()
            .filter(persona_conyuge::Column::PersonaId.eq(persona_id))
            .one(&ctx.db)
            .await?;

        if existing.is_some() {
            persona_conyuge::Entity::update_many()
                .set(dto.into_active_model())
                .filter(persona_conyuge::Column::PersonaId.eq(persona_id))
                .exec(&ctx.db)
                .await?;
        } else {
            dto.into_active_model().insert(&ctx.db).await?;
        }

        Ok(())
    }

    /// Eliminar cónyuge
    pub async fn delete_spouse(ctx: &ApiContext, persona_id: i32) -> Result<(), DbErr> {
        persona_conyuge::Entity::delete_many()
            .filter(persona_conyuge::Column::PersonaId.eq(persona_id))
            .exec(&ctx.db)
            .await?;
        Ok(())
    }

    /// Validar si una persona puede tener información operacional
    pub async fn can_have_operational(ctx: &ApiContext, persona_id: i32) -> Result<bool, DbErr> {
        let persona = persona::Entity::find_by_id(persona_id)
            .one(&ctx.db)
            .await?
            .ok_or(DbErr::RecordNotFound("Persona not found".to_string()))?;

        Ok(persona.state_id == Some(1))
    }

    /// Validar antes de agregar información operacional
    async fn validate_operational_access(ctx: &ApiContext, persona_id: i32) -> Result<(), DbErr> {
        if !Self::can_have_operational(ctx, persona_id).await? {
            return Err(DbErr::Custom(
                "Solo los funcionarios pueden tener información operacional".to_string(),
            ));
        }
        Ok(())
    }

    // ========================================
    // MÉTODO ATÓMICO - Para formularios completos
    // ========================================

    /// Crear persona completa de forma atómica (RECOMENDADO para formularios completos)
    /// Garantiza que toda la información se guarde o nada se guarde
    pub async fn create_complete(ctx: ApiContext, mut dto: CreatePersonaDTO) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

        if let Some(others) = dto.others {
            dto.personal.others = others.others;
        }

        dto.personal.r#type = PersonaType(dto.r#type.into());

        // 1. Crear persona principal
        let persona_id = dto
            .personal
            .into_active_model()
            .insert(&transaction)
            .await?
            .id;

        // 2. Crear traits (obligatorio)
        if let Some(mut traits) = dto.traits {
            traits.persona_id = persona_id;
            traits.into_active_model().insert(&transaction).await?;
        }

        // 3. Crear salud (obligatorio)
        if let Some(mut health) = dto.health {
            health.persona_id = persona_id;
            health.into_active_model().insert(&transaction).await?;
        }

        if let Some(mut situation) = dto.situation {
            situation.persona_id = persona_id;
            situation.into_active_model().insert(&transaction).await?;
        }

        // 5. Crear cónyuge (opcional)
        if let Some(mut conyuge) = dto.conyuge {
            conyuge.persona_id = persona_id;
            conyuge.into_active_model().insert(&transaction).await?;
        }

        // 6. Crear colecciones en paralelo para mejor performance
        let operational_future = async {
            for mut operational in dto.operational.unwrap_or_default() {
                operational.persona_id = persona_id;
                operational.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        };

        let relatives_future = async {
            for mut relative in dto.relatives.unwrap_or_default() {
                relative.persona_id = persona_id;
                relative.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        };

        let courses_future = async {
            for mut course in dto.courses.unwrap_or_default() {
                course.persona_id = persona_id;
                course.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        };

        let records_future = async {
            for mut record in dto.records.unwrap_or_default() {
                record.persona_id = persona_id;
                record.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        };

        let education_future = async {
            for mut education in dto.education.unwrap_or_default() {
                education.persona_id = persona_id;
                education.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        };

        let work_future = async {
            for mut labor in dto.work_experiencies.unwrap_or_default() {
                labor.persona_id = persona_id;
                labor.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        };

        let children_future = async {
            for mut children in dto.childrens.unwrap_or_default() {
                children.persona_id = persona_id;
                children.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        };

        // Ejecutar todas las operaciones en paralelo
        tokio::try_join!(
            operational_future,
            relatives_future,
            courses_future,
            records_future,
            education_future,
            work_future,
            children_future
        )?;

        transaction.commit().await?;
        Ok(persona_id)
    }

    // ========================================
    // MÉTODOS LEGACY - Compatibilidad
    // ========================================

    #[deprecated(
        note = "Use create_complete for atomic operations or create_basic + individual methods for step-by-step"
    )]
    pub async fn create(ctx: ApiContext, dto: CreatePersonaDTO) -> Result<i32, DbErr> {
        Self::create_complete(ctx, dto).await
    }

    pub async fn update(ctx: ApiContext, id: i32, mut dto: UpdatePersonaDTO) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

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

        // TODO: Verificar si es necesario

        if let Some(mut conyuge) = dto.conyuge {
            conyuge.persona_id = id;

            // Verificar si existe un cónyuge para esta persona
            let existing_conyuge = persona_conyuge::Entity::find()
                .filter(persona_conyuge::Column::PersonaId.eq(id))
                .one(&transaction)
                .await?;

            if existing_conyuge.is_some() {
                // Si existe, actualizar
                persona_conyuge::Entity::update_many()
                    .set(conyuge.into_active_model())
                    .filter(persona_conyuge::Column::PersonaId.eq(id))
                    .exec(&transaction)
                    .await?;
            } else {
                // Si no existe, insertar uno nuevo
                conyuge.into_active_model().insert(&transaction).await?;
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

    pub async fn find_by_id(ctx: ApiContext, id: i32) -> Result<GetPersonaDTO, DbErr> {
        let persona = persona::Entity::find_by_id(id)
            .into_partial_model::<GetPersonalDTO>()
            .one(&ctx.db)
            .await?;

        if persona.is_none() {
            return Err(DbErr::RecordNotFound("Persona not found".to_string()));
        }

        let traits = persona_traits::Entity::find()
            .filter(persona_traits::Column::PersonaId.eq(id))
            .into_partial_model::<Traits>()
            .one(&ctx.db)
            .await?;

        let relatives = persona_relative::Entity::find()
            .filter(persona_relative::Column::PersonaId.eq(id))
            .into_partial_model::<Relative>()
            .all(&ctx.db)
            .await?;

        let childrens = persona_children::Entity::find()
            .filter(persona_children::Column::PersonaId.eq(id))
            .into_partial_model::<Child>()
            .all(&ctx.db)
            .await?;

        let education = persona_education::Entity::find()
            .filter(persona_education::Column::PersonaId.eq(id))
            .into_partial_model::<Educational>()
            .all(&ctx.db)
            .await?;

        let conyuge = persona_conyuge::Entity::find()
            .filter(persona_conyuge::Column::PersonaId.eq(id))
            .into_partial_model::<Conyuge>()
            .one(&ctx.db)
            .await?;

        let courses = persona_course::Entity::find()
            .filter(persona_course::Column::PersonaId.eq(id))
            .into_partial_model::<Course>()
            .all(&ctx.db)
            .await?;

        let work_experiencies = persona_work_experience::Entity::find()
            .filter(persona_work_experience::Column::PersonaId.eq(id))
            .into_partial_model::<Labor>()
            .all(&ctx.db)
            .await?;

        let health = persona_health::Entity::find()
            .filter(persona_health::Column::PersonaId.eq(id))
            .into_partial_model::<Health>()
            .one(&ctx.db)
            .await?;

        let operational = persona_operational::Entity::find()
            .filter(persona_operational::Column::PersonaId.eq(id))
            .into_partial_model::<Operational>()
            .all(&ctx.db)
            .await?;

        let records = persona_record::Entity::find()
            .filter(persona_record::Column::PersonaId.eq(id))
            .into_partial_model::<Record>()
            .all(&ctx.db)
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
                persona_situation::Relation::State.def(),
                Alias::new("state"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Base.def(),
                Alias::new("base"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Hierarchy.def(),
                Alias::new("hierarchy"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Charge.def(),
                Alias::new("charge"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Division1.def(),
                Alias::new("division_origin"),
            )
            .into_partial_model::<GetSituationDTO>()
            .one(&ctx.db)
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
        ctx: ApiContext,
        filter: CommonQueryFilterDTO,
    ) -> Result<Vec<GetPersonaSummaryDTO>, DbErr> {
        let mut query = persona::Entity::find()
            .join_as(
                JoinType::LeftJoin,
                persona::Relation::PersonaState.def(),
                Alias::new("persona_state"),
            )
            .left_join(persona_situation::Entity)
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::State.def(),
                Alias::new("state"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Division2.def(),
                Alias::new("division"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Base.def(),
                Alias::new("base"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Organism.def(),
                Alias::new("organism"),
            )
            .join_as(
                JoinType::LeftJoin,
                persona_situation::Relation::Hierarchy.def(),
                Alias::new("hierarchy"),
            );

        if let Some(search) = &filter.search {
            query = query.filter(
                persona::Column::Name
                    .contains(search)
                    .or(persona::Column::LastName.contains(search))
                    .or(persona::Column::Ci.contains(search)),
            );
        }

        if let Some(ci) = &filter.ci {
            query = query.filter(persona::Column::Ci.eq(ci));
        }

        println!("persona_type: {:?}", &filter.persona_type);

        if let Some(persona_type) = &filter.persona_type {
            query = query.filter(persona::Column::Type.eq(persona_type));
        }

        if let Some(from_date) = &filter.from_date {
            query = query.filter(persona::Column::CreatedAt.gte(*from_date));
        }

        if let Some(to_date) = &filter.to_date {
            query = query.filter(persona::Column::CreatedAt.lte(*to_date));
        }

        if let Some(sort) = &filter.sort {
            // Ejemplo simple: sort = "name:asc" o "created_at:desc"
            let parts: Vec<&str> = sort.split(':').collect();
            if parts.len() == 2 {
                let column = parts[0];
                let direction = parts[1];
                match (column, direction) {
                    ("name", "asc") => {
                        query = query.order_by_asc(persona::Column::Name);
                    }
                    ("name", "desc") => {
                        query = query.order_by_desc(persona::Column::Name);
                    }
                    ("created_at", "asc") => {
                        query = query.order_by_asc(persona::Column::CreatedAt);
                    }
                    ("created_at", "desc") => {
                        query = query.order_by_desc(persona::Column::CreatedAt);
                    }
                    _ => {}
                }
            }
        }

        let pagination = &filter.into_pagination();

        let personas = query
            .limit(pagination.limit)
            .filter_by_claims(ctx.claims)
            .offset(pagination.offset)
            .into_partial_model::<GetPersonaSummaryDTO>()
            .all(&ctx.db)
            .await?;

        Ok(personas)
    }

    pub async fn get_pagination(
        ctx: ApiContext,
        filter: CommonQueryFilterDTO,
    ) -> Result<PaginationDTO, DbErr> {
        let mut query = persona::Entity::find();

        if let Some(search) = &filter.search {
            query = query.filter(
                persona::Column::Name
                    .contains(search)
                    .or(persona::Column::LastName.contains(search))
                    .or(persona::Column::Ci.contains(search)),
            );
        }

        if let Some(ci) = &filter.ci {
            query = query.filter(persona::Column::Ci.eq(ci));
        }

        if let Some(persona_type) = &filter.persona_type {
            query = query.filter(persona::Column::Type.eq(persona_type));
        }

        if let Some(from_date) = &filter.from_date {
            query = query.filter(persona::Column::CreatedAt.gte(*from_date));
        }

        if let Some(to_date) = &filter.to_date {
            query = query.filter(persona::Column::CreatedAt.lte(*to_date));
        }

        let pagination = filter.into_pagination();
        let paginator = query.paginate(&ctx.db, pagination.limit);
        let total_count = paginator.num_items().await?;
        let page_count = paginator.num_pages().await?;

        Ok(PaginationDTO {
            page: pagination.page,
            limit: pagination.limit,
            page_count,
            total_count,
            offset: pagination.offset,
        })
    }

    pub async fn update_summary(
        ctx: ApiContext,
        id: i32,
        mut dto: UpdatePersonaSummaryDTO,
    ) -> Result<i32, DbErr> {
        dto.persona.id = id;

        let id = dto.persona.into_active_model().update(&ctx.db).await?.id;

        persona_situation::Entity::update_many()
            .set(dto.situation.into_active_model())
            .filter(persona_situation::Column::PersonaId.eq(id))
            .exec(&ctx.db)
            .await?;

        Ok(id)
    }
}

macro_rules! update_entity {
    ($fn_name:ident, $dto_type:ty, $entity:ty, $column:expr) => {
        pub async fn $fn_name(ctx: &ApiContext, id: i32, mut dto: $dto_type) -> Result<i32, DbErr> {
            dto.persona_id = id;

            let active_model = dto.into_active_model();

            <$entity>::update_many()
                .set(active_model)
                .filter($column.eq(id))
                .exec(&ctx.db)
                .await?;

            Ok(id)
        }
    };
}

impl PersonaService {
    update_entity!(
        update_traits,
        Traits,
        persona_traits::Entity,
        persona_traits::Column::PersonaId
    );

    update_entity!(
        update_health,
        Health,
        persona_health::Entity,
        persona_health::Column::PersonaId
    );

    update_entity!(
        update_situation,
        UpdateSituationDTO,
        persona_situation::Entity,
        persona_situation::Column::PersonaId
    );

    pub async fn update_conyuge(
        ctx: &ApiContext,
        id: i32,
        mut dto: GetConyugeDTO,
    ) -> Result<i32, DbErr> {
        dto.persona_id = id;

        let existing_conyuge = persona_conyuge::Entity::find()
            .filter(persona_conyuge::Column::PersonaId.eq(id))
            .one(&ctx.db)
            .await?;

        if existing_conyuge.is_some() {
            persona_conyuge::Entity::update_many()
                .set(dto.into_active_model())
                .filter(persona_conyuge::Column::PersonaId.eq(id))
                .exec(&ctx.db)
                .await?;
        } else {
            dto.into_active_model().insert(&ctx.db).await?;
        }

        Ok(id)
    }

    /// Obtener información operacional (RESTful)
    pub async fn get_operational(
        ctx: &ApiContext,
        persona_id: i32,
    ) -> Result<Vec<Operational>, DbErr> {
        Self::validate_operational_access(ctx, persona_id).await?;

        persona_operational::Entity::find()
            .filter(persona_operational::Column::PersonaId.eq(persona_id))
            .into_partial_model::<Operational>()
            .all(&ctx.db)
            .await
    }

    /// Agregar información operacional (RESTful)
    pub async fn create_operational(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: Operational,
    ) -> Result<i32, DbErr> {
        Self::validate_operational_access(ctx, persona_id).await?;

        dto.persona_id = persona_id;
        let result = dto.into_active_model().insert(&ctx.db).await?;
        Ok(result.id)
    }

    /// Actualizar información operacional específica (RESTful)
    pub async fn update_operational(
        ctx: &ApiContext,
        persona_id: i32,
        operational_id: i32,
        mut dto: Operational,
    ) -> Result<(), DbErr> {
        Self::validate_operational_access(ctx, persona_id).await?;

        dto.persona_id = persona_id;
        let mut active_model = dto.into_active_model();
        active_model.id = Set(operational_id);

        active_model.update(&ctx.db).await?;
        Ok(())
    }

    /// Eliminar información operacional específica (RESTful)
    pub async fn delete_operational(
        ctx: &ApiContext,
        persona_id: i32,
        operational_id: i32,
    ) -> Result<(), DbErr> {
        Self::validate_operational_access(ctx, persona_id).await?;

        persona_operational::Entity::delete_by_id(operational_id)
            .exec(&ctx.db)
            .await?;
        Ok(())
    }

    /// Reemplazar toda la información operacional (Legacy method, now with validation)
    pub async fn add_operational(
        ctx: &ApiContext,
        id: i32,
        dto: Vec<Operational>,
    ) -> Result<i32, DbErr> {
        Self::validate_operational_access(ctx, id).await?;

        let transaction = ctx.db.begin().await?;

        persona_operational::Entity::delete_many()
            .filter(persona_operational::Column::PersonaId.eq(id))
            .exec(&ctx.db)
            .await?;

        for mut operational in dto {
            operational.persona_id = id;

            operational.into_active_model().insert(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(id)
    }

    /// ========================================
    /// MÉTODOS RESTful para Familiares
    /// ========================================

    /// Obtener familiares
    pub async fn get_relatives(ctx: &ApiContext, persona_id: i32) -> Result<Vec<Relative>, DbErr> {
        persona_relative::Entity::find()
            .filter(persona_relative::Column::PersonaId.eq(persona_id))
            .into_partial_model::<Relative>()
            .all(&ctx.db)
            .await
    }

    /// Agregar familiar
    pub async fn create_relative(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: Relative,
    ) -> Result<i32, DbErr> {
        dto.persona_id = persona_id;
        let result = dto.into_active_model().insert(&ctx.db).await?;
        Ok(result.id)
    }

    /// Actualizar familiar específico
    pub async fn update_relative(
        ctx: &ApiContext,
        persona_id: i32,
        relative_id: i32,
        mut dto: Relative,
    ) -> Result<(), DbErr> {
        dto.persona_id = persona_id;
        let mut active_model = dto.into_active_model();
        active_model.id = Set(relative_id);

        active_model.update(&ctx.db).await?;
        Ok(())
    }

    /// Eliminar familiar específico
    pub async fn delete_relative(
        ctx: &ApiContext,
        _persona_id: i32,
        relative_id: i32,
    ) -> Result<(), DbErr> {
        persona_relative::Entity::delete_by_id(relative_id)
            .exec(&ctx.db)
            .await?;
        Ok(())
    }

    /// ========================================
    /// MÉTODOS RESTful para Educación
    /// ========================================

    /// Obtener educación
    pub async fn get_education(
        ctx: &ApiContext,
        persona_id: i32,
    ) -> Result<Vec<Educational>, DbErr> {
        persona_education::Entity::find()
            .filter(persona_education::Column::PersonaId.eq(persona_id))
            .into_partial_model::<Educational>()
            .all(&ctx.db)
            .await
    }

    /// Agregar educación
    pub async fn create_education(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: Educational,
    ) -> Result<i32, DbErr> {
        dto.persona_id = persona_id;
        let result = dto.into_active_model().insert(&ctx.db).await?;
        Ok(result.id)
    }

    /// Actualizar educación específica
    pub async fn update_education(
        ctx: &ApiContext,
        persona_id: i32,
        education_id: i32,
        mut dto: Educational,
    ) -> Result<(), DbErr> {
        dto.persona_id = persona_id;
        dto.id = education_id;
        let active_model = dto.into_active_model();

        active_model.update(&ctx.db).await?;
        Ok(())
    }

    /// Eliminar educación específica
    pub async fn delete_education(
        ctx: &ApiContext,
        _persona_id: i32,
        education_id: i32,
    ) -> Result<(), DbErr> {
        persona_education::Entity::delete_by_id(education_id)
            .exec(&ctx.db)
            .await?;
        Ok(())
    }

    /// ========================================
    /// MÉTODOS RESTful para Cursos
    /// ========================================

    /// Obtener cursos
    pub async fn get_courses(ctx: &ApiContext, persona_id: i32) -> Result<Vec<Course>, DbErr> {
        persona_course::Entity::find()
            .filter(persona_course::Column::PersonaId.eq(persona_id))
            .into_partial_model::<Course>()
            .all(&ctx.db)
            .await
    }

    /// Agregar curso
    pub async fn create_course(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: Course,
    ) -> Result<i32, DbErr> {
        dto.persona_id = persona_id;
        let result = dto.into_active_model().insert(&ctx.db).await?;
        Ok(result.id)
    }

    /// Actualizar curso específico
    pub async fn update_course(
        ctx: &ApiContext,
        persona_id: i32,
        course_id: i32,
        mut dto: Course,
    ) -> Result<(), DbErr> {
        dto.persona_id = persona_id;
        let mut active_model = dto.into_active_model();
        active_model.id = Set(course_id);

        active_model.update(&ctx.db).await?;
        Ok(())
    }

    /// Eliminar curso específico
    pub async fn delete_course(
        ctx: &ApiContext,
        _persona_id: i32,
        course_id: i32,
    ) -> Result<(), DbErr> {
        persona_course::Entity::delete_by_id(course_id)
            .exec(&ctx.db)
            .await?;
        Ok(())
    }

    /// ========================================
    /// MÉTODOS RESTful para Experiencia Laboral
    /// ========================================

    /// Obtener experiencia laboral
    pub async fn get_work_experience(
        ctx: &ApiContext,
        persona_id: i32,
    ) -> Result<Vec<Labor>, DbErr> {
        persona_work_experience::Entity::find()
            .filter(persona_work_experience::Column::PersonaId.eq(persona_id))
            .into_partial_model::<Labor>()
            .all(&ctx.db)
            .await
    }

    /// Agregar experiencia laboral
    pub async fn create_work_experience(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: Labor,
    ) -> Result<i32, DbErr> {
        dto.persona_id = persona_id;
        let result = dto.into_active_model().insert(&ctx.db).await?;
        Ok(result.id)
    }

    /// Actualizar experiencia laboral específica
    pub async fn update_work_experience(
        ctx: &ApiContext,
        persona_id: i32,
        experience_id: i32,
        mut dto: Labor,
    ) -> Result<(), DbErr> {
        dto.persona_id = persona_id;
        let mut active_model = dto.into_active_model();
        active_model.id = Set(experience_id);

        active_model.update(&ctx.db).await?;
        Ok(())
    }

    /// Eliminar experiencia laboral específica
    pub async fn delete_work_experience(
        ctx: &ApiContext,
        _persona_id: i32,
        experience_id: i32,
    ) -> Result<(), DbErr> {
        persona_work_experience::Entity::delete_by_id(experience_id)
            .exec(&ctx.db)
            .await?;
        Ok(())
    }

    /// ========================================
    /// MÉTODOS RESTful para Hijos
    /// ========================================

    /// Obtener hijos
    pub async fn get_children(ctx: &ApiContext, persona_id: i32) -> Result<Vec<Child>, DbErr> {
        persona_children::Entity::find()
            .filter(persona_children::Column::PersonaId.eq(persona_id))
            .into_partial_model::<Child>()
            .all(&ctx.db)
            .await
    }

    /// Agregar hijo
    pub async fn create_child(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: Child,
    ) -> Result<i32, DbErr> {
        dto.persona_id = persona_id;
        let result = dto.into_active_model().insert(&ctx.db).await?;
        Ok(result.id)
    }

    /// Actualizar hijo específico
    pub async fn update_child(
        ctx: &ApiContext,
        persona_id: i32,
        child_id: i32,
        mut dto: Child,
    ) -> Result<(), DbErr> {
        dto.persona_id = persona_id;
        let mut active_model = dto.into_active_model();
        active_model.id = Set(child_id);

        active_model.update(&ctx.db).await?;
        Ok(())
    }

    /// Eliminar hijo específico
    pub async fn delete_child(
        ctx: &ApiContext,
        _persona_id: i32,
        child_id: i32,
    ) -> Result<(), DbErr> {
        persona_children::Entity::delete_by_id(child_id)
            .exec(&ctx.db)
            .await?;
        Ok(())
    }

    /// ========================================
    /// MÉTODOS RESTful para Antecedentes
    /// ========================================

    /// Obtener antecedentes
    pub async fn get_records(ctx: &ApiContext, persona_id: i32) -> Result<Vec<Record>, DbErr> {
        persona_record::Entity::find()
            .filter(persona_record::Column::PersonaId.eq(persona_id))
            .into_partial_model::<Record>()
            .all(&ctx.db)
            .await
    }

    /// Agregar antecedente
    pub async fn create_record(
        ctx: &ApiContext,
        persona_id: i32,
        mut dto: Record,
    ) -> Result<i32, DbErr> {
        dto.persona_id = persona_id;
        let result = dto.into_active_model().insert(&ctx.db).await?;
        Ok(result.id)
    }

    /// Actualizar antecedente específico
    pub async fn update_record(
        ctx: &ApiContext,
        persona_id: i32,
        record_id: i32,
        mut dto: Record,
    ) -> Result<(), DbErr> {
        dto.persona_id = persona_id;
        let mut active_model = dto.into_active_model();
        active_model.id = Set(record_id);

        active_model.update(&ctx.db).await?;
        Ok(())
    }

    /// Eliminar antecedente específico
    pub async fn delete_record(
        ctx: &ApiContext,
        _persona_id: i32,
        record_id: i32,
    ) -> Result<(), DbErr> {
        persona_record::Entity::delete_by_id(record_id)
            .exec(&ctx.db)
            .await?;
        Ok(())
    }

    /// ========================================
    /// MÉTODOS LEGACY - Compatibilidad (Reemplazar colecciones completas)
    /// ========================================

    /// Reemplazar todos los familiares (Legacy method)
    pub async fn add_relatives(
        ctx: &ApiContext,
        id: i32,
        dto: Vec<Relative>,
    ) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

        persona_relative::Entity::delete_many()
            .filter(persona_relative::Column::PersonaId.eq(id))
            .exec(&ctx.db)
            .await?;

        for mut relative in dto {
            relative.persona_id = id;

            relative.into_active_model().insert(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn add_courses(ctx: &ApiContext, id: i32, dto: Vec<Course>) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;
        persona_course::Entity::delete_many()
            .filter(persona_course::Column::PersonaId.eq(id))
            .exec(&ctx.db)
            .await?;

        for mut course in dto {
            course.persona_id = id;

            course.into_active_model().insert(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn add_records(ctx: &ApiContext, id: i32, dto: Vec<Record>) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

        persona_record::Entity::delete_many()
            .filter(persona_record::Column::PersonaId.eq(id))
            .exec(&ctx.db)
            .await?;

        for mut record in dto {
            record.persona_id = id;

            record.into_active_model().insert(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn add_education(
        ctx: &ApiContext,
        id: i32,
        dto: Vec<Educational>,
    ) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

        persona_education::Entity::delete_many()
            .filter(persona_education::Column::PersonaId.eq(id))
            .exec(&ctx.db)
            .await?;

        for mut education in dto {
            education.persona_id = id;

            education.into_active_model().insert(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn add_work_experience(
        ctx: &ApiContext,
        id: i32,
        dto: Vec<Labor>,
    ) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

        persona_work_experience::Entity::delete_many()
            .filter(persona_work_experience::Column::PersonaId.eq(id))
            .exec(&ctx.db)
            .await?;

        for mut labor in dto {
            labor.persona_id = id;

            labor.into_active_model().insert(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn add_childrens(ctx: &ApiContext, id: i32, dto: Vec<Child>) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

        persona_children::Entity::delete_many()
            .filter(persona_work_experience::Column::PersonaId.eq(id))
            .exec(&ctx.db)
            .await?;

        for mut children in dto {
            children.persona_id = id;

            children.into_active_model().insert(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn create_summary(
        ctx: &ApiContext,
        dto: CreatePersonaSummaryDTO,
    ) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

        let persona = persona::ActiveModel {
            ci: Set(dto.persona.ci),
            genre: Set(dto.persona.genre),
            // Asignar valores por defecto para los campos requeridos
            front_photo: Set(None),
            back_photo: Set(None),
            passport_number: Set(None),
            passport_expiration: Set(None),
            passport_years_valid: Set(None),
            name: Set(String::from("")),
            last_name: Set(String::from("")),
            birthdate: Set(String::from("")),
            email: Set(String::from("")),
            age: Set(0),
            birthplace: Set(String::from("")),
            address: Set(String::from("")),
            phone: Set(String::from("")),
            coordinates: Set(None),
            status_civil: Set(String::from("")),
            bank_account: Set(String::from("")),
            homeland_ci: Set(String::from("")),
            vehicle_license: Set(String::from("")),
            state_id: Set(None),
            others: Set(None),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;

        persona_situation::ActiveModel {
            persona_id: Set(persona.id),
            requested_by_id: Set(1),
            date: Set(chrono::Utc::now().naive_utc().date()),
            situation_type: Set(String::from("")),
            entry_type: Set(None),
            division_id: Set(Some(dto.situation.division_id)),
            state_id: Set(Some(dto.situation.state_id)),
            base_id: Set(Some(dto.situation.base_id)),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;

        transaction.commit().await?;

        Ok(persona.id)
    }
}
