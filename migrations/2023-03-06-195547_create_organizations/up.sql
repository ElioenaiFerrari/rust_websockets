CREATE TABLE organizations (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  description VARCHAR, 
  infra_location VARCHAR NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT false,
  is_emailing_enabled BOOLEAN NOT NULL DEFAULT false, 
  jira_id INTEGER, 
  name VARCHAR UNIQUE NOT NULL,
  parent_id INTEGER
)