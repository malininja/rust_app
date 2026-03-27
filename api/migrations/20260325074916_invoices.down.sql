-- Add down migration script here

DROP TRIGGER invoice_items_updated_at ON invoice_items;

DROP TABLE invoice_items;

DROP TRIGGER invoice_heads_updated_at ON invoice_heads;

DROP TABLE invoice_heads;