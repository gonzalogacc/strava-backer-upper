-- Convert integer columns to BIGINT to match i64 in Rust
ALTER TABLE "token" ALTER COLUMN "expires_at" TYPE INT8;
ALTER TABLE "token" ALTER COLUMN "expires_in" TYPE INT8;

-- Restore the Primary Key constraint on 'id'
ALTER TABLE "token" ADD PRIMARY KEY (id);