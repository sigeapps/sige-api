use crate::Result;
use application::{
    dtos::lookup::CreateBasicLookUpDTO, services::prevention::lookup::LookupService,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use domain::entities::{
    band, base, brand, brigade, charge, division, document_type, family_relationship, hierarchy,
    institution, municipality, novelty, organism, parish, persona_state, profession, role,
    seclusion_statuses, state, status_condition, transport_statuses, transport_type, vehicle_model,
    weapon_brand, weapon_model, weapon_type,
};
use sea_orm::ActiveModelTrait;
use tracing::debug;

use application::api::ApiContext;

// TODO: Refactoriza esta macro usando funciones tuplas

macro_rules! get_lookup {
    ($func_name:ident, $entity:ty, $model:ty, $active_model:ty) => {
        pub async fn $func_name(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
            let items = LookupService::find::<$entity, $model, $active_model>(ctx).await?;
            debug!("{:?}", Json(&items));
            Ok((StatusCode::OK, Json(items)).into_response())
        }
    };
}

macro_rules! create_lookup {
    ($func_name:ident, $entity:ty, $active_model:ty, $dto:ty) => {
        pub async fn $func_name(
            Extension(ctx): Extension<ApiContext>,
            Json(dto): Json<$dto>,
        ) -> Result<Response> {
            let active_model = dto.into_active_model();
            LookupService::create::<$entity, _, _>(ctx, active_model).await?;
            Ok(StatusCode::CREATED.into_response())
        }
    };
}

trait DtoIntoActiveModel<T: ActiveModelTrait> {
    fn into_active_model(self) -> T;
}

impl DtoIntoActiveModel<weapon_brand::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> weapon_brand::ActiveModel {
        weapon_brand::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<weapon_model::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> weapon_model::ActiveModel {
        weapon_model::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<weapon_type::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> weapon_type::ActiveModel {
        weapon_type::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<brigade::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> brigade::ActiveModel {
        brigade::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<organism::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> organism::ActiveModel {
        organism::ActiveModel {
            name: sea_orm::Set(self.name),
            phone: sea_orm::Set(String::new()), // Default empty string for phone
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<division::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> division::ActiveModel {
        division::ActiveModel {
            name: sea_orm::Set(self.name),
            state: sea_orm::Set(self.state.unwrap_or_default()),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<charge::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> charge::ActiveModel {
        charge::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<hierarchy::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> hierarchy::ActiveModel {
        hierarchy::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<state::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> state::ActiveModel {
        state::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<municipality::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> municipality::ActiveModel {
        municipality::ActiveModel {
            name: sea_orm::Set(self.name),
            state: sea_orm::Set(self.state.unwrap_or_default()),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<transport_type::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> transport_type::ActiveModel {
        transport_type::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<transport_statuses::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> transport_statuses::ActiveModel {
        transport_statuses::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<brand::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> brand::ActiveModel {
        brand::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<vehicle_model::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> vehicle_model::ActiveModel {
        vehicle_model::ActiveModel {
            name: sea_orm::Set(self.name),
            brand: sea_orm::Set(self.brand.unwrap_or_default()),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<parish::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> parish::ActiveModel {
        parish::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<seclusion_statuses::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> seclusion_statuses::ActiveModel {
        seclusion_statuses::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<family_relationship::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> family_relationship::ActiveModel {
        family_relationship::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<novelty::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> novelty::ActiveModel {
        novelty::ActiveModel {
            name: sea_orm::Set(self.name),
            format: sea_orm::Set(self.format.unwrap_or_default()),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<band::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> band::ActiveModel {
        band::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<institution::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> institution::ActiveModel {
        institution::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<profession::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> profession::ActiveModel {
        profession::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<base::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> base::ActiveModel {
        base::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<persona_state::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> persona_state::ActiveModel {
        persona_state::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

impl DtoIntoActiveModel<document_type::ActiveModel> for CreateBasicLookUpDTO {
    fn into_active_model(self) -> document_type::ActiveModel {
        document_type::ActiveModel {
            name: sea_orm::Set(self.name),
            ..Default::default()
        }
    }
}

get_lookup!(
    get_brigades,
    brigade::Entity,
    brigade::Model,
    brigade::ActiveModel
);
create_lookup!(
    create_brigade,
    brigade::Entity,
    brigade::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_organisms,
    organism::Entity,
    organism::Model,
    organism::ActiveModel
);
create_lookup!(
    create_organism,
    organism::Entity,
    organism::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_divisions,
    division::Entity,
    division::Model,
    division::ActiveModel
);
create_lookup!(
    create_division,
    division::Entity,
    division::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_charges,
    charge::Entity,
    charge::Model,
    charge::ActiveModel
);
create_lookup!(
    create_charge,
    charge::Entity,
    charge::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_hierarchies,
    hierarchy::Entity,
    hierarchy::Model,
    hierarchy::ActiveModel
);
create_lookup!(
    create_hierarchy,
    hierarchy::Entity,
    hierarchy::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(get_states, state::Entity, state::Model, state::ActiveModel);
create_lookup!(
    create_state,
    state::Entity,
    state::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_municipalities,
    municipality::Entity,
    municipality::Model,
    municipality::ActiveModel
);
create_lookup!(
    create_municipality,
    municipality::Entity,
    municipality::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_transport_types,
    transport_type::Entity,
    transport_type::Model,
    transport_type::ActiveModel
);
create_lookup!(
    create_transport_type,
    transport_type::Entity,
    transport_type::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_transport_statuses,
    transport_statuses::Entity,
    transport_statuses::Model,
    transport_statuses::ActiveModel
);
create_lookup!(
    create_transport_status,
    transport_statuses::Entity,
    transport_statuses::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(get_brands, brand::Entity, brand::Model, brand::ActiveModel);
create_lookup!(
    create_brand,
    brand::Entity,
    brand::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_vehicle_models,
    vehicle_model::Entity,
    vehicle_model::Model,
    vehicle_model::ActiveModel
);

pub async fn create_vehicle_model(
    Extension(ctx): Extension<ApiContext>,
    Json(vehicle_model): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    if vehicle_model.brand.is_none() {
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, "no brand id was provided").into_response());
    };

    let active_model = vehicle_model::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(vehicle_model.name),
        brand: sea_orm::Set(vehicle_model.brand.unwrap_or_default()),
    };

    LookupService::create::<vehicle_model::Entity, vehicle_model::Model, vehicle_model::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

get_lookup!(
    get_parish,
    parish::Entity,
    parish::Model,
    parish::ActiveModel
);
create_lookup!(
    create_parish,
    parish::Entity,
    parish::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_seclusion_statuses,
    seclusion_statuses::Entity,
    seclusion_statuses::Model,
    seclusion_statuses::ActiveModel
);
create_lookup!(
    create_seclusion_statuses,
    seclusion_statuses::Entity,
    seclusion_statuses::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_family_relationships,
    family_relationship::Entity,
    family_relationship::Model,
    family_relationship::ActiveModel
);
create_lookup!(
    create_family_relationships,
    family_relationship::Entity,
    family_relationship::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_novelties,
    novelty::Entity,
    novelty::Model,
    novelty::ActiveModel
);
create_lookup!(
    create_novelty,
    novelty::Entity,
    novelty::ActiveModel,
    CreateBasicLookUpDTO
);

pub async fn get_status_conditions(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let status_conditions = LookupService::find::<
        status_condition::Entity,
        status_condition::Model,
        status_condition::ActiveModel,
    >(ctx)
    .await?;

    debug!("{:?}", Json(&status_conditions));
    Ok((StatusCode::OK, Json(status_conditions)).into_response())
}

create_lookup!(
    create_band,
    band::Entity,
    band::ActiveModel,
    CreateBasicLookUpDTO
);
get_lookup!(get_bands, band::Entity, band::Model, band::ActiveModel);

create_lookup!(
    create_institution,
    institution::Entity,
    institution::ActiveModel,
    CreateBasicLookUpDTO
);
get_lookup!(
    get_institutions,
    institution::Entity,
    institution::Model,
    institution::ActiveModel
);

create_lookup!(
    create_profession,
    profession::Entity,
    profession::ActiveModel,
    CreateBasicLookUpDTO
);
get_lookup!(
    get_professions,
    profession::Entity,
    profession::Model,
    profession::ActiveModel
);

get_lookup!(get_bases, base::Entity, base::Model, base::ActiveModel);
create_lookup!(
    create_base,
    base::Entity,
    base::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(get_roles, role::Entity, role::Model, role::ActiveModel);

get_lookup!(
    get_persona_states,
    persona_state::Entity,
    persona_state::Model,
    persona_state::ActiveModel
);
create_lookup!(
    create_persona_state,
    persona_state::Entity,
    persona_state::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_document_types,
    document_type::Entity,
    document_type::Model,
    document_type::ActiveModel
);
create_lookup!(
    create_document_type,
    document_type::Entity,
    document_type::ActiveModel,
    CreateBasicLookUpDTO
);

get_lookup!(
    get_weapon_types,
    weapon_type::Entity,
    weapon_type::Model,
    weapon_type::ActiveModel
);

get_lookup!(
    get_weapon_brands,
    weapon_brand::Entity,
    weapon_brand::Model,
    weapon_brand::ActiveModel
);

get_lookup!(
    get_weapon_models,
    weapon_model::Entity,
    weapon_model::Model,
    weapon_model::ActiveModel
);

create_lookup!(
    create_weapon_type,
    weapon_type::Entity,
    weapon_type::ActiveModel,
    CreateBasicLookUpDTO
);

create_lookup!(
    create_weapon_brand,
    weapon_brand::Entity,
    weapon_brand::ActiveModel,
    CreateBasicLookUpDTO
);

create_lookup!(
    create_weapon_model,
    weapon_model::Entity,
    weapon_model::ActiveModel,
    CreateBasicLookUpDTO
);
