use crate::models::{ prelude::*, pay::CadenceOption };

#[Object]
impl Pay {
    async fn cadence(&self) -> &CadenceOption {
        &self.cadence
    }

    async fn min_base_pay(&self) -> &u32 {
        &self.min_base_pay
    }
}
