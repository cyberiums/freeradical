
/// Soft delete customer by setting deleted_at timestamp
pub async fn soft_delete_customer(
    pool: web::Data<DbPool>,
    customer_id: i32,
) -> Result<(), CustomHttpError> {
    use crate::schema::crm_customers::dsl::*;
    
    web::block(move || -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        
        diesel::update(crm_customers.filter(id.eq(customer_id)))
            .set(deleted_at.eq(Some(chrono::Utc::now().naive_utc())))
            .execute(&mut conn)?;
        
        Ok(())
    })
    .await
    .map_err(|e| CustomHttpError::InternalServerError(format!("Block error: {}", e)))?
    .map_err(|e| CustomHttpError::DatabaseError(e.to_string()))?;
    
    Ok(())
}
