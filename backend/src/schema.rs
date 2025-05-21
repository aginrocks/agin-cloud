#![allow(dead_code, unused_imports, unused_variables)]

use async_trait::async_trait;
use axum_oidc::OidcClaims;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId, RecordIdKey, Surreal, engine::any::Any};
use utoipa::ToSchema;
use visible::StructFields;

use crate::{GroupClaims, state::SurrealDb};

macro_rules! database_object {
    ($name:ident { $($field:tt)* }$(, $($omitfield:ident),*)?) => {
        #[derive(Partial, Debug, Serialize, Deserialize, Clone)]
        #[partial(omit(id $(, $($omitfield),* )?), derive(Debug, Serialize, Deserialize, Clone))]
        #[StructFields(pub)]
        pub struct $name {
            $($field)*
        }
    };
}

database_object!(User {
    id: RecordId,
    subject: String,
    name: String,
    email: String,
});

database_object!(Link {
    id: RecordId,
    url: String,
});

database_object!(Shortcut {
    id: RecordId,
    link: String,
});

database_object!(ExpandsTo {
    id: RecordId,

    #[serde(rename = "in")]
    shortcut: RecordId,

    #[serde(rename = "out")]
    object: RecordId,
});

database_object!(
    Created {
        id: RecordId,

        #[serde(rename = "in")]
        user: RecordId,

        #[serde(rename = "out")]
        object: RecordId,

        timestamp: Datetime,
    },
    timestamp
);

impl From<OidcClaims<GroupClaims>> for PartialUser {
    fn from(claims: OidcClaims<GroupClaims>) -> Self {
        Self {
            subject: claims.subject().to_string(),
            name: claims.name().unwrap().get(None).unwrap().to_string(),
            email: claims.email().unwrap().to_string(),
        }
    }
}
