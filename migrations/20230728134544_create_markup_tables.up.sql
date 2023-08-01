-- Add up migration script here
create extension if not exists "uuid-ossp";

create extension if not exists "postgis";

-- User
create type role as enum ('user', 'superuser', 'admin', 'moderator');

create table "user" (
    user_id uuid primary key not null default (uuid_generate_v4()),
    username varchar(100) not null unique,
    email text not null,
    password_hash text not null,
    role role default 'user',
    updated_at timestamp with time zone default now()
);


-- utility types: coordinates
create type vec3_f64  as (
    x double precision,
    y double precision,
    z double precision
);

create domain coords AS vec3_f64[];

-- markers
create table point_3d (
    point_id uuid primary key not null default (uuid_generate_v4()),
    coord vec3_f64
);

create table polyline_3d (
    polyline_id uuid primary key not null default (uuid_generate_v4()),
    coords coords
);

create type marker_3d_kind as enum ('polyline3d', 'point3d');

CREATE TABLE marker_3d (
    marker_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    marker_3d_kind marker_3d_kind not null,
    point_3d_id uuid references point_3d(point_id) CHECK (
        (
            marker_3d_kind = 'point3d'
            AND point_3d_id IS NOT NULL
        )
        OR (
            marker_3d_kind <> 'point3d'
            AND point_3d_id IS NULL
        )
    ),
    polyline_3d_id uuid references polyline_3d(polyline_id) CHECK (
        (
            marker_3d_kind = 'polyline3d'
            AND polyline_3d_id IS NOT NULL
        )
        OR (
            marker_3d_kind <> 'polyline3d'
            AND polyline_3d_id IS NULL
        )
    )
);



create table post (
    post_id uuid primary key default uuid_generate_v4(),
    user_id uuid not null references "user"(user_id) on delete cascade,
    marker_id uuid references marker_3d(marker_id),
    rich_text text not null,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);

create index on post(created_at desc);

create table comment (
    comment_id uuid primary key not null default (uuid_generate_v4()),
    post_id uuid not null references post(post_id) on delete cascade,
    user_id uuid not null references "user"(user_id) on delete cascade,
    marker_id uuid references marker_3d(marker_id),
    title varchar(255) not null unique,
    rich_text text not null,
    published boolean default false,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);

create index on comment(post_id, created_at);