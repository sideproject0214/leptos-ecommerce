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
    "created_at" timestamp not null default now(),
    "updated_at" timestamp not null default now()
);

create table if not exists posts (
    "id"  serial primary key,
    "uuid" uuid default uuid_generate_v4() not null,
    "user_id" uuid not null,
    "title" varchar(100),
    "image_src" text not null,
    "thumbnail_src" text[],
    "description" text not null,
    "brand" varchar(100) not null,
    "category" varchar(100) not null,
    "size"  jsonb not null,
    "price" bigint not null default 0,
    "count_in_stock" bigint not null default 0,
    "rating" DOUBLE PRECISION NOT NULL DEFAULT 0,
    "num_reviews" bigint not null default 0,
    "sale" bigint not null default 0,
    "free_shipping" bool not null default false,
    "delivery_fee" bigint not null default 0,
    "created_at" timestamp not null default now(),
    "updated_at" timestamp not null default now(),

    constraint fk_user foreign key ("user_id") references "users" ("uuid") on delete cascade
);