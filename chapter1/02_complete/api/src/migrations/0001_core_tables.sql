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
    "uuid" uuid default uuid_generate_v4() not null unique,
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
    "rating" double precision not null default 0,
    "num_reviews" bigint not null default 0,
    "sale" bigint not null default 0,
    "free_shipping" bool not null default false,
    "delivery_fee" bigint not null default 0,
    "created_at" timestamp not null default now(),
    "updated_at" timestamp not null default now(),

    constraint fk_user foreign key ("user_id") references "users" ("uuid") on delete cascade
);


create table if not exists addresses (
    "id"  serial primary key,
    "user_id" uuid not null,
    "recipient" text not null,
    "shipping_address" text not null,
    "postcode" varchar(50),
    "address" varchar(100),
    "detail1" varchar(100),
    "detail2" varchar(100),
    "phone1" varchar(10),
    "phone2" varchar(10),
    "phone3"  varchar(10),
    "created_at" timestamp not null default now(),
    "updated_at" timestamp not null default now(),

    constraint fk_user foreign key ("user_id") references "users" ("uuid") on delete cascade
);

create table if not exists carts (
    "id"  serial primary key,
    "user_id" uuid not null,
    "order_items" json[],
    "created_at" timestamp not null default now(),
    "updated_at" timestamp not null default now(),

    constraint fk_user foreign key ("user_id") references "users" ("uuid") on delete cascade
);

create table if not exists orders (
    "id"  serial primary key,
    "order_id" varchar(100) not null,
    "user_id" uuid not null,
    "user_name" varchar(100) not null,
    "order_items" json[],
    "total_price" bigint not null default 0,
    "shipping_address" text not null,
    "recipient" varchar(100) not null,
    "postcode" varchar(50),
    "full_phone_number" varchar(30),
    "address" varchar(100),
    "detail1" varchar(100),
    "detail2" varchar(100),
    "phone1" varchar(10),
    "phone2" varchar(10),
    "phone3"  varchar(10),
    "payment_method"  varchar(50),
    "payment_result"  json,
    "shipping_price"  bigint not null default 0,
    "is_paid"  boolean,
    "is_delivered"  boolean,
    "paid_at"  timestamp not null default now(),
    "delivered_at"  timestamp not null default now(),
    "tid"  varchar(100),
    "pg_token" varchar(300),
    "payment_method_type" varchar(100),
    "card_info" json,
    "tarcking_number" text[],
    "tracking_contents" json[],
    "review_check" text[],
    "created_at" timestamp not null default now(),
    "updated_at" timestamp not null default now(),

    constraint fk_user foreign key ("user_id") references "users" ("uuid") on delete cascade
);

create table if not exists order_cancels (
    "id"  serial primary key,
    "order_id" varchar(100) not null,
    "user_id" uuid not null,
    "order_items" json[],
    "total_price" bigint not null default 0,
    "shipping_address" text not null,
    "payment_method"  varchar(50),
    "payment_result"  json,
    "is_paid"  boolean,
    "paid_at"  timestamp not null default now(),
    "payment_method_type" varchar(100),
    "card_info" json,
    "cancel_data" json,
    "cancel_date" timestamp,
    "created_at" timestamp not null default now(),
    "updated_at" timestamp not null default now(),

    constraint fk_user foreign key ("user_id") references "users" ("uuid") on delete cascade
);

create table if not exists create_reviews (
    "id"  serial primary key unique,
    "product_id" uuid not null,
    "user_id" uuid not null,
    "order_id" serial,
    "user_name" varchar(100) not null,
    "rating" double precision not null default 0,
    "comments" text not null,
    "created_at" timestamp not null default now(),
    "updated_at" timestamp not null default now(),

    constraint fk_user foreign key ("user_id") references "users" ("uuid") on delete cascade,
    constraint fk_post foreign key ("product_id") references "posts" ("uuid") on delete cascade,
    constraint fk_order foreign key ("order_id") references "orders" ("id") on delete cascade
);

create table if not exists create_tokens (
    "id"  serial primary key,
    "uuid" uuid default uuid_generate_v4() not null unique,
    "refresh_token" text,
    "client_ip" varchar(100),
    "created_at" timestamp not null default now(),
    "updated_at" timestamp not null default now()
);