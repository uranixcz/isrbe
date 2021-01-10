SELECT resource.id, resource.name, 999999, resource_type.res_type_name,
(SELECT COUNT(resource_location.id) FROM resource_location
	JOIN resource_param ON resource_param.id = resource_location.res_param_id
	WHERE resource.id = resource_param.res_id AND resource_param.is_movable = 1) as "locations",
(SELECT COUNT(resource_param.id) FROM resource_param
    WHERE resource.id = resource_param.res_id) as "parameters"
FROM resource JOIN resource_type ON resource.type_id = resource_type.res_type_id
ORDER BY resource.id