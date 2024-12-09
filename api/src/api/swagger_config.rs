use super::routes::handle_snapshot;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi()]
pub struct ApiDoc;

pub async fn serve_swagger() -> SwaggerUi {
    let doc = generate_doc();

    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc)
}

pub fn save_doc_to_file() -> Result<(), std::io::Error> {
    let doc = generate_doc();

    let doc_json = match doc.to_json() {
        Ok(doc) => doc,
        Err(e) => {
            tracing::error!("Failed to convert doc to json: {}", e);
            std::process::exit(1);
        }
    };

    std::fs::write("./bindings/api-docs.json", doc_json)
}
pub fn generate_doc() -> utoipa::openapi::OpenApi {
    let mut doc: utoipa::openapi::OpenApi = ApiDoc::openapi();

    doc.merge(handle_snapshot::SnapshotDoc::openapi());

    doc
}
