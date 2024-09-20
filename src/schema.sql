CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY, 
  name TEXT NOT NULL, 
  done BOOLEAN NOT NULL DEFAULT 0,
  due DATE
);

CREATE TABLE IF NOT EXISTS tags (
                        id INTEGER PRIMARY KEY, 
                        name TEXT NOT NULL
                    );

CREATE TABLE IF NOT EXISTS tagged (
                task_id INTEGER,
                tag_id INTEGER
            );
