use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_shelves: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelves: usize,
}

pub async fn elf(body: String) -> Json<ElfCount> {
    let elf = body.matches("elf").count();
    let elf_shelves = body.matches("elf on a shelf").count();
    let shelves = body.matches("shelf").count() - elf_shelves;
    Json(ElfCount {
        elf,
        elf_shelves,
        shelves,
    })
}
