-- Add up migration script here
create extension if not exists "uuid-ossp";

create extension if not exists "postgis";

-- supabas already had a roles type
-- create type Role as enum ('USER', 'SUPERUSER', 'ADMIN', 'MODERATOR');
create table "user" (
    user_id uuid primary key not null default (uuid_generate_v4()),
    username varchar(100) not null unique,
    email text not null,
    role Role default 'USER',
    updated_at timestamp with time zone default now()
);

-- create type rgba as (
--     r integer not null as (
--         r >= 0
--         AND r <= 255
--     ),
--     g integer not null as (
--         g >= 0
--         AND g <= 255
--     ),
--     b integer not null as (
--         b >= 0
--         AND b <= 255
--     ),
--     a integer not null as (
--         a >= 0
--         AND a <= 255
--     )
-- );
create type marker_3d_kind as enum ('POLYLINE_3D', 'POINT_3D');

create type "vec3<f64>" as (
    x double precision,
    y double precision,
    z double precision
);

create table point3d (
    point_id uuid primary key not null default (uuid_generate_v4()),
    coord "vec3<f64>" []
);

create table polyline3d (
    polyline_id uuid primary key not null default (uuid_generate_v4()),
    coords "vec3<f64>" []
);

CREATE TABLE marker_3d (
    marker_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    marker_3d_kind marker_3d_kind NOT NULL,
    point_3d uuid references point3d(point_id) CHECK (
        (
            marker_3d_kind = 'POINT_3D'
            AND point_3d IS NOT NULL
        )
        OR (
            marker_3d_kind <> 'POINT_3D'
            AND point_3d IS NULL
        )
    ),
    polyline_3d uuid references polyline3d(polyline_id) CHECK (
        (
            marker_3d_kind = 'POLYLINE_3D'
            AND polyline_3d IS NOT NULL
        )
        OR (
            marker_3d_kind <> 'POLYLINE_3D'
            AND polyline_3d IS NULL
        )
    )
);

create table annotation (
    annotation_id uuid primary key not null default (uuid_generate_v4()),
    marker_id uuid not null references marker_3d(marker_id)
);

create table post (
    post_id uuid primary key default uuid_generate_v4(),
    user_id uuid not null references "user"(user_id) on delete cascade,
    annotation_id uuid references annotation(annotation_id),
    rich_text text not null,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);

create index on post(created_at desc);

create table comment (
    comment_id uuid primary key not null default (uuid_generate_v4()),
    post_id uuid not null references post(post_id) on delete cascade,
    user_id uuid not null references "user"(user_id) on delete cascade,
    title varchar(255) not null unique,
    rich_text text not null,
    category varchar(100),
    published boolean default false,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);

create index on comment(post_id, created_at);