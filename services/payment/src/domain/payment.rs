use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentType {
    PixIn,
    PixOut,
    StablecoinDeposit,
    StablecoinWithdraw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub payment_type: PaymentType,
    pub status: PaymentStatus,
    pub external_reference: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Payment {
    pub fn new(
        wallet_id: Uuid,
        amount: Decimal,
        currency: String,
        payment_type: PaymentType,
        external_reference: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            wallet_id,
            amount,
            currency,
            payment_type,
            status: PaymentStatus::Pending,
            external_reference,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn complete(&mut self) {
        self.status = PaymentStatus::Completed;
        self.updated_at = Utc::now();
    }

    pub fn fail(&mut self) {
        self.status = PaymentStatus::Failed;
        self.updated_at = Utc::now();
    }
}
