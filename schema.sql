CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create enum type only if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type WHERE typname = 'coupon_discount_type'
    ) THEN
        CREATE TYPE coupon_discount_type AS ENUM ('percent', 'fixed');
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    description VARCHAR(300),
    price INTEGER NOT NULL,
    stock INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    
    CONSTRAINT chk_price_min CHECK (price >= 0.01),
    CONSTRAINT chk_stock_range CHECK (stock BETWEEN 0 AND 999999)
);

CREATE TABLE IF NOT EXISTS coupons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(100) NOT NULL UNIQUE,
    type coupon_discount_type NOT NULL,
    value INTEGER NOT NULL,
    one_shot BOOLEAN NOT NULL DEFAULT false,
    max_uses INTEGER,
    uses_count INTEGER NOT NULL DEFAULT 0,
    valid_from TIMESTAMP NOT NULL,
    valid_until TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    
    CONSTRAINT chk_value_range CHECK (
        (type = 'percent' AND value BETWEEN 100 AND 8000) OR
        (type = 'fixed' AND value > 0)
    ),
    CONSTRAINT chk_validity_period CHECK (
        valid_until > valid_from AND 
        valid_until <= valid_from + INTERVAL '5 years'
    ),
    CONSTRAINT chk_max_uses CHECK (
        (one_shot = true AND max_uses IS NULL) OR 
        (one_shot = false AND (max_uses IS NULL OR max_uses > 0))
    )
);

CREATE TABLE IF NOT EXISTS product_coupon_applications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    coupon_id UUID NOT NULL REFERENCES coupons(id) ON DELETE CASCADE,
    applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    removed_at TIMESTAMP,
    
    CONSTRAINT chk_application_times CHECK (
        applied_at <= COALESCE(removed_at, CURRENT_TIMESTAMP)
    )
);

-- Create index only if it doesn't exist
CREATE UNIQUE INDEX IF NOT EXISTS idx_unique_active_coupon 
ON product_coupon_applications (product_id) 
WHERE removed_at IS NULL;

CREATE UNIQUE INDEX IF NOT EXISTS products_name_unique_idx ON products (name);
