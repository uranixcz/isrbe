SELECT param.id, param.name, param_type.name, qty_id FROM param
JOIN param_type ON param_type.id = param.type