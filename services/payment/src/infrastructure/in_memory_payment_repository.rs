// ~/nexavor/services/payment/src/infrastructure/in_memory_payment_repository.rs

use crate::domain::payment::{Payment, PaymentStatus};
use crate::domain::payment_repository::PaymentRepository;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub struct InMemoryPaymentRepository {
    payments: Mutex<HashMap<Uuid, Payment>>,
}

impl InMemoryPaymentRepository {
    pub fn new() -> Self {
        Self {
            payments: Mutex::new(HashMap::new()),
        }
    }
}

impl PaymentRepository for InMemoryPaymentRepository {
    fn save(&self, payment: Payment) {
        let mut map = self.payments.lock().unwrap();
        map.insert(payment.id, payment);
    }

    fn find_by_id(&self, id: Uuid) -> Option<Payment> {
        let map = self.payments.lock().unwrap();
        map.get(&id).cloned()
    }

    fn update_status(&self, id: Uuid, status: PaymentStatus) {
        let mut map = self.payments.lock().unwrap();
        if let Some(payment) = map.get_mut(&id) {
            payment.status = status;
        }
    }
}
