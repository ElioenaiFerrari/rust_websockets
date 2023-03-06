// @generated automatically by Diesel CLI.

diesel::table! {
    organizations (id) {
        id -> Nullable<Integer>,
        description -> Nullable<Text>,
        infra_location -> Text,
        is_active -> Bool,
        is_emailing_enabled -> Bool,
        jira_id -> Nullable<Integer>,
        name -> Text,
        parent_id -> Nullable<Integer>,
    }
}
