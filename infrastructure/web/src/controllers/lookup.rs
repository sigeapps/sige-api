use axum::extract::State;

use crate::state::AppState;

pub async fn get_lookup_registers(
    State(app_state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) {
}
