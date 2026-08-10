use crate::domain::payment::{Payment, PaymentStatus};
use uuid::Uuid;

pub trait PaymentRepository: Send + Sync {
    fn save(&self, payment: Payment);
    fn find_by_id(&self, id: Uuid) -> Option<Payment>;
    fn update_status(&self, id: Uuid, status: PaymentStatus);
}
