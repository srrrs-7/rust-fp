CREATE TABLE IF NOT EXISTS tasks_users (
    task_id UUID NOT NULL,
    user_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (task_id, user_id),
    CONSTRAINT fk_tasks_users_task FOREIGN KEY (task_id) REFERENCES tasks(task_id) ON DELETE CASCADE,
    CONSTRAINT fk_tasks_users_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

DO $$
BEGIN
    IF to_regclass('"_TasksToUser"') IS NOT NULL THEN
        INSERT INTO tasks_users (task_id, user_id)
        SELECT "A", "B"
        FROM "_TasksToUser"
        ON CONFLICT DO NOTHING;

        DROP TABLE IF EXISTS "_TasksToUser";
    END IF;
END $$;
