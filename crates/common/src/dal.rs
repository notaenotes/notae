use crate::database::get_connection;
use sea_orm::entity::{EntityTrait, PrimaryKeyTrait};

pub async fn get_entities<T: EntityTrait>() -> Vec<<T as EntityTrait>::Model> {
    let connection = get_connection().await.unwrap_or_default();
    T::find().all(&connection).await.unwrap_or_default()
}

pub async fn find_entity_by_id<T: EntityTrait>(
    id_entity: <T::PrimaryKey as PrimaryKeyTrait>::ValueType,
) -> <T as EntityTrait>::Model {
    let connection = get_connection().await.unwrap_or_default();
    T::find_by_id(id_entity)
        .one(&connection)
        .await
        .unwrap_or_default()
        .unwrap()
}

// pub async fn find_entity_by_id_with_related<T: EntityTrait, R: EntityTrait>(
//     id_entity: <T::PrimaryKey as PrimaryKeyTrait>::ValueType,
// ) {
//     let connection = get_connection().await.unwrap_or_default();
//     let model = T::find_by_id(id_entity).find_with_related(R);
// }
