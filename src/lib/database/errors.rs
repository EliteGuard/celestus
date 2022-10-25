use ::thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Failed to generate database URL!")]
    URLGenerationFailed,
    #[error("Failed to create database connection pool!")]
    PoolCreationFailed,
    #[error(
        "Failed to connect to database! Please check credentials in the .env file and try again!"
    )]
    ConnectFailed,
    #[error("Failed to seed data!")]
    SeedFailed,
    #[error("Failed to select data!")]
    DataSelectFailed,
    #[error("Failed to create data!")]
    DataCreateFailed,
    #[error("Failed to update data!")]
    DataUpdateFailed,
    #[error("Failed to delete data!")]
    DataDeleteFailed,
    #[error("Data corruption attempt!")]
    DataCorruptionAttempt,
    #[error("Failed to recover from an unsuccessful operation!")]
    RecoveryFailed,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum SeedDatabaseError {
    #[error("Seed corruption attempt!")]
    SeedCorruptionAttempt,
    #[error("Failed to seed system_configs!")]
    SeedSystemConfigsFailed,
    #[error("Failed to seed feature_flags!")]
    SeedFeatureFlagsFailed,
    #[error("Failed to seed role_groups!")]
    SeedRoleGroupsFailed,
    #[error("Failed to seed roles!")]
    SeedRolesFailed,
    #[error("Failed to seed user_groups!")]
    SeedUserGroupsFailed,
    #[error("Failed to seed users!")]
    SeedUsersFailed,
}
