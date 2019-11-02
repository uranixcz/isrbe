SELECT param.id, param.name, param_val.val_float, param_val.val_text, resource.name, IFNULL(quantity.unit, q.unit), resource_param.is_movable, resource_param.id
FROM resource_param
JOIN param ON param.id = resource_param.param_id
LEFT JOIN param_val ON param_val.res_param_id = resource_param.id
LEFT JOIN quantity ON (quantity.id = param.qty_id AND param.type != 3)

LEFT JOIN resource_param rp ON rp.id = param_val.val_res
LEFT JOIN resource ON resource.id = rp.res_id
LEFT JOIN param p ON (p.id = rp.param_id AND param.type = 3)
LEFT JOIN quantity q ON (q.id = p.qty_id AND param.type = 3)

WHERE resource_param.res_id = ?