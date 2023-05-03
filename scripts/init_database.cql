CREATE KEYSPACE axum_api
    WITH replication = {
        'class': 'SimpleStrategy',
        'replication_factor': 1
    };

CREATE TABLE axum_api.users (
    email TEXT PRIMARY KEY,
    password_hash TEXT,
    password_salt TEXT,
);
