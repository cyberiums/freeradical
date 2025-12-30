-- Sets up the initial postgre-specific helpers
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $inner$
    BEGIN
        IF (NEW IS DISTINCT FROM OLD) AND (NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at) THEN
            NEW.updated_at := current_timestamp;
        END IF;
        RETURN NEW;
    END;
    $inner$ LANGUAGE plpgsql;');
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
        FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;
