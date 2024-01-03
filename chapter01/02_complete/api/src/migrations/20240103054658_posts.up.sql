-- Add up migration script here
create extension if not exists "uuid-ossp";

create table if not exists posts (
    "id"  serial primary key,
    "uuid" uuid default uuid_generate_v4() not null unique,
    "user_id" uuid not null references "users"("uuid") on delete cascade,
    "title" varchar(100) not null,
    "image_src" text not null,
    "thumbnail_src" text[],
    "description" text not null,
    "brand" varchar(100) not null,
    "category" varchar(100) not null,
    "size"  jsonb not null,
    "price" bigint not null default 0,
    "count_in_stock" bigint not null default 0,
    "rating" double precision not null default 0,
    "num_reviews" bigint not null default 0,
    "sale" bigint not null default 0,
    "free_shipping" bool not null default false,
    "delivery_fee" bigint not null default 0,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);
--""를 안쓰면 모두 소문자로 인식한다.  