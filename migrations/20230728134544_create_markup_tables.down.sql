-- Add down migration script here
-- Drop the 'comment' table and associated indexes
-- Drop the 'user' table
DROP TABLE IF EXISTS comment;

DROP TABLE IF EXISTS post;

DROP TABLE IF EXISTS annotation;

DROP TABLE IF EXISTS marker_3d;

DROP TABLE IF EXISTS point3d;

DROP TABLE IF EXISTS polyline3d;

DROP TABLE IF EXISTS point_3d;

DROP TABLE IF EXISTS polyline_3d;

DROP TABLE IF EXISTS session;

DROP TABLE IF EXISTS "user";

DROP TYPE IF EXISTS role;

DROP TYPE IF EXISTS role_enum;

DROP TYPE IF EXISTS rgba;

DROP TYPE IF EXISTS marker_3d_kind;

DROP TYPE IF EXISTS "vec3<f64>";

DROP TYPE IF EXISTS coords;

DROP TYPE IF EXISTS vec3_f64;

DROP TYPE IF EXISTS session;