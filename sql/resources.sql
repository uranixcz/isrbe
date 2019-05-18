SELECT resource.res_id, resource.res_name, resource_type.res_type_name,
(SELECT COUNT(res_loc_id) FROM resource_location WHERE resource.res_id = resource_location.res_id) as "locations",
(SELECT SUM(loc_val) FROM resource_location WHERE resource.res_id = resource_location.res_id) as "total quantity"
FROM resource JOIN resource_type ON resource.res_type_id = resource_type.res_type_id ORDER BY res_id