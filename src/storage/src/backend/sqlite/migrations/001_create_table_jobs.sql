CREATE TABLE IF NOT EXISTS jobs (
  id TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  task TEXT NOT NULL,
  args TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'scheduled' CHECK (
    status IN (
      'claimed', 'pending', 'scheduled',
      'running', 'completed', 'failed',
      'cancelled'
    )
  ),
  claimed_at INTEGER,
  claimed_by TEXT,
  scheduled_at INTEGER NOT NULL,
  started_at INTEGER,
  completed_at INTEGER,
  errors TEXT NOT NULL DEFAULT '[]',
  result TEXT,
  attempts INTEGER NOT NULL DEFAULT 0,
  max_attempts INTEGER NOT NULL,
  CHECK (attempts >= 0),
  CHECK (max_attempts > 0),
  CHECK (attempts <= max_attempts)
);

CREATE INDEX IF NOT EXISTS idx_jobs_status_scheduled_at ON jobs (status, scheduled_at ASC);

CREATE INDEX IF NOT EXISTS idx_jobs_scheduled_at ON jobs (scheduled_at ASC);

CREATE INDEX IF NOT EXISTS idx_jobs_name ON jobs (name);
