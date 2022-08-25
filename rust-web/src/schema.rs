table! {
    gh_configurations (conf_id) {
        conf_id -> Int4,
        active -> Bool,
        server -> Text,
        org -> Text,
        repo -> Text,
        token -> Nullable<Text>,
        jenkins -> Nullable<Text>,
    }
}

table! {
    gh_pulls (conf_id, pull_id) {
        conf_id -> Int4,
        pull_id -> Int4,
        number -> Int4,
        title -> Text,
        url -> Text,
        state -> Text,
    }
}

table! {
    gh_reviews (conf_id, pull_id, review_id) {
        conf_id -> Int4,
        pull_id -> Int4,
        review_id -> Int4,
        state -> Text,
    }
}

table! {
    gh_statuses (conf_id, pull_id, status_id) {
        conf_id -> Int4,
        pull_id -> Int4,
        status_id -> Int4,
        description -> Text,
        target_url -> Text,
        state -> Text,
    }
}

table! {
    jenkins_builds (conf_id, pull_id, build_id) {
        conf_id -> Int4,
        pull_id -> Int4,
        build_id -> Int4,
        name -> Text,
        status -> Text,
        start_time_millis -> Int4,
        duration_millis -> Int4,
    }
}

table! {
    jenkins_stages (conf_id, pull_id, build_id, stage_id) {
        conf_id -> Int4,
        pull_id -> Int4,
        build_id -> Int4,
        stage_id -> Int4,
        name -> Text,
        status -> Text,
        start_time_millis -> Int4,
        duration_millis -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    gh_configurations,
    gh_pulls,
    gh_reviews,
    gh_statuses,
    jenkins_builds,
    jenkins_stages,
);
