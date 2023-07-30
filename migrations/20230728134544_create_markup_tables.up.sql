-- Add up migration script here
-- Add migration script here

create extension if not exists "uuid-ossp";

-- Create the Role enum type
-- create type Role as enum ('USER', 'SUPERUSER', 'ADMIN', 'MODERATOR');

-- Create the User table
create table users (
    id uuid primary key not null default (uuid_generate_v4()),
    username varchar(100) not null unique,
    email text not null check (email ~* '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$'), -- Corrected regex check
    password text not null check (length(password) >= 8 AND password ~ '[A-Z]' AND password ~ '[a-z]' AND password ~ '[0-9]' AND password ~ '[^A-Za-z0-9]'),
    -- role Role default 'USER',
    updated_at timestamp with time zone default now()
);

create table comments (
    id uuid primary key not null default (uuid_generate_v4()),
    title varchar(255) not null unique,
    rich_text text not nulL,
    category varchar(100),
    published boolean default false,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now(),
    user_id uuid not nulL,
    foreign key (user_id) references users (id) on delete cascade
);