-- Add up migration script here

CREATE TABLE goods_receipt_heads (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  supplier_name TEXT NOT NULL,
  confirmed BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL
);

CREATE TRIGGER goods_receipt_heads_updated_at
BEFORE UPDATE ON goods_receipt_heads
FOR EACH ROW
EXECUTE FUNCTION on_update_timestamp();

CREATE TABLE goods_receipt_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  goods_receipt_head_id UUID NOT NULL REFERENCES goods_receipt_heads(id),
  article_id UUID NOT NULL REFERENCES articles(id),
  ordinal INT NOT NULL,
  quantity NUMERIC(12,4) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER goods_receipt_items_updated_at
BEFORE UPDATE ON goods_receipt_items
FOR EACH ROW
EXECUTE FUNCTION on_update_timestamp();
