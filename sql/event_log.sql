CREATE TABLE IF NOT EXISTS event_log (
  wsid varchar(32) NOT NULL encode zstd distkey, -- widget session id
  usid varchar(32) encode zstd, -- user session id
  eid integer NOT NULL encode mostly16, -- event id
  wid integer NOT NULL encode zstd, -- widget id
  mid integer encode zstd, -- medium id
  pid integer encode zstd, -- product id
  ts integer NOT NULL encode delta sortkey -- timestamp
);
