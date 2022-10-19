// @generated automatically by Diesel CLI.

diesel::table! {
    feature_flags (id) {
        id -> Uuid,
        name -> Varchar,
        config -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        hidden_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    role_groups (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Varchar>,
        config -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        hidden_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Varchar>,
        config -> Jsonb,
        role_group_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        hidden_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    system_configs (id) {
        id -> Uuid,
        name -> Varchar,
        config -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        hidden_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_groups (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Varchar>,
        config -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        hidden_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email_address -> Varchar,
        phone -> Nullable<Varchar>,
        external_provider_config -> Nullable<Jsonb>,
        config -> Nullable<Jsonb>,
        user_group_id -> Nullable<Uuid>,
        role_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        hidden_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(roles -> role_groups (role_group_id));
diesel::joinable!(users -> roles (role_id));
diesel::joinable!(users -> user_groups (user_group_id));

diesel::allow_tables_to_appear_in_same_query!(
    feature_flags,
    role_groups,
    roles,
    system_configs,
    user_groups,
    users,
);
