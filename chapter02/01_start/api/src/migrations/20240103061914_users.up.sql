-- Add up migration script here
create extension if not exists "uuid-ossp";

create table if not exists users (
    "id"  serial primary key,
    "uuid" uuid default uuid_generate_v4() not null unique,
    "name" varchar(50) not null,
    "email" varchar(100) not null unique,
    "password" varchar(100) not null,
    "is_admin" boolean not null default false,
    "google_id" varchar(100) unique,
    "naver_id" varchar(100) unique,
    "kakao_id" varchar(100) unique,
    "email_token" text,
    "is_verified" boolean not null default false,
    "pw_email_address" varchar(100),
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);
