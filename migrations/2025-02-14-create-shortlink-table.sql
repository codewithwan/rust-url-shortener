DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_tables WHERE tablename = 'shortlink') THEN
        CREATE TABLE shortlink (
            id SERIAL PRIMARY KEY,
            short_code VARCHAR(8) NOT NULL UNIQUE,
            original_url TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    END IF;
END $$;
