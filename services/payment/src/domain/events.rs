// ~/nexavor/services/payment/src/domain/events.rs

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentDomainEvent {
    PaymentInitiated {
        payment_id: Uuid,
        wallet_id: Uuid,
        amount: Decimal,
        currency: String,
        timestamp: DateTime<Utc>,
    },
    PaymentCompleted {
        payment_id: Uuid,
        wallet_id: Uuid,
        amount: Decimal,
        currency: String,
        timestamp: DateTime<Utc>,
    },
    PaymentFailed {
        payment_id: Uuid,
        wallet_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
}
