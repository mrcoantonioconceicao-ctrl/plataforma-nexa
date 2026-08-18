// ~/nexavor/services/wallet/src/main.rs

mod domain;
mod infrastructure;

use domain::wallet::Wallet;
use infrastructure::postgres_wallet_repository::PostgresWalletRepository;
use rust_decimal_macros::dec;
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    println!("Inicializando Nexavor Wallet Service com PostgreSQL...");

    // URL padrão de conexão do workspace para testes locais / docker
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/nexavor".to_string());

    let pool = match PgPool::connect(&database_url).await {
        Ok(p) => {
            println!("Conexão com o PostgreSQL estabelecida com sucesso!");
            p
        }
        Err(e) => {
            println!(
                "Aviso: Não foi possível conectar ao banco local ({}), rodando modo simulado/mock.",
                e
            );
            return;
        }
    };

    let repo = PostgresWalletRepository::new(pool);

    let user_id = Uuid::new_v4();
    let mut wallet = Wallet::new(user_id, "BRL".to_string(), dec!(1000.00));

    if let Err(e) = repo.save(&wallet).await {
        println!("Erro ao salvar carteira: {}", e);
        return;
    }
    println!("Carteira criada e salva no banco! ID: {}", wallet.id);

    // Movimentação de saldo
    if let Err(e) = wallet.credit(dec!(500.00)) {
        println!("Erro ao creditar: {}", e);
    } else {
        println!("Credito aplicado. Novo saldo: {}", wallet.balance);
    }

    if let Err(e) = repo.save(&wallet).await {
        println!("Erro ao atualizar carteira: {}", e);
    } else {
        println!("Estado atualizado persistido com sucesso no banco de dados.");
    }
}
