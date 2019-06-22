SELECT resource_param.id, param.name, unit
FROM resource_param
JOIN param ON param.id = resource_param.param_id
JOIN quantity ON quantity.id = param.qty_id
WHERE res_id = (SELECT resource_param.res_id FROM resource_location
    JOIN resource_param ON resource_param.id = resource_location.res_param_id
    WHERE resource_location.id = ?)
AND is_movable = 1