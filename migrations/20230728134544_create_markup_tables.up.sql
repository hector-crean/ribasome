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

create table if not exists session (
    session_token BYTEA PRIMARY KEY,
    user_id uuid references "user"(user_id) on delete cascade
);

-- utility types: coordinates
create type vec3 as (
    x double precision,
    y double precision,
    z double precision
);

create type quat as (
    x double precision,
    y double precision,
    z double precision,
    w double precision
);

create domain vec3_array AS vec3 [];

-- markers
create table point_3d (
    point_id uuid primary key not null default (uuid_generate_v4()),
    coord vec3
);

create table polyline_3d (
    polyline_id uuid primary key not null default (uuid_generate_v4()),
    coords vec3_array
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

create domain posts AS post [];

create table glb (
    glb_id uuid primary key not null default (uuid_generate_v4()),
    url varchar not null,
    title varchar not null,
    translation vec3,
    rotation quat,
    scale vec3,
    published boolean default false,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);

create domain glbs AS glb [];

create table scene_3d (
    scene_id uuid primary key not null default (uuid_generate_v4()),
    title varchar(255) not null,
    published boolean default false,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now(),
    glbs glbs,
    post posts
);