CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY, 
  name TEXT NOT NULL, 
  created_date TEXT,
  finished_date TEXT,
  due_date TEXT
);

CREATE TABLE IF NOT EXISTS tags (
  id INTEGER PRIMARY KEY, 
  name TEXT NOT NULL,
  UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS tagged (
  task_id INTEGER,
  tag_id INTEGER,
  FOREIGN KEY(task_id) REFERENCES tasks(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id)
  UNIQUE(task_id, tag_id)
);
