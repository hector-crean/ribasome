-- scene_3d
drop table if exists scene_3d;

-- glb
drop domain if exists glbs;
drop table if exists glb;

-- comment
drop index if exists comment_post_id_created_at_idx;
drop table if exists comment;

-- post
drop index if exists post_created_at_desc_idx;
drop domain if exists posts;
drop table if exists post;

-- marker_3d
drop table if exists marker_3d;
drop type if exists marker_3d_kind;

-- polyline_3d
drop table if exists polyline_3d;

-- point_3d
drop table if exists point_3d;

-- utility types: coordinates
drop domain if exists vec3_array;
drop type if exists quat;
drop type if exists vec3;

-- session
drop table if exists session;

-- user
drop table if exists "user";
drop type if exists role;

-- postgis & uuid-ossp extensions
-- drop extension if exists "postgis";
-- drop extension if exists "uuid-ossp";