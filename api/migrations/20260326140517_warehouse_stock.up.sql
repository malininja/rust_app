-- Add up migration script here

CREATE TABLE warehouse_stocks (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  article_id UUID UNIQUE NOT NULL REFERENCES articles(id),
  quantity NUMERIC(12, 4) NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TRIGGER warehouse_stock_updated_at
BEFORE UPDATE ON warehouse_stocks
FOR EACH ROW
EXECUTE FUNCTION on_update_timestamp();
