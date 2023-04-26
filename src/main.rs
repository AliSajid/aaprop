use aaprop::create_router;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = create_router();

    Ok(router.into())
}
