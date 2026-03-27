-- Add up migration script here

CREATE TABLE invoice_heads (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  customer_name TEXT NOT NULL,
  confirmed BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL
);

CREATE TRIGGER invoice_heads_updated_at
BEFORE UPDATE ON invoice_heads
FOR EACH ROW
EXECUTE FUNCTION on_update_timestamp();

CREATE TABLE invoice_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  invoice_head_id UUID NOT NULL REFERENCES invoice_heads(id),
  article_id UUID NOT NULL REFERENCES articles(id),
  ordinal INT NOT NULL,
  quantity NUMERIC(12,4) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(invoice_head_id, ordinal)
);

CREATE TRIGGER invoice_items_updated_at
BEFORE UPDATE ON invoice_items
FOR EACH ROW
EXECUTE FUNCTION on_update_timestamp();
