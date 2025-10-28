-- Task Dependencies Table
CREATE TABLE task_dependencies (
        task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
        depends_on UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        PRIMARY KEY (task_id, depends_on),
        -- Prevent a task from depending on itself
        CONSTRAINT no_self_dependency CHECK (task_id != depends_on)
        );

-- Index for efficient queries
CREATE INDEX idx_task_dependencies_task ON task_dependencies(task_id);
CREATE INDEX idx_task_dependencies_depends ON task_dependencies(depends_on);

    -- Audit Logs Table
    CREATE TABLE audit_logs (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id),
            action VARCHAR(50) NOT NULL,
            resource_type VARCHAR(50) NOT NULL,
            resource_id UUID NOT NULL,
            old_values JSONB,
            new_values JSONB,
            ip_address INET,
            user_agent TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

    -- Indexes for audit queries
    CREATE INDEX idx_audit_logs_user ON audit_logs(user_id);
    CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
    CREATE INDEX idx_audit_logs_action ON audit_logs(action);
    CREATE INDEX idx_audit_logs_created ON audit_logs(created_at DESC);

-- View for task dependencies with names (useful for debugging)
    CREATE VIEW task_dependency_view AS
    SELECT
    td.task_id,
    t1.title as task_title,
    td.depends_on,
    t2.title as depends_on_title,
    t2.status as dependency_status
    FROM task_dependencies td
    JOIN tasks t1 ON td.task_id = t1.id
    JOIN tasks t2 ON td.depends_on = t2.id;
