create type status as enum (
	'PENDING',
	'IN_PROGRESS',
	'DONE',
	'CANCELLED'
);

create table todos (
	title varchar(255),
	description text,
	created_at timestamptz,
	last_modified timestamptz,
	status status DEFAULT 'PENDING'
);
